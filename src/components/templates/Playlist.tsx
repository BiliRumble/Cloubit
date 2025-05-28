import { debounce } from 'lodash-es';
import { memo, useCallback, useEffect, useMemo, useState } from 'react';
import { usePlayerManager } from '../../context/PlayerContext';
import { PlayList, PlayListItem } from '../../models/song';
import LazyImage from '../atoms/Image';

interface PlayListProps {
	onClose: () => void;
	className?: string;
	style?: React.CSSProperties;
}

const PlayListModal: React.FC<PlayListProps> = memo(({ onClose, className = '', style }) => {
	const usePlayer = usePlayerManager();
	const [playlist, setPlaylist] = useState<PlayList>({
		count: 0,
		data: [],
	});
	const [nowPlaying, setNowPlaying] = useState<number>(0);
	const [isVisible, setIsVisible] = useState(false);

	// 缓存防抖函数
	const debouncedSetCurrentSong = useCallback(
		debounce((id: number) => {
			usePlayer.setCurrentSong(id, true);
		}, 300),
		[usePlayer]
	);

	const handleSongClick = useCallback(
		(item: PlayListItem) => {
			debouncedSetCurrentSong(item.id);
			onClose();
		},
		[debouncedSetCurrentSong, onClose]
	);

	const handleRemoveSong = useCallback(
		(id: number, e: React.MouseEvent) => {
			e.stopPropagation();
			usePlayer.removeFromPlaylist(id);
		},
		[usePlayer]
	);

	const handleClearPlaylist = useCallback(() => {
		usePlayer.clearPlaylist();
	}, [usePlayer]);

	// 缓存渲染的播放列表
	const renderedPlaylist = useMemo(() => {
		return playlist.data.map((item, index) => {
			const isActive = item.id === nowPlaying;

			return (
				<div
					key={item.id}
					className={`group flex items-center m-4px p-3 rounded-8px cursor-pointer transition-all duration-300 ease-out transform-gpu hover:scale-102 ${
						isActive
							? 'bg-[var(--button-focus-bg-color)] shadow-md ring-1 ring-[var(--primary-color)]/20'
							: 'hover:bg-[var(--button-hover-bg-color)] hover:shadow-sm'
					} ${index !== playlist.data.length - 1 ? 'mb-2' : ''}`}
					onClick={() => handleSongClick(item)}
					style={{
						animationDelay: `${index * 50}ms`,
					}}
				>
					{/* 封面图片 - 恢复悬浮放大效果 */}
					<div className="relative overflow-hidden rounded-8px mr-4 flex-shrink-0">
						<LazyImage
							src={item.cover || ''}
							alt={item.name}
							className="w-12.5 h-12.5 object-cover transition-transform duration-300 group-hover:scale-110"
						/>
						{isActive && (
							<div className="absolute inset-0 bg-black/30 flex items-center justify-center">
								<div className="w-3 h-3 bg-[var(--primary-color)] rounded-full animate-pulse" />
							</div>
						)}
					</div>

					{/* 歌曲信息 */}
					<div className="flex flex-2 w-full flex-col items-start flex-nowrap pr-3.2 min-w-0">
						<h3
							className={`max-w-full whitespace-nowrap overflow-hidden text-ellipsis text-sm font-medium transition-colors duration-200 ${
								isActive
									? 'text-[var(--primary-color)]'
									: 'text-[var(--text-color)] group-hover:text-[var(--primary-color)]'
							}`}
						>
							{item.name}
						</h3>
						<span className="max-w-full whitespace-nowrap overflow-hidden text-ellipsis text-[var(--second-text-color)] text-0.9rem mt-0.5">
							{Array.isArray(item.artists) ? item.artists.join('/') : item.artists}
						</span>
					</div>

					{/* 操作按钮 */}
					<div className="flex items-center space-x-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
						<button
							className="flex items-center justify-center w-8 h-8 rounded-full bg-transparent border-none text-[var(--second-text-color)] hover:text-red-500 hover:bg-red-500/10 transition-all duration-200 cursor-pointer"
							onClick={(e) => handleRemoveSong(item.id, e)}
							title="从播放列表中移除"
						>
							<span className="i-solar-trash-bin-2-linear text-1.2rem" />
						</button>
					</div>
				</div>
			);
		});
	}, [playlist.data, nowPlaying, handleSongClick, handleRemoveSong]);

	// 播放列表容器也需要调整内边距
	<div className="flex flex-col overflow-y-auto h-full w-full max-w-full py-4 px-2 overflow-x-hidden animate-in slide-in-from-bottom duration-700 [&::-webkit-scrollbar]:w-1.5 [&::-webkit-scrollbar]:transition-all [&::-webkit-scrollbar]:duration-300 [&::-webkit-scrollbar:hover]:w-2.5 [&::-webkit-scrollbar-track]:bg-transparent [&::-webkit-scrollbar-thumb]:bg-[var(--hr-color)] [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb:hover]:bg-[var(--second-text-color)]">
		{playlist.data.length === 0 ? (
			<div className="flex flex-col items-center justify-center h-full text-[var(--second-text-color)] animate-in fade-in duration-500">
				<span className="i-solar-music-note-4-line-duotone text-4xl mb-4 opacity-50" />
				<p className="text-center">播放列表为空</p>
				<p className="text-sm text-center opacity-70">添加一些歌曲开始播放吧</p>
			</div>
		) : (
			<div className="space-y-1">{renderedPlaylist}</div>
		)}
	</div>;

	useEffect(() => {
		setIsVisible(true);
		setPlaylist(usePlayer.playlist);
		setNowPlaying(usePlayer.currentSong.id);
	}, [usePlayer.playlist, usePlayer.currentSong]);

	return (
		<div
			className={`flex flex-col overflow-hidden h-full w-21% min-w-87.5 min-h-full max-w-101.25 max-h-screen bg-[var(--background-color)] p-4 rounded-l-8px backdrop-blur-sm border-l border-[var(--hr-color)]/50 transition-all duration-500 ${
				isVisible ? 'translate-x-0 opacity-100' : 'translate-x-full opacity-0'
			} ${className}`}
			style={style}
		>
			{/* 头部区域 */}
			<div className="flex flex-col text-[var(--text-color)] pb-4 border-b border-[var(--hr-color)] relative animate-in slide-in-from-top duration-500">
				<div className="flex items-center justify-between mb-2">
					<h2 className="text-lg font-semibold text-[var(--text-color)] m-0">播放列表</h2>
					<button
						className="absolute top-0 right-0 bg-transparent border-none text-2xl text-[var(--text-color)] cursor-pointer transition-all duration-200 hover:text-[var(--primary-color)] hover:scale-110 active:scale-95 w-8 h-8 flex items-center justify-center rounded-full hover:bg-[var(--button-hover-bg-color)]"
						onClick={onClose}
						title="关闭播放列表"
					>
						<span className="i-solar-close-circle-line-duotone" />
					</button>
				</div>

				<div className="flex items-center justify-between">
					<span className="text-[var(--second-text-color)] text-sm">
						共{' '}
						<span className="text-[var(--primary-color)] font-medium">
							{playlist.count}
						</span>{' '}
						首歌曲
					</span>
					<button
						className="text-[var(--second-text-color)] text-sm cursor-pointer transition-colors duration-200 hover:text-[var(--primary-color)] bg-transparent border-none underline decoration-dotted underline-offset-2"
						onClick={handleClearPlaylist}
						title="清空播放列表"
					>
						清空列表
					</button>
				</div>
			</div>

			{/* 播放列表 */}
			<div className="flex flex-col overflow-y-auto h-full w-full max-w-full pt-4 overflow-x-hidden animate-in slide-in-from-bottom duration-700 [&::-webkit-scrollbar]:w-1.5 [&::-webkit-scrollbar]:transition-all [&::-webkit-scrollbar]:duration-300 [&::-webkit-scrollbar:hover]:w-2.5 [&::-webkit-scrollbar-track]:bg-transparent [&::-webkit-scrollbar-thumb]:bg-[var(--hr-color)] [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb:hover]:bg-[var(--second-text-color)]">
				{playlist.data.length === 0 ? (
					<div className="flex flex-col items-center justify-center h-full text-[var(--second-text-color)] animate-in fade-in duration-500">
						<span className="i-solar-music-note-4-line-duotone text-4xl mb-4 opacity-50" />
						<p className="text-center">播放列表为空</p>
						<p className="text-sm text-center opacity-70">添加一些歌曲开始播放吧</p>
					</div>
				) : (
					<div className="space-y-1">{renderedPlaylist}</div>
				)}
			</div>

			{/* 底部统计信息 - 移除时长相关内容 */}
			{playlist.data.length > 0 && (
				<div className="flex items-center justify-between pt-3 mt-3 border-t border-[var(--hr-color)]/50 text-xs text-[var(--second-text-color)] animate-in slide-in-from-bottom duration-700 delay-200">
					<span>
						正在播放第 {playlist.data.findIndex((item) => item.id === nowPlaying) + 1}{' '}
						首
					</span>
					<span className="flex items-center">
						<span className="i-solar-music-notes-line-duotone mr-1" />共{' '}
						{playlist.count} 首歌曲
					</span>
				</div>
			)}
		</div>
	);
});

PlayListModal.displayName = 'PlayListModal';

export default PlayListModal;
