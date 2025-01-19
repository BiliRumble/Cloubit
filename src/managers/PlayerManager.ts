import { event } from '@tauri-apps/api';
import { invoke } from '@tauri-apps/api/core';
import { Howl } from 'howler';
import { getSongURL } from '../apis/song';
import { PlayList, PlayListItem } from '../models/main';
import { usePlayerStore } from '../store/player';
import { useSettingStore } from '../store/setting';

export default class PlayerManager {
	// 占位符currentSong
	private _placeholderSong: PlayListItem = {
		index: -1,
		id: 0,
		name: '暂无歌曲',
	};

	private static instance: PlayerManager;
	private _playlist: PlayList = { count: 0, data: [] };
	private _currentSong: PlayListItem = this._placeholderSong;
	private _mode: 'list' | 'single' | 'random' = 'list';
	private _player: Howl | null = new Howl({
		src: [''],
		format: ['mp3', 'wav', 'ogg'],
		volume: usePlayerStore.getState().volume,
		mute: usePlayerStore.getState().muted,
		autoplay: false,
		loop: this._mode === 'single',
	});
	private isChangingSong: boolean = false;
	private _playing: boolean = false;
	private _volume: number = 0.5;

	constructor() {
		console.debug('🎵 Player Manager Init');
		this._playlist = usePlayerStore.getState().playlist;
		this._currentSong = usePlayerStore.getState().currentSong;
		this._mode = usePlayerStore.getState().mode;
		this._volume = usePlayerStore.getState().volume;
		this.init();
	}

	public init() {
		if (!this._currentSong || !this._playlist.data.length) return;
		this.setCurrentSong(this._currentSong.id, useSettingStore.getState().autoPlay, true).then(
			() => this._player?.seek(usePlayerStore.getState().seek)
		);
	}

	public async setCurrentSong(id: number, play: boolean = true, init: boolean = false) {
		if (!init) usePlayerStore.setState({ seek: 0 });
		if (this.isChangingSong) return;
		this.isChangingSong = true;

		const target = this._playlist.data.find((item) => item.id === id);
		if (!target) {
			this.isChangingSong = false;
			return;
		}

		if (this._player) {
			this._player.pause();
			this._player.unload();
		}

		try {
			const url = await getSongURL(target.id);
			const data = url?.data[0];
			if (!data?.url) {
				this.isChangingSong = false;
				this.next();
				return;
			}

			this._player = new Howl({
				src: [data.url],
				format: ['mp3', 'wav', 'ogg'],
				volume: this._volume,
				mute: usePlayerStore.getState().muted,
				autoplay: play,
				loop: this._mode === 'single',
				onend: () => {
					this.next();
				},
				onpause: () => {
					event.emit('player-update-playing', false);
				},
				onplay: () => {
					event.emit('player-update-playing', false);
				},
			});

			this._currentSong = target;
			this._playing = play;
			usePlayerStore.setState({ currentSong: target });
			if (init) this._player?.seek(usePlayerStore.getState().seek);
		} catch (error) {
			console.error('🎵 Error setting current song:', error);
		} finally {
			this.isChangingSong = false;
		}
	}

	public addToPlaylist(song: PlayListItem) {
		if (this._playlist.data.find((item) => item.id === song.id)) return;
		this._playlist.data.push(song);
		this._playlist.count++;
		usePlayerStore.setState({ playlist: this._playlist });
		console.log(this._playlist);
	}

	public removeFromPlaylist(id: number) {
		if (this._playlist.count < 1) return;
		if (this._playlist.count === 1) return this.clearPlaylist();
		const index = this._playlist.data.findIndex((item) => item.id === id);
		// 如果当前播放的歌曲被移除，则播放下一首
		if (this._currentSong.id === id) {
			this._player?.unload();
			this.next();
		}
		this._playlist.data.splice(index, 1);
		this._playlist.count--;
		usePlayerStore.setState({ playlist: this._playlist });
	}

	public clearPlaylist() {
		this._playlist = { count: 0, data: [] };
		usePlayerStore.setState({ playlist: this._playlist });
		this._currentSong = this._placeholderSong;
		usePlayerStore.setState({ currentSong: this._placeholderSong });

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
		return {
			index: -1,
			id: -1,
			name: 'QTMusic',
			cover: 'https://cdn.discordapp.com/attachments/929847977705945610/929848029813848478/unknown.png',
		};
	}
	get mode() {
		return this._mode;
	}
	get player() {
		return this._player;
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
			name: this._currentSong?.name || 'None',
			artist: this._currentSong?.artists || 'None',
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
}
