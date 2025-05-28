import { Window } from '@tauri-apps/api/window';
import { debounce } from 'lodash-es';
import { memo, useCallback, useEffect, useMemo, useRef, useState } from 'react';
import cover from '../../assets/images/song.png';
import { usePlayerManager } from '../../context/PlayerContext';
import { LyricContent, PlayListItem } from '../../models/song';
import { useSettingStore } from '../../store/setting';
import Progress from '../atoms/Progress';
import Popover from '../numerator/Popover';

interface LryicProps {
	onClose: () => void;
	className?: string;
}

function parseLyric(
	lyric: LyricContent,
	translated?: LyricContent
): { index: number; text: string; translatedText?: string }[] {
	const parseLrc = (lrcText: string) => {
		return lrcText
			.split('\n')
			.map((line) => {
				const timeMatch = line.match(/^\[(\d{2}):(\d{2})\.(\d+)\]/);
				if (!timeMatch) return null;
				const minute = parseInt(timeMatch[1], 10);
				const second = parseInt(timeMatch[2], 10);
				const millis = parseInt(timeMatch[3], 10);
				const index =
					minute * 60 + second + millis / (timeMatch[3].length === 3 ? 1000 : 100);
				const text = line.slice(timeMatch[0].length).trim();
				return { index, text };
			})
			.filter((item) => item !== null);
	};

	let data;

	if (!translated?.lyric) {
		data = parseLrc(lyric.lyric);
	} else {
		const mainLines = parseLrc(lyric.lyric);
		const transLines = parseLrc(translated.lyric);
		data = transLines.map((tItem) => {
			const mainItem = mainLines.find((mItem) => mItem.index === tItem.index);
			return {
				index: tItem.index,
				text: mainItem?.text || '',
				translatedText: tItem.text,
			};
		});
	}

	if (data.length === 0) {
		data = [{ index: 0, text: '暂无歌词' }];
	}

	return data;
}

const LryicModal: React.FC<LryicProps> = memo(({ onClose, className = '' }) => {
	const usePlayer = usePlayerManager();
	const appWindow = new Window('main');

	const [isFullScreen, setIsFullScreen] = useState(false);
	const [seekDragging, setSeekDragging] = useState(false);
	const [showControls, setShowControls] = useState(true);
	const [isVisible, setIsVisible] = useState(false);

	const volumeRef = useRef<HTMLSpanElement>(null);
	const lyricRef = useRef<HTMLDivElement>(null);
	const lyricListRef = useRef<HTMLDivElement>(null);
	const lastInteractionRef = useRef<number>(Date.now() - 1.5 * 1000);
	const hideControlsTimeoutRef = useRef<NodeJS.Timeout>();

	const [lyricType, setLyricType] = useState<'raw' | 'translate'>(
		useSettingStore.getState().lyricsType
	);

	const [mode, setMode] = useState<'list' | 'random' | 'single'>(usePlayer.mode);
	const [currentSong, setCurrentSong] = useState<PlayListItem>(usePlayer.currentSong);
	const [playing, setPlaying] = useState<boolean>(usePlayer.playing);
	const [duration, setDuration] = useState<number>(usePlayer.duration);
	const [seek, setSeek] = useState<number>(usePlayer.seek);
	const [volume, setVolume] = useState<number>(usePlayer.volume);
	const [muted, setMuted] = useState<boolean>(usePlayer.muted);
	const [lyrics, setLryics] = useState<
		{ index: number; text: string; translatedText?: string }[]
	>(parseLyric(usePlayer.lyric.lrc, usePlayer.lyric.tlyric));

	// 缓存格式化时间
	const formattedSeek = useMemo(() => {
		return `${Math.floor(seek / 60)
			.toString()
			.padStart(2, '0')}:${Math.floor(seek % 60)
			.toString()
			.padStart(2, '0')}`;
	}, [seek]);

	const formattedDuration = useMemo(() => {
		return `${Math.floor(duration / 60)
			.toString()
			.padStart(2, '0')}:${Math.floor(duration % 60)
			.toString()
			.padStart(2, '0')}`;
	}, [duration]);

	const formattedVolume = useMemo(() => {
		return `${Math.floor(volume * 100)
			.toString()
			.padStart(2, '0')}%`;
	}, [volume]);

	// 自动隐藏控制栏
	const resetHideControlsTimer = useCallback(() => {
		if (hideControlsTimeoutRef.current) {
			clearTimeout(hideControlsTimeoutRef.current);
		}
		setShowControls(true);
		hideControlsTimeoutRef.current = setTimeout(() => {
			setShowControls(false);
		}, 3000);
	}, []);

	const handleMouseMove = useCallback(() => {
		resetHideControlsTimer();
	}, [resetHideControlsTimer]);

	const handleScroll = useCallback(() => {
		lastInteractionRef.current = Date.now();
		resetHideControlsTimer();
	}, [resetHideControlsTimer]);

	// 防抖的操作函数
	const debouncedClose = useCallback(
		debounce(async () => {
			const isFullScreen = await appWindow.isFullscreen();
			if (isFullScreen) {
				await appWindow.setFullscreen(false).then(() => setIsFullScreen(false));
			}
			onClose();
		}, 300),
		[appWindow, onClose]
	);

	const debouncedFullScreen = useCallback(
		debounce(async () => {
			const isFullScreen = await appWindow.isFullscreen();
			await appWindow.setFullscreen(!isFullScreen).then(() => setIsFullScreen(!isFullScreen));
		}, 300),
		[appWindow]
	);

	const debouncedModeChange = useCallback(
		debounce(() => {
			const newMode = mode === 'list' ? 'random' : mode === 'random' ? 'single' : 'list';
			usePlayer.mode = newMode;
		}, 300),
		[mode, usePlayer]
	);

	const debouncedNext = useCallback(
		debounce(() => {
			usePlayer.next();
		}, 300),
		[usePlayer]
	);

	const debouncedMute = useCallback(
		debounce(() => {
			usePlayer.muted = !usePlayer.muted;
		}, 300),
		[usePlayer]
	);

	// 播放控制函数
	const handlePlayPause = useCallback(() => {
		if (playing) return usePlayer.pause();
		usePlayer.play();
	}, [playing, usePlayer]);

	const handlePrev = useCallback(() => usePlayer.prev(), [usePlayer]);

	const handleSeekChange = useCallback(
		(newSeek: number) => {
			usePlayer.seek = newSeek;
		},
		[usePlayer]
	);

	const handleVolumeChange = useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const newVolume = Number(e.target.value);
			usePlayer.volume = newVolume;
		},
		[usePlayer]
	);

	useEffect(() => {
		setIsVisible(true);
		resetHideControlsTimer();

		const lyricHeader = document.getElementById('lyric-header');
		if (lyricHeader && isFullScreen) {
			lyricHeader.removeAttribute('data-tauri-drag-region');
		} else if (lyricHeader && !isFullScreen) {
			lyricHeader.setAttribute('data-tauri-drag-region', '');
		}

		lyricListRef.current?.addEventListener('scroll', handleScroll);

		const handleKeyDown = (e: KeyboardEvent) => {
			if (e.key === 'Escape') {
				debouncedClose();
			}
		};
		window.addEventListener('keydown', handleKeyDown);
		window.addEventListener('mousemove', handleMouseMove);

		lyricRef.current?.scrollIntoView({ behavior: 'smooth' });
		lastInteractionRef.current = Date.now() - 1.5 * 1000;

		return () => {
			if (hideControlsTimeoutRef.current) {
				clearTimeout(hideControlsTimeoutRef.current);
			}
			lyricListRef.current?.removeEventListener('scroll', handleScroll);
			window.removeEventListener('keydown', handleKeyDown);
			window.removeEventListener('mousemove', handleMouseMove);
		};
	}, [isFullScreen, handleScroll, handleMouseMove, debouncedClose, resetHideControlsTimer]);

	// 歌词自动滚动
	useEffect(() => {
		const currentTime = Date.now();
		if (currentTime - lastInteractionRef.current >= 3000) {
			lyricRef.current?.scrollIntoView({ behavior: 'smooth' });
		}
	}, [seek]);

	// 状态同步
	useEffect(() => {
		setCurrentSong(usePlayer.currentSong);
		setMode(usePlayer.mode);
		setDuration(usePlayer.duration);
		setPlaying(usePlayer.playing);
		setVolume(usePlayer.volume);
		setMuted(usePlayer.muted);
		setLryics(parseLyric(usePlayer.lyric.lrc, usePlayer.lyric.tlyric));
	}, [
		usePlayer.currentSong,
		usePlayer.playing,
		usePlayer.duration,
		usePlayer.mode,
		usePlayer.volume,
		usePlayer.muted,
		usePlayer.lyric,
	]);

	useEffect(() => {
		if (!seekDragging) setSeek(usePlayer.seek);
	}, [usePlayer.seek, seekDragging]);

	useEffect(() => {
		setLyricType(useSettingStore.getState().lyricsType);
	}, [useSettingStore.getState().lyricsType]);

	// 渲染歌词列表
	const renderedLyrics = useMemo(() => {
		return lyrics.map((item, index) => {
			const isCurrent =
				item.index <= seek &&
				(lyrics[index + 1]?.index > seek || index === lyrics.length - 1);
			const nextIsCurrent =
				lyrics[index + 1]?.index <= seek &&
				(lyrics[index + 2]?.index > seek || index === lyrics.length - 2);

			return (
				<div
					key={item.index}
					onClick={() => handleSeekChange(item.index)}
					className={`flex w-full h-auto flex-col justify-center px-1.2 py-2 rounded cursor-pointer transition-all duration-300 ease-out transform-gpu ${
						item.text !== '' ? 'block' : 'hidden'
					} ${
						isCurrent
							? 'text-white bg-white/10 scale-105 opacity-100 shadow-lg backdrop-blur-sm rounded-lg border-none outline-none'
							: 'text-white/60 opacity-80 hover:text-white hover:bg-white/5 hover:scale-102'
					}`}
					style={{
						animationDelay: `${index * 50}ms`,
					}}
					ref={nextIsCurrent ? lyricRef : null}
				>
					{(lyricType === 'raw' && item.text !== '') ||
					usePlayer.lyric.tlyric?.lyric === '' ? (
						<>
							<h1
								className={`text-1.8rem font-medium transition-all duration-300 ${isCurrent ? 'text-shadow-lg' : ''}`}
							>
								{item.text}
							</h1>
							{item.translatedText && (
								<h2
									className={`text-1.2rem opacity-75 transition-all duration-300 ${isCurrent ? 'text-shadow' : ''}`}
								>
									{item.translatedText}
								</h2>
							)}
						</>
					) : (
						<>
							<h1
								className={`text-1.8rem font-medium transition-all duration-300 ${isCurrent ? 'text-shadow-lg' : ''}`}
							>
								{item.translatedText}
							</h1>
							{item.text && (
								<h2
									className={`text-1.2rem opacity-75 transition-all duration-300 ${isCurrent ? 'text-shadow' : ''}`}
								>
									{item.text}
								</h2>
							)}
						</>
					)}
				</div>
			);
		});
	}, [lyrics, seek, lyricType, usePlayer.lyric.tlyric?.lyric, handleSeekChange]);

	return (
		<div
			id="lyric"
			className={`h-full w-full relative text-white bg-gradient-to-br from-purple-900/20 via-blue-900/20 to-indigo-900/20 backdrop-blur-xl transition-all duration-500 ${
				isVisible ? 'opacity-100 scale-100' : 'opacity-0 scale-95'
			} ${className}`}
		>
			{/* 背景动效 */}
			<div className="absolute inset-0 bg-gradient-to-br from-purple-500/5 via-blue-500/5 to-indigo-500/5 animate-pulse" />

			{/* 头部控制栏 */}
			<div
				id="lyric-header"
				data-tauri-drag-region
				className={`absolute top-0 left-0 w-full h-18 flex items-center flex-row-reverse p-4 transition-all duration-300 ${
					showControls ? 'opacity-100 translate-y-0' : 'opacity-0 -translate-y-full'
				}`}
			>
				<button
					className="h-10 w-10 rounded-8px bg-transparent border-none outline-none flex items-center justify-center text-1.5rem text-white cursor-pointer transition-all duration-300 hover:bg-white/10 hover:scale-110 active:scale-95"
					onClick={debouncedClose}
				>
					<span className="i-solar-minimize-bold" />
				</button>
				<button
					className="h-10 w-10 rounded-8px bg-transparent border-none outline-none flex items-center justify-center text-1.5rem text-white cursor-pointer transition-all duration-300 hover:bg-white/10 hover:scale-110 active:scale-95 mr-2"
					onClick={debouncedFullScreen}
				>
					{isFullScreen ? (
						<span className="i-solar-quit-full-screen-bold" />
					) : (
						<span className="i-solar-full-screen-bold" />
					)}
				</button>
			</div>

			{/* 主体内容 */}
			<div className="my-18 flex flex-row items-center h-4/5">
				{/* 歌曲信息区域 */}
				<div className="flex flex-col justify-center items-center w-full h-full flex-1">
					<div className="flex flex-col items-start mx-4 animate-in slide-in-from-left duration-500">
						<img
							src={currentSong.cover || cover}
							alt={currentSong.name}
							className="w-91.25 h-91.25 rounded-8px bg-transparent shadow-2xl transition-all duration-500 hover:scale-105 hover:shadow-3xl transform-gpu"
							style={{
								filter: 'drop-shadow(0 25px 50px rgba(0, 0, 0, 0.5))',
							}}
						/>
						<div className="max-w-100 mt-4">
							<h1 className="text-1.5rem font-bold text-white mb-2 animate-in slide-in-from-bottom duration-700 delay-200">
								{currentSong.name}
							</h1>
							<p className="text-white/80 animate-in slide-in-from-bottom duration-700 delay-300">
								{currentSong.artists?.map((artist) => artist).join('/')}
							</p>
						</div>
					</div>
				</div>

				{/* 歌词区域 */}
				<div
					className="max-h-4/5 w-full px-4 mr-4 overflow-y-auto flex-1 animate-in slide-in-from-right duration-500 [&::-webkit-scrollbar]:hidden"
					ref={lyricListRef}
				>
					<div className="space-y-2">{renderedLyrics}</div>
				</div>
			</div>

			{/* 底部控制栏 */}
			<div
				className={`absolute bottom-0 left-0 w-full h-18 flex justify-center items-center p-4 transition-all duration-300 ${
					showControls ? 'opacity-100 translate-y-0' : 'opacity-0 translate-y-full'
				}`}
			>
				<div className="flex flex-1 flex-col flex-nowrap justify-center h-full min-w-83 items-center mx-auto">
					{/* 播放控制按钮 */}
					<div className="flex flex-row flex-nowrap items-center justify-center h-full min-w-38.75 mr-4">
						<span
							className={`text-1.3rem cursor-pointer mr-6.25 transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95 ${
								mode === 'list' ? 'i-solar-repeat-line-duotone' : ''
							}${mode === 'random' ? 'i-solar-shuffle-line-duotone' : ''}${
								mode === 'single' ? 'i-solar-repeat-one-line-duotone' : ''
							}`}
							onClick={debouncedModeChange}
						/>
						<span
							className="i-solar-rewind-back-line-duotone text-1.3rem cursor-pointer mr-6.25 transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95"
							onClick={handlePrev}
						/>
						<span
							className={`text-1.3rem cursor-pointer mr-6.25 transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95 ${
								playing ? 'i-solar-pause-line-duotone' : 'i-solar-play-line-duotone'
							}`}
							onClick={handlePlayPause}
						/>
						<span
							className="i-solar-rewind-forward-line-duotone text-1.3rem cursor-pointer mr-6.25 transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95"
							onClick={debouncedNext}
						/>
						<span
							className={`text-1.3rem cursor-pointer transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95 ${
								muted ? 'i-solar-muted-line-duotone' : ''
							}${volume < 0.25 ? 'i-solar-volume-line-duotone' : ''}${
								volume < 0.75 ? 'i-solar-volume-small-line-duotone' : ''
							}${volume >= 0.75 ? 'i-solar-volume-loud-line-duotone' : ''}`}
							onClick={debouncedMute}
							ref={volumeRef}
							title={`当前音量：${formattedVolume}，点击静音/取消静音`}
						/>
					</div>

					{/* 进度条 */}
					<div className="flex-1 flex flex-row flex-nowrap items-center justify-center h-full w-full">
						<span className="text-0.8rem mr-2 font-mono">{formattedSeek}</span>
						<Progress
							max={duration}
							value={seek}
							onMouseDown={() => setSeekDragging(true)}
							onMouseUp={() => {
								setSeekDragging(false);
								handleSeekChange(seek);
							}}
							onInput={(e) => {
								if (seekDragging) {
									const newSeek = Number((e?.target as HTMLInputElement).value);
									setSeek(newSeek);
								}
							}}
							className="flex-1"
						/>
						<span className="text-0.8rem ml-2 font-mono">{formattedDuration}</span>
					</div>
				</div>
			</div>

			{/* 音量控制弹窗 */}
			<Popover
				listen={volumeRef}
				onClose={() => {}}
				className="flex! flex-col items-center flex-nowrap bg-black/80 backdrop-blur-md rounded-8px p-3 border border-white/20"
			>
				<input
					type="range"
					min={0}
					max={1}
					step={0.01}
					value={volume}
					onChange={handleVolumeChange}
					className="appearance-none bg-transparent outline-none rounded-2px cursor-pointer mx-2 w-4 h-25 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-[var(--primary-color)] [&::-webkit-slider-thumb]:cursor-pointer [&::-webkit-slider-track]:bg-white/20 [&::-webkit-slider-track]:rounded-2px [&::-webkit-slider-track]:h-1"
					style={{
						WebkitAppearance: 'slider-vertical',
					}}
				/>
				<span className="text-0.8rem text-[var(--second-text-color)] mt-2">
					{formattedVolume}
				</span>
			</Popover>
		</div>
	);
});

LryicModal.displayName = 'LryicModal';

export default LryicModal;
