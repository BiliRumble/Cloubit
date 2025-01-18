import { event } from '@tauri-apps/api';
import { Howl } from 'howler';
import { getSongURL } from '../api/song';
import { PlayList, PlayListItem } from '../models/main';
import { usePlayerStore } from '../store/player';
import { useSettingStore } from '../store/setting';
import { invoke } from 'lodash-es';

export default class PlayerManager {
	private static instance: PlayerManager;
	private playerStore = usePlayerStore();
	private _playlist: PlayList = { count: 0, data: [] };
	private _currentSong: PlayListItem | null = null;
	private _mode: 'list' | 'single' | 'random' = 'list';
	private _player: Howl | null = new Howl({
		src: [''],
		format: ['mp3', 'wav', 'ogg'],
		volume: this.playerStore.volume,
		mute: this.playerStore.muted,
		autoplay: false,
		loop: this._mode === 'single',
	});
	private isChangingSong: boolean = false;
	private isFadingOut: boolean = false;
	private _playing: boolean = false;

	constructor() {
		console.debug('🎵 Player Manager Init');
		this._playlist = this.playerStore.playlist;
		this._currentSong = this.playerStore.currentSong;
		this._mode = this.playerStore.mode;
	}

	public init() {
		if (!this._currentSong || !this._playlist.data.length) return;
		this.setCurrentSong(this._currentSong.id, useSettingStore().autoPlay, true);
		console.debug('🎵 Player Manager Init Complete');
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
			if (!data) {
				this.isChangingSong = false;
				return;
			}

			this._player = new Howl({
				src: [data.url],
				format: ['mp3', 'wav', 'ogg'],
				volume: this.playerStore.volume,
				mute: this.playerStore.muted,
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
			if (init) this._player?.seek(this.playerStore.seek);
			console.debug('🎵 Set Current Song:', this._currentSong);
		} catch (error) {
			console.error('Error setting current song:', error);
		} finally {
			this.isChangingSong = false;
		}
	}

	public addToPlaylist(song: PlayListItem) {
		if (this._playlist.data.find((item) => item.id === song.id)) return;
		this._playlist.data.push(song);
		this._playlist.count++;
		this.playerStore.playlist = this._playlist;
		console.log(this._playlist);
	}

	public removeFromPlaylist(id: number) {
		const index = this._playlist.data.findIndex((item) => item.id === id);
		if (index === -1) return;
		this._playlist.data.splice(index, 1);
		this._playlist.count--;
		this.playerStore.playlist = this._playlist;
	}

	public play() {
		if (!this._player || this._playing || this.isFadingOut) return;
		this.isFadingOut = true;
		this._player.play();
		this._playing = true;
		if (useSettingStore.getState().fadeInOut) {
			this._player.fade(0, this.playerStore.volume, useSettingStore.getState().fadeTime);
			setTimeout(() => {
				this.isFadingOut = false;
			}, useSettingStore.getState().fadeTime);
		}
		event.emit('player-play');
	}

	public pause() {
		if (!this._player || !this._playing || this.isFadingOut) return;
		this.isFadingOut = true;
		if (useSettingStore.getState().fadeInOut)
			this._player.fade(this.playerStore.volume, 0, useSettingStore.getState().fadeTime);
		this._playing = false;
		setTimeout(() => {
			if (!this._player?.playing) return;
			this._player.pause();
			this.isFadingOut = false;
		}, useSettingStore.getState().fadeTime);
		event.emit('player-pause');
	}

	public next() {
		usePlayerStore.setState({ seek: 0 });
		switch (this._mode) {
			case 'single':
				if (!this._player?.playing) this._player?.play();
				break;
			case 'list':
				if (this._playlist.count < 1) break;
				if (this._playlist.count === 1) return this._player?.play();
				if (!this._currentSong) return this.setCurrentSong(this._playlist.data[0].id);
				// eslint-disable-next-line no-case-declarations
				const index = (this._currentSong?.index as number) + 1;
				if (index >= this._playlist.count)
					return this.setCurrentSong(this._playlist.data[0].id);
				this.setCurrentSong(this._playlist.data[index].id);
				break;
			case 'random':
				if (this._playlist.count < 1) break;
				if (this._playlist.count === 1) return this._player?.play();
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
				if (!this._player?.playing) this._player?.play();
				break;
			case 'list':
				if (this._playlist.count < 1) break;
				if (this._playlist.count === 1) return this._player?.play();
				if (!this._currentSong) return this.setCurrentSong(this._playlist.data[0].id);
				// eslint-disable-next-line no-case-declarations
				const index = (this._currentSong?.index as number) - 1;
				if (index < 0)
					return this.setCurrentSong(this._playlist.data[this._playlist.count - 1].id);
				this.setCurrentSong(this._playlist.data[index].id);
				break;
			case 'random':
				if (this._playlist.count < 1) break;
				if (this._playlist.count === 1) return this._player?.play();
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

	get volume() {
		return this._player?.volume() || 0.5;
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

	set mode(mode: 'list' | 'single' | 'random') {
		this._mode = mode;
		this.playerStore.mode = mode;
		usePlayerStore.setState({ mode: mode });
	}
	set seek(seek: number) {
		if (!this._player) return;
		this._player.seek(seek);
	}
	set volume(volume: number) {
		if (!this._player) return;
		this._player.volume(volume);
		this.playerStore.volume = volume;
	}
	set muted(muted: boolean) {
		if (!this._player) return;
		this._player.mute(muted);
		this.playerStore.muted = muted;
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
