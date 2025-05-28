import { Howl } from 'howler';
import { debounce } from 'lodash-es';
import { getLyric, getSongURL } from '../apis/song';
import { scrobble } from '../apis/user';
import { Lyric, LyricContent, PlayList, PlayListItem } from '../models/song';
import { usePlayerStore } from '../store/player';
import { useSettingStore } from '../store/setting';
import { eventBus } from '../utils/EventBus';

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

		this.subscribeEvents();

		if (this._currentSong?.id && this._playlist.data.length) {
			this.setCurrentSong(this._currentSong.id, settingStore.autoPlay, true).then(() => {
				if (settingStore.savePlaySeek) {
					this._player?.seek(playerStore.seek);
				}
			});
		}
	}

	private subscribeEvents() {
		eventBus.on('playerSetState', (playing?: boolean) => {
			debounce(() => {
				if (playing && playing !== this._playing) {
					if (playing) this.play();
					else this.pause();
					return;
				}
				return this._playing ? this.pause() : this.play();
			}, 300)();
		});

		eventBus.on('playerSetVolume', (volume?: number) => {
			if (volume !== undefined) {
				this.volume = volume;
				return;
			}
			this.muted = !this.muted;
		});

		eventBus.on('playerNext', () => this.next());
		eventBus.on('playerPrev', () => this.prev());

		eventBus.on('playerSetMode', (mode: 'list' | 'single' | 'random') => {
			this.mode = mode;
		});

		eventBus.on('systemPipReady', () => {
			eventBus.emit('playerCurrent', this._currentSong);
			eventBus.emit('playerState', this._playing);
			eventBus.emit('playerDuration', this.duration);
			eventBus.emit('playerLyric', this.currentLyric());
		});
	}

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
			this.updateMediaSession();

			eventBus.emit('playerCurrent', target);
			eventBus.emit('playerDuration', this.duration);
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
		this._player?.unload();
		this._player = null;

		this._player = new Howl({
			src: [url],
			html5: true,
			format: ['mp3', 'wav', 'ogg'],
			volume: this._volume,
			mute: usePlayerStore.getState().muted,
			autoplay: play,
			onend: () => this.next(),
			onpause: () => {
				eventBus.emit('playerState', false);
				this.updateMediaPlaybackState('paused');
			},
			onplay: () => {
				this.handlePlayStart();
				this.setupMediaSessionHandlers();
			},
			onplayerror: () => this.handlePlayError(),
			onstop: () => this.clearMediaSession(),
			preload: 'metadata',
			pool: 1,
			xhr: this.createXhrConfig(),
		});

		if (init) this._player.seek(usePlayerStore.getState().seek);

		eventBus.emit('playerInit', this._player);
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

	private setupMediaSessionHandlers() {
		if ('mediaSession' in navigator) {
			try {
				navigator.mediaSession.setActionHandler('play', () => this.play());
				navigator.mediaSession.setActionHandler('pause', () => this.pause());
				navigator.mediaSession.setActionHandler('previoustrack', () => this.prev());
				navigator.mediaSession.setActionHandler('nexttrack', () => this.next());

				navigator.mediaSession.setActionHandler('seekto', (details) => {
					if (details.seekTime !== undefined) {
						this.seek = details.seekTime;
					}
				});
			} catch (error) {
				console.warn('MediaSession API 部分操作不支持:', error);
			}
		}
	}

	private updateMediaSession() {
		if (!('mediaSession' in navigator)) return;

		const artwork = [];
		const coverUrl = this._currentSong.cover;
		if (coverUrl) {
			artwork.push({
				src: coverUrl,
				sizes: '512x512',
				type: 'image/jpeg',
			});
		}

		navigator.mediaSession.metadata = new MediaMetadata({
			title: this._currentSong.name || '未知曲目',
			artist: this._currentSong.artists?.join('/') || '未知艺术家',
			artwork: artwork,
		});

		this.updateMediaPlaybackState(this._playing ? 'playing' : 'paused');
	}

	private updateMediaPlaybackState(state: 'playing' | 'paused' | 'none') {
		if ('mediaSession' in navigator) {
			navigator.mediaSession.playbackState = state;
		}
	}

	private handlePlayStart() {
		this.updateMediaSession();
		eventBus.emit('playerState', true);
	}

	private handlePlayError() {
		console.error('Error playing audio');
		alert('歌曲无法播放');
	}

	private clearMediaSession() {
		if ('mediaSession' in navigator) {
			navigator.mediaSession.metadata = null;
			this.updateMediaPlaybackState('none');
		}
	}

	private async handleLyricsFetch(songId: number) {
		this._lyric = (await getLyric(songId)) || this.createDefaultLyric();
	}

	private updatePlayerState(song: PlayListItem, playState: boolean) {
		this._currentSong = song;
		this._playing = playState;
		usePlayerStore.setState({ currentSong: song });
	}

	private updateDocumentTitle() {
		document.title = `${this._currentSong.name} - ${this._currentSong.artists?.join('/')}`;
	}

	public addToPlaylist(song: PlayListItem) {
		debounce(() => {
			if (this._playlist.data.some((item) => item.id === song.id)) return;

			this._playlist.data.push(song);
			this._playlist.count++;
			this.resetPlaylistIndices();
			usePlayerStore.setState({ playlist: this._playlist });
		}, 300)();
	}

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
		document.title = 'Cloubit';
		this.next();
	}

	public clearPlaylist() {
		this._playlist = { count: 0, data: [] };
		this._currentSong = PLACEHOLDER_SONG;
		document.title = 'Cloubit';

		usePlayerStore.setState({
			playlist: this._playlist,
			currentSong: PLACEHOLDER_SONG,
		});

		if (this._player) {
			this._player.pause();
			this._player.unload();
		}

		if ('mediaSession' in navigator) {
			navigator.mediaSession.metadata = null;
			this.updateMediaPlaybackState('none');
		}
	}

	public async play() {
		if (!this.isPlayActionValid()) return;
		this.isChangingPlayState = true;

		this._player?.volume(0);
		this._player?.play();
		this._playing = true;

		await this.handlePlayTransition();
		this.isChangingPlayState = false;
		this.updateMediaPlaybackState('playing');
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

	public async pause() {
		if (!this.isPauseActionValid()) return;
		this.isChangingPlayState = true;

		await this.handlePauseTransition();
		this.isChangingPlayState = false;
		this.updateMediaPlaybackState('paused');
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
