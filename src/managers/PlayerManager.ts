import { event } from '@tauri-apps/api';
import { Howl } from 'howler';
import { debounce } from 'lodash-es';
import { getLyric, getSongURL } from '../apis/song';
import { scrobble } from '../apis/user';
import { Lyric, LyricContent, PlayList, PlayListItem } from '../models/song';
import { usePlayerStore } from '../store/player';
import { useSettingStore } from '../store/setting';

const DEFAULT_VOLUME = 0.5;
const PLACEHOLDER_SONG: PlayListItem = {
	index: -1,
	id: 0,
	source: 0,
	name: '暂无歌曲',
};

interface PlaybackStrategy {
	getNextIndex(currentIndex: number, playlistLength: number): number;
	getPrevIndex(currentIndex: number, playlistLength: number): number;
}

class ListPlaybackStrategy implements PlaybackStrategy {
	getNextIndex(currentIndex: number, playlistLength: number): number {
		return currentIndex >= playlistLength - 1 ? 0 : currentIndex + 1;
	}

	getPrevIndex(currentIndex: number, playlistLength: number): number {
		return currentIndex <= 0 ? playlistLength - 1 : currentIndex - 1;
	}
}

class RandomPlaybackStrategy implements PlaybackStrategy {
	getNextIndex(currentIndex: number, playlistLength: number): number {
		return Math.floor(Math.random() * playlistLength);
	}

	getPrevIndex(currentIndex: number, playlistLength: number): number {
		return Math.floor(Math.random() * playlistLength);
	}
}

export default class PlayerManager {
	private static instance: PlayerManager;
	private _playlist: PlayList = { count: 0, data: [] };
	private _currentSong: PlayListItem = PLACEHOLDER_SONG;
	private _mode: 'list' | 'single' | 'random' = 'list';
	private _player: Howl | null = null;
	private isChangingSong: boolean = false;
	private isChangingPlayState: boolean = false;
	private _playing: boolean = false;
	private _volume: number = DEFAULT_VOLUME;
	private _lyric: Lyric = this.createDefaultLyric();
	private playbackStrategies: Record<string, PlaybackStrategy> = {
		list: new ListPlaybackStrategy(),
		random: new RandomPlaybackStrategy(),
	};

	private createDefaultLyric(): Lyric {
		return {
			code: 200,
			lrc: {
				lyric: '',
				version: 0,
			} as LyricContent,
		};
	}

	constructor() {
		this.initializeFromStore();
	}

	private initializeFromStore() {
		const playerStore = usePlayerStore.getState();
		const settingStore = useSettingStore.getState();

		this._playlist = playerStore.playlist;
		this._currentSong = playerStore.currentSong;
		this._mode = playerStore.mode;
		this._volume = playerStore.volume;

		if (this._currentSong?.id && this._playlist.data.length) {
			this.setCurrentSong(this._currentSong.id, settingStore.autoPlay, true).then(() => {
				if (settingStore.savePlaySeek) {
					this._player?.seek(playerStore.seek);
				}
			});
		}
	}

	/**
	 * Sets the current song to play
	 * @param id - ID of the song to play
	 * @param play - Whether to start playing immediately
	 * @param init - Initialization flag for restoring playback state
	 */
	public async setCurrentSong(id: number, play: boolean = true, init: boolean = false) {
		if (this.isChangingSong) return;
		this.isChangingSong = true;

		try {
			const target = this.findSongInPlaylist(id);
			if (!target) return;

			await this.handleCurrentSongChange();
			const urlData = await this.fetchSongUrl(target.id);

			this.initializePlayer(urlData.url, play, init);
			await this.handleLyricsFetch(target.id);

			this.updatePlayerState(target, play);
			this.updateDocumentTitle();
		} catch (error) {
			console.error('Error setting current song:', error);
		} finally {
			this.isChangingSong = false;
		}
	}

	private findSongInPlaylist(id: number): PlayListItem | undefined {
		return this._playlist.data.find((item) => item.id === id);
	}

	private async handleCurrentSongChange() {
		if (this._player) {
			const seek = this._player.seek();
			if (useSettingStore.getState().scrobble) {
				scrobble(this._currentSong.id, this._currentSong.source, seek);
			}
			this._player.stop();
			this._player.unload();
		}
	}

	private async fetchSongUrl(songId: number) {
		const urlResponse = await getSongURL(songId);
		const urlData = urlResponse?.data[0];
		if (!urlData?.url) throw new Error('Song URL not available');
		return urlData;
	}

	private initializePlayer(url: string, play: boolean, init: boolean) {
		this._player = new Howl({
			src: [url],
			html5: true,
			format: ['mp3', 'wav', 'ogg'],
			volume: this._volume,
			mute: usePlayerStore.getState().muted,
			autoplay: play,
			onend: () => this.next(),
			onpause: () => this.emitPlayerUpdate('pause'),
			onplay: () => this.handlePlayStart(),
			onplayerror: () => this.handlePlayError(),
			onstop: () => this.clearMediaSession(),
			preload: 'metadata',
			pool: 1,
			xhr: this.createXhrConfig(),
		});

		if (init) this._player.seek(usePlayerStore.getState().seek);
	}

	private createXhrConfig() {
		return {
			withCredentials: true,
			headers: {
				Referer: 'https://music.163.com/',
				Origin: 'https://music.163.com',
			},
		};
	}

	private handlePlayStart() {
		if ('mediaSession' in navigator && useSettingStore.getState().pushToSMTC) {
			this.updateMediaSession();
		}
		this.emitPlayerUpdate('play');
	}

	private updateMediaSession() {
		navigator.mediaSession.metadata = new MediaMetadata({
			title: this.currentSong.name,
			artist: this._currentSong.artists?.join('/'),
			artwork: [
				{
					src: this._currentSong.cover as string,
					sizes: '1600x1600',
					type: 'image/jpeg',
				},
			],
		});
	}

	private handlePlayError() {
		console.error('Error playing audio');
		alert('歌曲无法播放');
	}

	private clearMediaSession() {
		if ('mediaSession' in navigator) {
			navigator.mediaSession.metadata = null;
		}
	}

	private async handleLyricsFetch(songId: number) {
		this._lyric = (await getLyric(songId)) || this.createDefaultLyric();
	}

	private updatePlayerState(song: PlayListItem, playState: boolean) {
		this._currentSong = song;
		this._playing = playState;
		usePlayerStore.setState({ currentSong: song });
		this.emitPlayerUpdate('song-change');
	}

	private updateDocumentTitle() {
		document.title = `${this._currentSong.name} - ${this._currentSong.artists?.join('/')}`;
	}

	/**
	 * Adds a song to the playlist
	 * @param song - Song item to add
	 */
	public addToPlaylist(song: PlayListItem) {
		debounce(() => {
			if (this._playlist.data.some((item) => item.id === song.id)) return;

			this._playlist.data.push(song);
			this._playlist.count++;
			this.resetPlaylistIndices();
			usePlayerStore.setState({ playlist: this._playlist });
		}, 300)();
	}

	/**
	 * Removes a song from the playlist
	 * @param id - ID of the song to remove
	 */
	public removeFromPlaylist(id: number) {
		if (this._playlist.count < 1) return;

		const index = this._playlist.data.findIndex((item) => item.id === id);
		if (index === -1) return;

		if (this._currentSong.id === id) {
			this.handleCurrentSongRemoval();
		}

		this._playlist.data.splice(index, 1);
		this._playlist.count--;
		this.resetPlaylistIndices();
		usePlayerStore.setState({ playlist: this._playlist });
	}

	private handleCurrentSongRemoval() {
		this._player?.unload();
		document.title = 'AzusaP';
		this.next();
	}

	/**
	 * Clears the entire playlist
	 */
	public clearPlaylist() {
		this._playlist = { count: 0, data: [] };
		this._currentSong = PLACEHOLDER_SONG;
		document.title = 'AzusaP';

		usePlayerStore.setState({
			playlist: this._playlist,
			currentSong: PLACEHOLDER_SONG,
		});

		if (this._player) {
			this._player.pause();
			this._player.unload();
		}
	}

	/**
	 * Starts or resumes playback
	 */
	public async play() {
		if (!this.isPlayActionValid()) return;
		this.isChangingPlayState = true;

		this._player?.volume(0);
		this._player?.play();
		this._playing = true;

		await this.handlePlayTransition();
		this.isChangingPlayState = false;
	}

	private isPlayActionValid(): boolean {
		return (
			!!this._player &&
			!this.isChangingPlayState &&
			this._currentSong.index !== -1 &&
			!this._player.playing()
		);
	}

	private async handlePlayTransition() {
		return new Promise<void>((resolve) => {
			this._player?.once('play', () => {
				this._player?.fade(0, this._volume, useSettingStore.getState().fadeTime);
				resolve();
			});
		});
	}

	/**
	 * Pauses playback
	 */
	public async pause() {
		if (!this.isPauseActionValid()) return;
		this.isChangingPlayState = true;

		await this.handlePauseTransition();
		this.isChangingPlayState = false;
	}

	private isPauseActionValid(): boolean {
		return (
			!!this._player &&
			this._currentSong.index !== -1 &&
			!!this._player.playing() &&
			!this.isChangingPlayState
		);
	}

	private async handlePauseTransition() {
		return new Promise<void>((resolve) => {
			if (!this._player) return resolve();

			this._player.fade(this._volume, 0, useSettingStore.getState().fadeTime);
			this._player.once('fade', () => {
				this._player?.pause();
				this._player?.volume(this._volume);
				this._playing = false;
				resolve();
			});
		});
	}

	/**
	 * Skips to the next track
	 * @param force - Force skip regardless of playback mode
	 */
	public next(force = false) {
		usePlayerStore.setState({ seek: 0 });

		if (this._playlist.count < 1 || this._mode === 'single') {
			this._player?.seek(0);
			return;
		}

		const strategy = force ? new ListPlaybackStrategy() : this.playbackStrategies[this._mode];

		const newIndex = strategy.getNextIndex(this._currentSong?.index ?? 0, this._playlist.count);

		this.setCurrentSong(this._playlist.data[newIndex].id);
	}

	/**
	 * Returns to the previous track
	 */
	public prev() {
		usePlayerStore.setState({ seek: 0 });

		if (this._playlist.count < 1 || this._mode === 'single') {
			this._player?.seek(0);
			return;
		}

		const strategy = this.playbackStrategies[this._mode];

		const newIndex = strategy.getPrevIndex(this._currentSong?.index ?? 0, this._playlist.count);

		this.setCurrentSong(this._playlist.data[newIndex].id);
	}

	public resetPlaylistIndices() {
		this._playlist.data.forEach((song, index) => {
			song.index = index;
		});
	}

	private emitPlayerUpdate(eventType: 'play' | 'pause' | 'song-change') {
		const eventMap = {
			play: () => event.emit('player-update-playing', true),
			pause: () => event.emit('player-update-playing', false),
			'song-change': () => event.emit('player-update-current-song', this._currentSong),
		};

		eventMap[eventType]();
	}

	// Getters and setters
	get playlist() {
		return this._playlist;
	}
	get currentSong() {
		return this._currentSong || PLACEHOLDER_SONG;
	}
	get mode() {
		return this._mode;
	}
	get player() {
		return this._player;
	}
	get lyric() {
		return this._lyric || this.createDefaultLyric();
	}
	get muted() {
		return this._player?.mute() || false;
	}
	get duration() {
		return this._player?.duration() || 0;
	}
	get seek() {
		return this._player?.seek() || 0;
	}
	get playing() {
		return this._playing;
	}
	get volume() {
		return this._volume;
	}

	set mode(mode: 'list' | 'single' | 'random') {
		this._mode = mode;
		usePlayerStore.setState({ mode });
	}

	set seek(seek: number) {
		if (!this._player || this._player.state() !== 'loaded') return;
		const clampedSeek = Math.max(0, Math.min(seek, this.duration));
		this._player.seek(clampedSeek);
		usePlayerStore.setState({ seek: clampedSeek });
	}

	set muted(muted: boolean) {
		this._player?.mute(muted);
		usePlayerStore.setState({ muted });
	}

	set volume(volume: number) {
		this._volume = volume;
		this._player?.volume(volume);
		usePlayerStore.setState({ volume });
	}

	public static getInstance(): PlayerManager {
		if (!PlayerManager.instance) {
			PlayerManager.instance = new PlayerManager();
		}
		return PlayerManager.instance;
	}

	/**
	 * Parses lyric data into timestamped map
	 * @param lyricLines - Array of lyric strings
	 * @returns Map of timestamps to lyric text
	 */
	public parseLyric(lyricLines: string[]): Map<number, string> {
		const lyricMap = new Map<number, string>();
		const timeRegex = /\[(\d{2}):(\d{2})\.(\d+)\]/;

		for (const line of lyricLines) {
			const match = line.match(timeRegex);
			if (match) {
				const time = this.calculateLyricTime(match);
				const text = line.replace(timeRegex, '').trim();
				if (text) lyricMap.set(time, text);
			}
		}

		return lyricMap;
	}

	private calculateLyricTime(match: RegExpMatchArray): number {
		const minutes = parseInt(match[1], 10);
		const seconds = parseInt(match[2], 10);
		const milliseconds = parseInt(match[3], 10);
		const precisionFactor = match[3].length === 3 ? 1000 : 100;
		return minutes * 60 + seconds + milliseconds / precisionFactor;
	}

	/**
	 * Gets current lyric based on playback position
	 * @param type - Lyric type (original or translated)
	 */
	public currentLyric(type: 'raw' | 'translate' = 'raw'): string {
		const lyricContent = this.getLyricContent(type);
		if (!lyricContent) return '';

		const seekTime = this._player?.seek() || 0;
		return this.findCurrentLyricText(lyricContent.split('\n'), seekTime);
	}

	private getLyricContent(type: 'raw' | 'translate'): string {
		if (type === 'translate' && this._lyric.tlyric?.lyric) {
			return this._lyric.tlyric.lyric;
		}
		return this._lyric.lrc?.lyric || '';
	}

	private findCurrentLyricText(lines: string[], seekTime: number): string {
		const lyricMap = this.parseLyric(lines);
		let currentLyric = '';

		for (const [time, lyric] of lyricMap) {
			if (time > seekTime) break;
			currentLyric = lyric;
		}

		return currentLyric;
	}
}
