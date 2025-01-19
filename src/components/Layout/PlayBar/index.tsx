import { event } from '@tauri-apps/api';
import { debounce } from 'lodash-es';
import { useEffect, useRef, useState } from 'react';
import { usePlayerManager } from '../../../context/PlayerContext';
import { usePlayerStore } from '../../../store/player';
import { useSettingStore } from '../../../store/setting';
import Modal from '../../Common/Modal';
import Popover from '../../Common/Popover';
import PlayList from './PlayerList';
import styles from './PlayBar.module.scss';

interface PlayBarProps {
	className?: string;
}

const PlayBar: React.FC<PlayBarProps> = ({ className }) => {
	const [playerListModalOpen, setPlayerListModalOpen] = useState(false);

	const usePlayer = usePlayerManager();
	const [currentSong, setCurrentSong] = useState(usePlayer.currentSong);
	const [mode, setMode] = useState(usePlayer.mode);
	const [playing, setPlaying] = useState(usePlayer.playing);
	const [muted, setMuted] = useState(usePlayer.muted);
	const [volume, setVolume] = useState(usePlayer.volume);
	const [seek, setSeek] = useState(usePlayer.seek);
	const [duration, setDuration] = useState(usePlayer.duration);
	const [playlist, setPlaylist] = useState(usePlayer.playlist);

	const volumeRef = useRef<HTMLSpanElement>(null);

	const [seekDragging, setSeekDragging] = useState(false);

	// 同步播放器状态
	useEffect(() => {
		const updateState = () => {
			setCurrentSong(usePlayer.currentSong);
			setMode(usePlayer.mode);
			setMuted(usePlayer.muted);
			setVolume(usePlayer.volume);
			if (!seekDragging) {
				setSeek(usePlayer.seek);
			}
			setDuration(usePlayer.duration);
			setPlaylist(usePlayer.playlist);
			setPlaying(usePlayer.playing);
		};

		updateState();

		const interval = setInterval(updateState, 250); // 每秒更新一次状态

		return () => clearInterval(interval);
	}, [usePlayer, seekDragging]);

	useEffect(() => {
		if (useSettingStore.getState().savePlaySeek) usePlayerStore.setState({ seek: seek });
	}, [seek]);

	useEffect(() => {
		const lazyAsync = async () => {
			await event.emit('player-update-duration', duration);
			await event.emit('player-update-current-song', currentSong);
		};
		const interval = setInterval(lazyAsync, 1500);
		const STMC = () => {
			if (useSettingStore.getState().pushToSMTC) usePlayer.pushToSTMC();
		};
		const interval2 = setInterval(STMC, 15000);
		return () => {
			clearInterval(interval);
			clearInterval(interval2);
		};
	}, [duration]);

	useEffect(() => {
		event.emit('player-update-current-song', currentSong);
		event.emit('player-update-duration', duration);
		if (useSettingStore.getState().pushToSMTC) usePlayer.pushToSTMC();
		if (useSettingStore.getState().autoPlay) event.emit('player-play');
	}, [currentSong]);

	useEffect(() => {
		const async = async () => {
			await event.emit('player-update-seek', seek);
		};
		setPlaying(usePlayer.playing);
		async();
	}, [seek]);

	useEffect(() => {
		// 注册事件
		const prev = event.listen('pip-prev', () => {
			debounce(() => {
				usePlayer.prev();
			}, 300)();
		});

		const next = event.listen('pip-next', () => {
			debounce(() => {
				usePlayer.next();
			}, 300)();
		});

		const play = event.listen('pip-play', () => {
			debounce(() => {
				if (playing) return usePlayer.pause();
				else return usePlayer.play();
			}, 300)();
		});

		event.emit('player-update-current-song', currentSong);
		event.emit('player-update-duration', duration);
		event.emit('player-update-playing', playing);

		return () => {
			// 取消事件监听
			prev.then((f) => f());
			next.then((f) => f());
			play.then((f) => f());
		};
	}, [playing]);

	// 注册快捷键事件
	useEffect(() => {
		const play = event.listen('shortcut-play', () => {
			debounce(() => {
				if (playing) return usePlayer.pause();
				else return usePlayer.play();
			}, 300)();
		});
		const prev = event.listen('shortcut-prev', () => {
			debounce(() => {
				usePlayer.prev();
			}, 300)();
		});
		const next = event.listen('shortcut-next', () => {
			debounce(() => {
				console.log('next');
				usePlayer.next();
			}, 300)();
		});
		const volumeUp = event.listen('shortcut-volume-up', () => {
			if (volume < 1) {
				usePlayer.volume = volume + 0.1;
			}
		});
		const volumeDown = event.listen('shortcut-volume-down', () => {
			if (volume > 0) {
				usePlayer.volume = volume - 0.1;
			}
		});
		return () => {
			play.then((f) => f());
			prev.then((f) => f());
			next.then((f) => f());
			volumeUp.then((f) => f());
			volumeDown.then((f) => f());
		};
	}, [playing, volume]);

	useEffect(() => {
		const pipRequest = event.listen('pip-request', () => {
			debounce(() => {
				event.emit('player-update-current-song', currentSong);
				event.emit('player-update-duration', duration);
			}, 300)();
		});
		return () => {
			pipRequest.then((f) => f());
		};
	}, []);

	return (
		<>
			<div className={`${className || ''} ${styles.playbar}`.trim()}>
				<div className={styles.playbar__left}>
					<div className={styles.info}>
						<img src={currentSong?.cover} alt={`${currentSong?.name}的封面`} />
						<div className={styles.info__text}>
							<h1 id="title">{currentSong?.name}</h1>
							<h2>
								{currentSong?.artists?.map((artist) => artist).join('/') ||
									'未知艺术家'}
							</h2>
						</div>
					</div>
					<div className={styles.action}>
						<span className={`i-solar-heart-angle-line-duotone ${styles.like}`} />
						{false /* 在love列表内 */ && (
							<span className="i-solar-heart-broken-line-duotone" />
						)}
						<span className="i-solar-dialog-2-line-duotone" />
						<span className="i-solar-menu-dots-circle-line-duotone" />
					</div>
				</div>
				<div className={styles.playbar__control}>
					<div className={styles.buttons}>
						<span
							className={`
								${mode === 'list' ? 'i-solar-repeat-line-duotone' : ''}
								${mode === 'random' ? 'i-solar-shuffle-line-duotone' : ''}
								${mode === 'single' ? 'i-solar-repeat-one-line-duotone' : ''}
							`}
							onClick={() => {
								const newMode =
									mode === 'list'
										? 'random'
										: mode === 'random'
											? 'single'
											: 'list';
								usePlayer.mode = newMode;
							}}
						/>
						<span
							className="i-solar-rewind-back-line-duotone"
							onClick={() => usePlayer.prev()}
						/>
						<span
							className={`
				${playing ? 'i-solar-pause-line-duotone' : 'i-solar-play-line-duotone'}
				`}
							onClick={() => {
								if (playing) return usePlayer.pause();
								usePlayer.play();
								// deubg
								console.debug({
									currentSong,
									playlist,
									mode,
									playing,
									muted,
									volume,
									seek,
									duration,
								});
							}}
						/>
						<span
							className="i-solar-rewind-forward-line-duotone"
							onClick={() => usePlayer.next()}
						/>
						<span
							className={`
								${muted ? 'i-solar-muted-line-duotone' : ''}
								${volume < 0.25 ? 'i-solar-volume-line-duotone' : ''}
								${volume < 0.75 ? 'i-solar-volume-small-line-duotone' : ''}
								${volume >= 0.75 ? 'i-solar-volume-loud-line-duotone' : ''}
							`}
							onClick={() => {
								usePlayer.muted = !usePlayer.muted;
							}}
							ref={volumeRef}
							title={'当前音量：' + volume * 100 + '%，点击静音/取消静音'}
						/>
					</div>
					<div className={styles.progress}>
						<span>
							{`${Math.floor(seek / 60)
								.toString()
								.padStart(2, '0')}:${Math.floor(seek % 60)
								.toString()
								.padStart(2, '0')}`}
						</span>
						<input
							type="range"
							min={0}
							max={duration}
							value={seek}
							onMouseDown={() => setSeekDragging(true)}
							onMouseUp={() => {
								setSeekDragging(false);
								usePlayer.seek = seek; // 更新播放器的 seek 值
							}}
							onInput={(e) => {
								if (seekDragging) {
									const newSeek = Number((e.target as HTMLInputElement).value);
									setSeek(newSeek); // 实时更新 seek 值
								}
							}}
						/>
						<span>
							{`${Math.floor(duration / 60)
								.toString()
								.padStart(2, '0')}:${Math.floor(duration % 60)
								.toString()
								.padStart(2, '0')}`}
						</span>
					</div>
				</div>
				<div className={styles.playbar__list}>
					<span
						className="i-solar-hamburger-menu-line-duotone"
						onClick={() => setPlayerListModalOpen(true)}
					/>
				</div>
			</div>
			<Popover
				listen={volumeRef}
				onClose={() => {
					/* 关闭逻辑 */
				}}
				className={styles.volume__popover}
			>
				<input
					type="range"
					min={0}
					max={1}
					step={0.01}
					value={volume}
					onChange={(e) => {
						const newVolume = Number(e.target.value);
						usePlayer.volume = newVolume;
					}}
				/>
				<span>
					{`${Math.floor(volume * 100)
						.toString()
						.padStart(2, '0')}%`}
				</span>
			</Popover>
			<Modal
				isOpen={playerListModalOpen}
				hasCard={false}
				onClose={() => setPlayerListModalOpen(false)}
				style={{ flexDirection: 'row-reverse', justifyContent: 'space-between' }}
			>
				<PlayList onClose={() => setPlayerListModalOpen(false)} />
			</Modal>
		</>
	);
};

export default PlayBar;
