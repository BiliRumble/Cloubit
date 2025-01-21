import { Window } from '@tauri-apps/api/window';
import { debounce } from 'lodash-es';
import { useEffect, useRef, useState } from 'react';
import { usePlayerManager } from '../../../../context/PlayerContext';
import Popover from '../../../Common/Popover';
import styles from './Lyric.module.scss';

interface LryicProps {
	onClose: () => void;
	className?: string;
}

const LryicModal: React.FC<LryicProps> = ({ onClose, className = '' }) => {
	const usePlayer = usePlayerManager();
	const window = new Window('main');

	const [isFullScreen, setIsFullScreen] = useState(false);
	const [seekDragging, setSeekDragging] = useState(false);
	const volumeRef = useRef<HTMLSpanElement>(null);

	const [mode, setMode] = useState<'list' | 'random' | 'single'>(usePlayer.mode);
	const [currentSong, setCurrentSong] = useState(usePlayer.currentSong);
	const [playing, setPlaying] = useState(usePlayer.playing);
	const [duration, setDuration] = useState(usePlayer.duration);
	const [seek, setSeek] = useState(usePlayer.seek);
	const [volume, setVolume] = useState(usePlayer.volume);
	const [muted, setMuted] = useState(usePlayer.muted);

	useEffect(() => {
		// 删除 data-tauri-drag-region 属性
		const lyricHeader = document.getElementById('lyric-header');
		if (lyricHeader && isFullScreen) {
			lyricHeader.removeAttribute('data-tauri-drag-region');
		} else if (lyricHeader && !isFullScreen) {
			lyricHeader.setAttribute('data-tauri-drag-region', '');
		}
	}, [isFullScreen]);

	// 慢更新
	useEffect(() => {
		setMode(usePlayer.mode);
		setDuration(usePlayer.duration);
		setPlaying(usePlayer.playing);
		setVolume(usePlayer.volume);
		setMuted(usePlayer.muted);
	}, [
		usePlayer.playing,
		usePlayer.duration,
		usePlayer.mode,
		usePlayer.volume,
		usePlayer.muted,
		usePlayer.lyric,
	]);

	// 快更新
	useEffect(() => {
		if (!seekDragging) setSeek(usePlayer.seek);
	}, [usePlayer.seek]);

	const fullScreen = async () => {
		const isFullScreen = await window.isFullscreen();
		await window.setFullscreen(!isFullScreen).then(() => setIsFullScreen(!isFullScreen));
	};
	const close = async () => {
		const isFullScreen = await window.isFullscreen();
		if (isFullScreen) {
			await window.setFullscreen(false).then(() => setIsFullScreen(false));
		}
		onClose();
	};

	return (
		<div id="lyric" className={styles.lyric + ' ' + className}>
			<div id="lyric-header" data-tauri-drag-region className={styles.lyric__header}>
				<button
					className={styles.lyric__header__button}
					onClick={() => debounce(close, 300)()}
				>
					<span className=" i-solar-minimize-bold" />
				</button>
				<button
					className={styles.lyric__header__button}
					onClick={() => debounce(fullScreen, 300)()}
				>
					{isFullScreen ? (
						<span className=" i-solar-quit-full-screen-bold" />
					) : (
						<span className=" i-solar-full-screen-bold" />
					)}
				</button>
			</div>
			<div className={styles.lyric__body}>
				<div className={styles.lyric__body__info}>
					<div className={styles.lyric__body__info__inner}>
						<img src={usePlayer.currentSong.cover} alt={usePlayer.currentSong.name} />
						<div className={styles.lyric__body__info__text}>
							<h1 className={styles.lyric__body__info__text__title}>
								{usePlayer.currentSong.name}
							</h1>
							<p>{currentSong.artists?.map((artist) => artist).join('/')}</p>
						</div>
					</div>
				</div>
				<div className={styles.lyric__body__lyric}>
					{/* todo: 渲染歌词列表 & 标出当前播放内容 & 点击跳转 */}
				</div>
			</div>
			<div className={styles.lyric__bottom}>
				<div className={styles.lyric__bottom__center}>
					<div className={styles.lyric__bottom__center__bar}>
						<div className={styles.lyric__bottom__center__bar__controls}>
							<span
								className={`
								${mode === 'list' ? 'i-solar-repeat-line-duotone' : ''}
								${mode === 'random' ? 'i-solar-shuffle-line-duotone' : ''}
								${mode === 'single' ? 'i-solar-repeat-one-line-duotone' : ''}
							`}
								onClick={() =>
									debounce(() => {
										const newMode =
											mode === 'list'
												? 'random'
												: mode === 'random'
													? 'single'
													: 'list';
										usePlayer.mode = newMode;
									}, 300)()
								}
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
								}}
							/>
							<span
								className="i-solar-rewind-forward-line-duotone"
								onClick={() =>
									debounce(() => {
										usePlayer.next();
									}, 300)()
								}
							/>
							<span
								className={`
								${muted ? 'i-solar-muted-line-duotone' : ''}
								${volume < 0.25 ? 'i-solar-volume-line-duotone' : ''}
								${volume < 0.75 ? 'i-solar-volume-small-line-duotone' : ''}
								${volume >= 0.75 ? 'i-solar-volume-loud-line-duotone' : ''}
							`}
								onClick={() =>
									debounce(() => {
										usePlayer.muted = !usePlayer.muted;
									}, 300)()
								}
								ref={volumeRef}
								title={'当前音量：' + volume * 100 + '%，点击静音/取消静音'}
							/>
						</div>
						<div className={styles.lyric__bottom__center__bar__progress}>
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
										const newSeek = Number(
											(e.target as HTMLInputElement).value
										);
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
		</div>
	);
};

export default LryicModal;
