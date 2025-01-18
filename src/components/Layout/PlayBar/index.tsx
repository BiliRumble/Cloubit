import { event } from '@tauri-apps/api';
import { useEffect, useState } from 'react';
import { usePlayerManager } from '../../../context/PlayerContext';
import { usePlayerStore } from '../../../store/player';
import styles from './PlayBar.module.scss';
import { useSettingStore } from '../../../store/setting';

interface PlayBarProps {
	className?: string;
}

const PlayBar: React.FC<PlayBarProps> = ({ className }) => {
	const usePlayer = usePlayerManager();
	const [currentSong, setCurrentSong] = useState(usePlayer.currentSong);
	const [mode, setMode] = useState(usePlayer.mode);
	const [playing, setPlaying] = useState(usePlayer.playing);
	const [muted, setMuted] = useState(usePlayer.muted);
	const [volume, setVolume] = useState(usePlayer.volume);
	const [seek, setSeek] = useState(usePlayer.seek);
	const [duration, setDuration] = useState(usePlayer.duration);
	const [playlist, setPlaylist] = useState(usePlayer.playlist);

	const [seekDragging, setSeekDragging] = useState(false);

	// 同步播放器状态
	useEffect(() => {
		const updateState = () => {
			setCurrentSong(usePlayer.currentSong);
			setMode(usePlayer.mode);
			setPlaying(usePlayer.playing);
			setMuted(usePlayer.muted);
			setVolume(usePlayer.volume);
			if (!seekDragging) {
				setSeek(usePlayer.seek);
			}
			setDuration(usePlayer.duration);
			setPlaylist(usePlayer.playlist);
		};

		updateState();

		usePlayerStore.setState({ seek: seek });

		const interval = setInterval(updateState, 250); // 每秒更新一次状态

		return () => clearInterval(interval);
	}, [usePlayer, seekDragging]);

	useEffect(() => {
		const lazyAsync = async () => {
			await event.emitTo('main', 'player-update-duration', duration);
			await event.emitTo('main', 'player-update-current-song', currentSong);
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
		event.emitTo('main', 'player-update-current-song', currentSong);
		event.emitTo('main', 'player-update-duration', duration);
		if (useSettingStore.getState().pushToSMTC) usePlayer.pushToSTMC();
	}, [currentSong]);

	useEffect(() => {
		const async = async () => {
			await event.emitTo('main', 'player-update-seek', seek);
		};
		async();
	}, [seek]);

	useEffect(() => {
		// 注册事件
		const prev = event.listen('pip-prev', () => {
			usePlayer.prev();
		});

		const next = event.listen('pip-next', () => {
			usePlayer.next();
		});

		const play = event.listen('pip-play', () => {
			if (playing) {
				console.log('暂停');
				return usePlayer.pause();
			} else {
				console.log('播放');
				return usePlayer.play();
			}
		});

		event.emitTo('main', 'player-update-current-song', currentSong);
		event.emitTo('main', 'player-update-duration', duration);
		event.emitTo('main', 'player-update-playing', playing);

		return () => {
			// 取消事件监听
			prev.then((f) => f());
			next.then((f) => f());
			play.then((f) => f());
		};
	}, [playing]);

	useEffect(() => {
		const pipRequest = event.listen('pip-request', () => {
			event.emitTo('main', 'player-update-current-song', currentSong);
			event.emitTo('main', 'player-update-duration', duration);
		});
		return () => {
			pipRequest.then((f) => f());
		};
	}, []);

	return (
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
								mode === 'list' ? 'random' : mode === 'random' ? 'single' : 'list';
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
					onClick={() => {
						console.log(playlist);
					}}
				/>
			</div>
		</div>
	);
};

export default PlayBar;
