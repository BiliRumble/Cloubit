import { debounce } from 'lodash-es';
import { memo, useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getLikeList } from '../../apis/user';
import cover from '../../assets/images/song.png';
import { usePlayerManager } from '../../context/PlayerContext';
import { usePlayerStore } from '../../store/player';
import { useSettingStore } from '../../store/setting';
import { useUserStore } from '../../store/user';
import { eventBus } from '../../utils/EventBus';
import { toLikeSong } from '../../utils/song';
import Progress from '../atoms/Progress';
import Modal from '../numerator/Modal';
import Popover from '../numerator/Popover';
import PlayList from './Playlist';
import LryicModal from './PlayBar/Lyric';

interface PlayBarProps {
	className?: string;
}

const PlayBar: React.FC<PlayBarProps> = memo(({ className }) => {
	const usePlayer = usePlayerManager();
	const navigate = useNavigate();

	const [playerListModalOpen, setPlayerListModalOpen] = useState(false);
	const [lyricModalOpen, setLyricModalOpen] = useState(false);
	const [isVisible, setIsVisible] = useState(false);

	const volumeRef = useRef<HTMLSpanElement>(null);
	const [seekDragging, setSeekDragging] = useState(false);

	const [currentSong, setCurrentSong] = useState(usePlayer.currentSong);
	const [mode, setMode] = useState(usePlayer.mode);
	const [playing, setPlaying] = useState(usePlayer.playing);
	const [muted, setMuted] = useState(usePlayer.muted);
	const [volume, setVolume] = useState(usePlayer.volume);
	const [seek, setSeek] = useState(usePlayer.seek);
	const [duration, setDuration] = useState(usePlayer.duration);
	const [lyrics, setLyrics] = useState<string | null>(null);

	const { likeSongs } = useUserStore.getState();

	// 缓存计算结果
	const isLikeSong = useMemo((): boolean => {
		return likeSongs.ids?.includes(currentSong?.id ?? 0) ?? false;
	}, [likeSongs.ids, currentSong?.id]);

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

	// 缓存防抖函数
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

	const debouncedOpenPlaylist = useCallback(
		debounce(() => {
			setPlayerListModalOpen(true);
		}, 300),
		[]
	);

	// 事件处理函数
	const handleLike = useCallback(() => {
		toLikeSong(currentSong?.id ?? 0, !isLikeSong);
	}, [currentSong?.id, isLikeSong]);

	const handlePlayPause = useCallback(() => {
		if (playing) return usePlayer.pause();
		usePlayer.play();
	}, [playing, usePlayer]);

	const handlePrev = useCallback(() => usePlayer.prev(), [usePlayer]);

	const handleNavigateToComment = useCallback(() => {
		navigate('/comment/song/' + currentSong.id);
	}, [navigate, currentSong.id]);

	const handleOpenLyric = useCallback(() => setLyricModalOpen(true), []);
	const handleCloseLyric = useCallback(() => setLyricModalOpen(false), []);
	const handleClosePlaylist = useCallback(() => setPlayerListModalOpen(false), []);

	const handleVolumeChange = useCallback(
		(e: React.ChangeEvent<HTMLInputElement>) => {
			const newVolume = Number(e.target.value);
			usePlayer.volume = newVolume;
		},
		[usePlayer]
	);

	const updateState = useCallback(() => {
		setCurrentSong(usePlayer.currentSong);
		setMode(usePlayer.mode);
		setMuted(usePlayer.muted);
		setVolume(usePlayer.volume);
		if (!seekDragging) {
			setSeek(usePlayer.seek);
		}
		setDuration(usePlayer.duration);
		setPlaying(usePlayer.playing);
	}, [usePlayer, seekDragging]);

	const handleLyricsUpdate = useCallback(() => {
		if (useSettingStore.getState().showLyrics) {
			eventBus.emit(
				'playerLyricChange',
				usePlayer.currentLyric(useSettingStore.getState().lyricsType)
			);
			setLyrics(usePlayer.currentLyric(useSettingStore.getState().lyricsType));
		} else {
			setLyrics(null);
			eventBus.emit('playerLyricChange', lyrics);
		}
	}, [currentSong, useSettingStore.getState().showLyrics, seek, usePlayer, lyrics]);

	const updateSeek = useCallback(async () => {
		if (useSettingStore.getState().savePlaySeek) usePlayerStore.setState({ seek });
		setPlaying(usePlayer.playing);
	}, [seek, usePlayer]);

	useEffect(() => {
		setIsVisible(true);
		usePlayer.resetPlaylistIndices();
		getLikeList();
	}, [usePlayer]);

	useEffect(() => {
		updateState();
		const interval = setInterval(updateState, 250);
		return () => clearInterval(interval);
	}, [updateState]);

	useEffect(() => {
		handleLyricsUpdate();
	}, [handleLyricsUpdate]);

	useEffect(() => {
		updateSeek();
	}, [updateSeek]);

	useEffect(() => {
		const emitLyricsChange = async () => {
			eventBus.emit('playerLyricChange', lyrics);
		};
		emitLyricsChange();
	}, [lyrics]);

	return (
		<>
			<div
				className={`flex flex-row items-center flex-nowrap px-4 py-2 justify-between bg-[var(--background-color)]/95 backdrop-blur-md border-t border-[var(--hr-color)]/50 transition-all duration-500 ${
					isVisible ? 'translate-y-0 opacity-100' : 'translate-y-full opacity-0'
				} ${className || ''}`}
			>
				{/* 左侧：歌曲信息和操作 */}
				<div className="flex flex-1 flex-row flex-nowrap items-center justify-start">
					{/* 歌曲信息 */}
					<div className="flex flex-1 flex-row flex-nowrap items-center justify-start w-41.25 mr-4 animate-in slide-in-from-left duration-500">
						{/* 封面 */}
						<div className="group h-12 max-w-12 w-12 mr-2 relative overflow-hidden cursor-pointer rounded transition-all duration-300 hover:shadow-lg">
							<img
								src={currentSong?.cover || cover}
								alt={`${currentSong?.name}的封面`}
								className="h-full w-full object-cover transition-transform duration-300 group-hover:scale-110"
							/>
							<div
								className="absolute inset-0 bg-black/50 backdrop-blur-8px opacity-0 group-hover:opacity-100 transition-all duration-200 flex items-center justify-center"
								onClick={handleOpenLyric}
							>
								<span className="i-solar-maximize-linear text-white text-1.5rem" />
							</div>
						</div>

						{/* 歌曲文本信息 */}
						<div className="overflow-hidden flex-1 whitespace-nowrap w-full flex flex-col flex-nowrap items-start min-w-0">
							<h1 className="text-1rem font-medium text-[var(--text-color)] truncate w-full transition-colors duration-200 hover:text-[var(--primary-color)] cursor-pointer">
								{currentSong?.name || '未知歌曲'}
							</h1>
							<h2 className="text-0.8rem text-[var(--second-text-color)] truncate w-full transition-all duration-300">
								{!lyrics
									? currentSong.artists?.map((artist) => artist).join('/') ||
										'未知艺术家'
									: lyrics}
							</h2>
						</div>
					</div>

					{/* 操作按钮 */}
					<div className="flex flex-1 flex-row flex-nowrap items-center justify-start mr-4 min-w-38.75 h-full animate-in slide-in-from-left duration-700 delay-200">
						<span
							onClick={handleLike}
							className={`text-1.3rem cursor-pointer mr-6.25 transition-all duration-200 hover:scale-110 active:scale-95 ${
								isLikeSong
									? 'i-solar-heart-broken-line-duotone text-red-500 hover:text-red-600'
									: 'i-solar-heart-angle-line-duotone text-[var(--second-text-color)] hover:text-red-500'
							}`}
							title={isLikeSong ? '取消喜欢' : '喜欢歌曲'}
						/>
						<span
							onClick={handleNavigateToComment}
							className="i-solar-dialog-2-line-duotone text-1.3rem cursor-pointer mr-6.25 text-[var(--second-text-color)] transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95"
							title="查看评论"
						/>
						<span
							className="i-solar-menu-dots-circle-line-duotone text-1.3rem cursor-pointer text-[var(--second-text-color)] transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95"
							title="更多操作"
						/>
					</div>
				</div>

				{/* 中间：播放控制 */}
				<div className="flex flex-1 flex-col flex-nowrap justify-center h-full min-w-38.75 items-center mx-auto animate-in slide-in-from-bottom duration-500 delay-100">
					{/* 播放按钮 */}
					<div className="flex flex-row flex-nowrap items-center justify-center h-full min-w-38.75 mb-2">
						<span
							className={`text-1.3rem cursor-pointer mr-6.25 transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95 ${
								mode === 'list' ? 'i-solar-repeat-line-duotone' : ''
							}${mode === 'random' ? 'i-solar-shuffle-line-duotone' : ''}${
								mode === 'single' ? 'i-solar-repeat-one-line-duotone' : ''
							}`}
							onClick={debouncedModeChange}
							title={`播放模式：${mode === 'list' ? '列表循环' : mode === 'random' ? '随机播放' : '单曲循环'}`}
						/>
						<span
							className="i-solar-rewind-back-line-duotone text-1.3rem cursor-pointer mr-6.25 transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95"
							onClick={handlePrev}
							title="上一首"
						/>
						<span
							className={`text-1.5rem cursor-pointer mr-6.25 transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95 p-2 bg-[var(--primary-color)]/10 hover:bg-[var(--primary-color)]/20 ${
								playing ? 'i-solar-pause-line-duotone' : 'i-solar-play-line-duotone'
							}`}
							onClick={handlePlayPause}
							title={playing ? '暂停' : '播放'}
						/>
						<span
							className="i-solar-rewind-forward-line-duotone text-1.3rem cursor-pointer mr-6.25 transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95"
							onClick={debouncedNext}
							title="下一首"
						/>
						<span
							className={`text-1.3rem cursor-pointer transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95 
							${
								muted
									? 'i-solar-muted-line-duotone text-red-500'
									: volume < 0.25
										? 'i-solar-volume-line-duotone'
										: volume < 0.75
											? 'i-solar-volume-small-line-duotone'
											: volume >= 0.75
												? 'i-solar-volume-loud-line-duotone'
												: ''
							}`}
							onClick={debouncedMute}
							ref={volumeRef}
							title={`当前音量：${formattedVolume}，点击${muted ? '取消静音' : '静音'}`}
						/>
					</div>

					{/* 进度条 */}
					<div className="flex-1 flex flex-row flex-nowrap items-center justify-center h-full w-full">
						<span className="text-0.8rem text-[var(--second-text-color)] mr-2 font-mono min-w-10 text-right">
							{formattedSeek}
						</span>
						<Progress
							max={duration}
							value={seek}
							onMouseDown={() => setSeekDragging(true)}
							onMouseUp={() => {
								setSeekDragging(false);
								usePlayer.seek = seek;
							}}
							onInput={(e) => {
								if (seekDragging) {
									const newSeek = Number((e?.target as HTMLInputElement).value);
									setSeek(newSeek);
								}
							}}
							className="flex-1 mx-2"
						/>
						<span className="text-0.8rem text-[var(--second-text-color)] ml-2 font-mono min-w-10 text-left">
							{formattedDuration}
						</span>
					</div>
				</div>

				{/* 右侧：播放列表 */}
				<div className="flex flex-1 flex-row flex-nowrap items-center justify-end mr-4 min-w-38.75 h-full animate-in slide-in-from-right duration-500 delay-300">
					<span
						className="i-solar-hamburger-menu-line-duotone text-1.3rem cursor-pointer transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95 p-2 rounded-full hover:bg-[var(--button-hover-bg-color)]"
						onClick={debouncedOpenPlaylist}
						title="播放列表"
					/>
				</div>
			</div>

			{/* 音量控制弹窗 */}
			<Popover
				listen={volumeRef}
				onClose={() => {}}
				className="flex! flex-col items-center flex-nowrap bg-[var(--background-color)]/95 backdrop-blur-md rounded-8px p-3 border border-[var(--hr-color)]/50 shadow-lg"
			>
				<input
					type="range"
					min={0}
					max={1}
					step={0.01}
					value={volume}
					onChange={handleVolumeChange}
					className="appearance-none bg-transparent outline-none rounded-2px cursor-pointer mx-2 w-4 h-25 [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:w-4 [&::-webkit-slider-thumb]:h-4 [&::-webkit-slider-thumb]:rounded-full [&::-webkit-slider-thumb]:bg-[var(--primary-color)] [&::-webkit-slider-thumb]:cursor-pointer [&::-webkit-slider-track]:bg-[var(--hr-color)] [&::-webkit-slider-track]:rounded-2px [&::-webkit-slider-track]:h-1"
					style={{
						WebkitAppearance: 'slider-vertical',
					}}
				/>
				<span className="text-0.8rem text-[var(--second-text-color)] mt-2">
					{formattedVolume}
				</span>
			</Popover>

			{/* 播放列表模态框 */}
			<Modal
				isOpen={playerListModalOpen}
				hasCard={false}
				onClose={handleClosePlaylist}
				className="animate-in slide-in-from-right duration-300"
				style={{ flexDirection: 'row-reverse', justifyContent: 'space-between' }}
			>
				<PlayList onClose={handleClosePlaylist} />
			</Modal>

			{/* 歌词模态框 */}
			<Modal
				isOpen={lyricModalOpen}
				hasCard={false}
				onClose={handleCloseLyric}
				className="bg-cover bg-center bg-no-repeat bg-fixed backdrop-blur-24px transition-all duration-300 animate-in fade-in"
				style={{ background: `url(${currentSong?.cover})` }}
			>
				<LryicModal onClose={handleCloseLyric} />
			</Modal>
		</>
	);
});

PlayBar.displayName = 'PlayBar';

export default PlayBar;
