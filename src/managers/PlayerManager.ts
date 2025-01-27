import { event } from '@tauri-apps/api';
import { invoke } from '@tauri-apps/api/core';
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

export default class PlayerManager {
	private static instance: PlayerManager;
	private _playlist: PlayList = { count: 0, data: [] };
	private _currentSong: PlayListItem = PLACEHOLDER_SONG;
	private _mode: 'list' | 'single' | 'random' = 'list';
	private _player: Howl | null = null;
	private isChangingSong: boolean = false;
	private _playing: boolean = false;
	private _volume: number = DEFAULT_VOLUME;
	private _lyric: Lyric = {
		code: 200,
		lrc: {
			lyric: '[00:00.00]暂无歌词',
			version: 0,
		} as LyricContent,
	} as Lyric;

	constructor() {
		this.init();
	}

	public init() {
		const playerStore = usePlayerStore.getState();
		const settingStore = useSettingStore.getState();

		this._playlist = playerStore.playlist;
		this._currentSong = playerStore.currentSong;
		this._mode = playerStore.mode;
		this._volume = playerStore.volume;

		if (this._currentSong && this._playlist.data.length) {
			this.setCurrentSong(this._currentSong.id, settingStore.autoPlay, true).then(() => {
				if (settingStore.savePlaySeek) this._player?.seek(playerStore.seek);
			});
		}
	}

	public async setCurrentSong(id: number, play: boolean = true, init: boolean = false) {
		if (this.isChangingSong) return;
		this.isChangingSong = true;

		const target = this._playlist.data.find((item) => item.id === id);
		if (!target) {
			this.isChangingSong = false;
			return;
		}

		if (this._player) {
			const seek = this._player.seek();
			if (useSettingStore.getState().scrobble)
				scrobble(this._currentSong.id, this._currentSong.source, seek);
			this._player.stop();
			this._player.unload();
		}

		try {
			const url = await getSongURL(target.id);
			const data = url?.data[0];
			if (!data?.url) {
				alert('歌曲无法播放');
				return;
			}

			this._player = new Howl({
				src: [data.url],
				html5: true,
				format: ['mp3', 'wav', 'ogg'],
				volume: this._volume,
				mute: usePlayerStore.getState().muted,
				autoplay: play,
				loop: this._mode === 'single',
				onend: () => this.next(),
				onpause: () => event.emit('player-update-playing', false),
				onplay: () => event.emit('player-update-playing', true),
				onplayerror: (error) => {
					console.error('🎵 Error playing audio:', error);
					alert('歌曲无法播放');
				},
				xhr: {
					withCredentials: true,
					headers: {
						referer: 'https://music.163.com/',
					},
				},
			});

			if (init) this._player.seek(usePlayerStore.getState().seek);
			this._lyric = await getLyric(data.id).then(
				(res) =>
					res || {
						code: 200,
						lrc: {
							lyric: '[00:00.00]暂无歌词',
							version: 0,
						} as LyricContent,
					}
			);
			this._currentSong = target;
			this._playing = play;
			document.title = `${this._currentSong.name} - ${this._currentSong.artists?.join('/')}`;
			event.emit('player-update-current-song', this._currentSong);
			usePlayerStore.setState({ currentSong: target });
			if (useSettingStore.getState().pushToSMTC) this.pushToSTMC();
		} catch (error) {
			console.error('🎵 Error setting current song:', error);
		} finally {
			this.isChangingSong = false;
		}
	}

	public addToPlaylist(song: PlayListItem) {
		debounce(() => {
			if (this._playlist.data.find((item) => item.id === song.id)) return;
			this._playlist.data.push(song);
			this._playlist.count++;
			this.resetPlaylistIndices(); // 添加歌曲后重设index
			usePlayerStore.setState({ playlist: this._playlist });
		}, 300)();
	}

	public removeFromPlaylist(id: number) {
		if (this._playlist.count < 1) return;
		if (this._playlist.count === 1) return this.clearPlaylist();
		const index = this._playlist.data.findIndex((item) => item.id === id);
		// 如果当前播放的歌曲被移除，则播放下一首
		if (this._currentSong.id === id) {
			this._player?.unload();
			document.title = 'QTMusic';
			this.next();
		}
		this._playlist.data.splice(index, 1);
		this._playlist.count--;
		this.resetPlaylistIndices(); // 删除歌曲后重设index
		usePlayerStore.setState({ playlist: this._playlist });
	}

	public resetPlaylistIndices() {
		this._playlist.data.forEach((song, index) => {
			song.index = index;
		});
	}

	public clearPlaylist() {
		this._playlist = { count: 0, data: [] };
		usePlayerStore.setState({ playlist: this._playlist });
		this._currentSong = PLACEHOLDER_SONG;
		usePlayerStore.setState({ currentSong: PLACEHOLDER_SONG });
		document.title = 'QTMusic';

		// 清空播放器
		if (this._player) {
			this._player.pause();
			this._player.unload();
		}
	}

	public play() {
		if (!this._player || this._currentSong.index === -1 || this._player.playing()) return;
		this._player.play();
		this._playing = true;
		event.emit('player-play');
	}

	public pause() {
		if (!this._player || this._currentSong.index === -1 || !this._player.playing()) return;
		this._playing = false;
		this._player.pause();
		event.emit('player-pause');
	}

	public next(force = false) {
		usePlayerStore.setState({ seek: 0 });
		if (force) {
			if (this._playlist.count < 2) {
				this._player?.seek(0);
			}
			if (!this._currentSong) return this.setCurrentSong(this._playlist.data[0].id);
			// eslint-disable-next-line no-case-declarations
			const index = (this._currentSong?.index as number) + 1;
			if (index >= this._playlist.count)
				return this.setCurrentSong(this._playlist.data[0].id);
			this.setCurrentSong(this._playlist.data[index].id);
			return;
		}
		switch (this._mode) {
			case 'single':
				this._player?.seek(0);
				if (!this._player?.playing) this._player?.play();
				break;
			case 'list':
				if (this._playlist.count < 2) {
					this._player?.seek(0);
				}
				if (!this._currentSong) return this.setCurrentSong(this._playlist.data[0].id);
				// eslint-disable-next-line no-case-declarations
				const index = (this._currentSong?.index as number) + 1;
				if (index >= this._playlist.count)
					return this.setCurrentSong(this._playlist.data[0].id);
				this.setCurrentSong(this._playlist.data[index].id);
				break;
			case 'random':
				if (this._playlist.count < 2) {
					this._player?.seek(0);
				}
				if (!this._currentSong) return this.setCurrentSong(this._playlist.data[0].id);
				// eslint-disable-next-line no-case-declarations
				let random;
				do {
					random = Math.floor(Math.random() * this._playlist.count);
				} while (random === this._currentSong.index);
				this.setCurrentSong(this._playlist.data[random].id);
				break;
		}
	}

	public prev() {
		usePlayerStore.setState({ seek: 0 });
		switch (this._mode) {
			case 'single':
				this._player?.seek(0);
				if (!this._player?.playing) this._player?.play();
				break;
			case 'list':
				if (this._playlist.count < 2) {
					this._player?.seek(0);
				}
				if (!this._currentSong) return this.setCurrentSong(this._playlist.data[0].id);
				// eslint-disable-next-line no-case-declarations
				const index = (this._currentSong?.index as number) - 1;
				if (index < 0)
					return this.setCurrentSong(this._playlist.data[this._playlist.count - 1].id);
				this.setCurrentSong(this._playlist.data[index].id);
				break;
			case 'random':
				if (this._playlist.count < 2) {
					this._player?.seek(0);
				}
				if (!this._currentSong) return this.setCurrentSong(this._playlist.data[0].id);
				// eslint-disable-next-line no-case-declarations
				let random;
				do {
					random = Math.floor(Math.random() * this._playlist.count);
				} while (random === this._currentSong.index);
				this.setCurrentSong(this._playlist.data[random].id);
				break;
		}
	}

	get playlist() {
		return this._playlist;
	}
	get currentSong(): PlayListItem {
		if (this._currentSong) return this._currentSong;
		return PLACEHOLDER_SONG;
	}
	get mode() {
		return this._mode;
	}
	get player() {
		return this._player;
	}
	// 歌词列表
	get lyric(): Lyric {
		return (
			this._lyric ||
			({
				code: 200,
				lrc: {
					lyric: '[00:00.00]暂无歌词',
					version: 0,
				} as LyricContent,
			} as Lyric)
		);
	}
	public currentLyric(type: 'raw' | 'translate' = 'raw') {
		if (!this._lyric) return '';
		let lyricLines;
		if (type === 'raw') {
			lyricLines = this._lyric.lrc.lyric.split('\n');
		} else if (type === 'translate') {
			if (!this._lyric.tlyric) return '';
			lyricLines = this._lyric.tlyric.lyric.split('\n');
		} else {
			return '';
		}

		const seekTime = this._player?.seek() || 0;
		const lyricMap = this.parseLyric(lyricLines);

		// 查找当前歌词
		let currentLyric = '';
		for (const [time, lyric] of lyricMap) {
			if (time > seekTime) break;
			currentLyric = lyric;
		}

		return currentLyric;
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
		return this._playing || false;
	}
	get volume() {
		return this._volume || 0.5;
	}

	set mode(mode: 'list' | 'single' | 'random') {
		this._mode = mode;
		usePlayerStore.setState({ mode: mode });
	}
	set seek(seek: number) {
		if (!this._player) return;
		this._player.seek(seek);
	}
	set muted(muted: boolean) {
		if (!this._player) return;
		this._player.mute(muted);
		usePlayerStore.setState({ muted: muted });
	}
	set volume(volume: number) {
		this._volume = volume;
		if (!this._player) return;
		this._player.volume(volume);
		usePlayerStore.setState({ volume: volume });
	}

	public pushToSTMC() {
		if (!this._player || !useSettingStore.getState().pushToSMTC) return;
		invoke('push_to_stmc', {
			title: this._currentSong?.name || 'None',
			artist: this._currentSong?.artists?.join('/') || 'None',
			cover:
				this._currentSong?.cover ||
				'https://cdn.discordapp.com/attachments/929847977705945610/929848029813848478/unknown.png',
		});
	}

	public static getInstance(): PlayerManager {
		if (!PlayerManager.instance) {
			PlayerManager.instance = new PlayerManager();
		}
		return PlayerManager.instance;
	}

	// 解析歌词
	public parseLyric(lyricLines: string[]): Map<number, string> {
		const lyricMap = new Map<number, string>();
		const timeRegex = /\[(\d{2}):(\d{2})\.(\d{2,3})\]/;
		for (const line of lyricLines) {
			const match = line.match(timeRegex);
			if (match) {
				const minutes = parseInt(match[1], 10);
				const seconds = parseInt(match[2], 10);
				const milliseconds = parseInt(match[3], 10);
				const time =
					minutes * 60 + seconds + milliseconds / (match[3].length === 3 ? 1000 : 100);
				const lyricText = line.replace(timeRegex, '').trim();
				if (lyricText) {
					lyricMap.set(time, lyricText);
				}
			}
		}
		return lyricMap;
	}
}
