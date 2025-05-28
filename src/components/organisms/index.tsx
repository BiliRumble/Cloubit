import { openUrl } from '@tauri-apps/plugin-opener';
import React, { useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { FixedSizeList as List } from 'react-window';
import { getLikeList } from '@/apis/user';
import LazyImage from '@/components/atoms/Image';
import ContextMenu, { MenuItem } from '@/components/numerator/ContextMenu';
import { usePlayerManager } from '@/context/PlayerContext';
import { Artist, Song } from '@/models/common';
import { useUserStore } from '@/store/user';
import { toLikeSong } from '@/utils/song';

interface SongListProps {
	songs: Song[];
	className?: string;
	style?: React.CSSProperties;
	virtualized?: boolean;
	height?: number;
}

// 单个歌曲项组件，使用 memo 优化
const SongItem = React.memo<{
	song: Song;
	index: number;
	isActive: boolean;
	isLiked: boolean;
	onPlay: (song: Song) => void;
	onLike: (songId: number) => void;
	onNavigate: (path: string) => void;
	menuItems: MenuItem[];
}>(({ song, isActive, isLiked, onPlay, onLike, onNavigate, menuItems }) => {
	const handlePlay = useCallback(() => onPlay(song), [onPlay, song]);
	const handleLike = useCallback(() => onLike(song.id), [onLike, song.id]);
	const handleAlbumClick = useCallback(
		() => onNavigate(`/album/${song.album.id}`),
		[onNavigate, song.album.id]
	);

	const formatDuration = useMemo(() => {
		const minutes = Math.floor(song.duration / 1000 / 60);
		const seconds = Math.floor((song.duration / 1000) % 60);
		return `${minutes < 10 ? '0' : ''}${minutes}:${seconds < 10 ? '0' : ''}${seconds}`;
	}, [song.duration]);

	const artistNames = useMemo(
		() => song.artists.map((artist: Artist) => artist.name).join(' / '),
		[song.artists]
	);

	const coverUrl = useMemo(() => song.album.img1v1Url, [song.album.picUrl, song.album.img1v1Url]);
	console.debug(song);

	return (
		<div
			className={`
				flex items-center py-2 px-4 hover:bg-[var(--button-hover-bg-color)]
				transition-colors duration-200 cursor-pointer border-b border-[var(--border)] ${isActive ? 'bg-[var(--button-focus-bg-color)]' : ''}
			`}
			data-context-data={song}
			onClick={handlePlay}
		>
			{/* 标题和艺术家 */}
			<div className="flex items-center flex-1 min-w-0">
				<LazyImage
					className="w-12 h-12 rounded-md object-cover flex-shrink-0"
					src={coverUrl}
					alt={song.name}
				/>
				<div className="ml-3 min-w-0 flex-1">
					<h3 className="text-sm font-medium text-[var(--text-color)]/900 truncate">
						{song.name}
					</h3>
					<p className="text-xs text-[var(--text-color)]/500 truncate">{artistNames}</p>
				</div>
			</div>

			{/* 专辑 */}
			<div className="flex-1 min-w-0 px-4 hidden md:block">
				<h3
					className="text-sm text-[var(--text-color)] truncate hover:text-[var(--primary-color)] cursor-pointer transition-colors duration-100 w-atuo"
					onClick={handleAlbumClick}
				>
					{song.album.name}
				</h3>
			</div>

			{/* 操作 */}
			<div className="flex items-center justify-center w-16">
				<span
					className={`
						text-lg cursor-pointer transition-colors duration-200
						${
							isLiked
								? 'i-solar-heart-broken-line-duotone text-red-500 hover:text-red-600'
								: 'i-solar-heart-angle-line-duotone text-gray-400 hover:text-red-500'
						}
					`}
					onClick={(e) => {
						e.stopPropagation();
						handleLike();
					}}
				/>
			</div>

			{/* 时长 */}
			<div className="w-16 text-right">
				<span className="text-sm text-[var(--text-color)]/500">{formatDuration}</span>
			</div>
		</div>
	);
});

SongItem.displayName = 'SongItem';

const SongList: React.FC<SongListProps> = ({
	songs,
	className = '',
	style,
	virtualized = false,
	height = 600,
}) => {
	const navigate = useNavigate();
	const usePlayer = usePlayerManager();
	const [currentID, setCurrentID] = useState<number>(-1);
	const { likeSongs } = useUserStore();
	const intervalRef = useRef<NodeJS.Timeout | null>(null);

	useEffect(() => {
		getLikeList();
	}, []);

	// 修复 interval 问题，使用 ref 和正确的清理
	useEffect(() => {
		intervalRef.current = setInterval(() => {
			setCurrentID(usePlayer.currentSong.id);
		}, 250);

		return () => {
			if (intervalRef.current) {
				clearInterval(intervalRef.current);
			}
		};
	}, [usePlayer.currentSong.id]);

	const isLikeSong = useCallback(
		(songId: number): boolean => {
			return likeSongs.ids?.includes(songId) ?? false;
		},
		[likeSongs.ids]
	);

	const handleLike = useCallback(
		(songId: number) => {
			toLikeSong(songId, !isLikeSong(songId));
		},
		[isLikeSong]
	);

	const play = useCallback(
		(song: Song) => {
			const artistNames: string[] = song.artists.map((a) => a.name);
			const cover = song.album.picUrl;

			usePlayer.addToPlaylist({
				index: usePlayer.playlist.count,
				id: song.id,
				name: song.name,
				cover,
				source: -2,
				artists: artistNames,
			});
			usePlayer.setCurrentSong(song.id, true);
		},
		[usePlayer]
	);

	const handleNavigate = useCallback(
		(path: string) => {
			navigate(path);
		},
		[navigate]
	);

	const menuItems = useCallback(
		(song: Song): MenuItem[] => [
			{
				label: <p>播放</p>,
				onClick: () => play(song),
			},
			{
				label: <p>评论</p>,
				onClick: () => navigate('/comment/song/' + song.id),
			},
			{
				label: <p>打开链接</p>,
				onClick: () => openUrl(`https://music.163.com/#/song?id=${song.id}`),
			},
		],
		[play, navigate]
	);

	// 虚拟化列表项渲染器
	const VirtualizedItem = useCallback(
		({ index, style: itemStyle }: any) => {
			const song = songs[index];
			const songMenuItems = menuItems(song);

			return (
				<div style={itemStyle}>
					<SongItem
						song={song}
						index={index}
						isActive={currentID === song.id}
						isLiked={isLikeSong(song.id)}
						onPlay={play}
						onLike={handleLike}
						onNavigate={handleNavigate}
						menuItems={songMenuItems}
					/>
				</div>
			);
		},
		[songs, currentID, isLikeSong, play, handleLike, handleNavigate, menuItems]
	);

	// 渲染表头
	const renderHeader = () => (
		<div className="flex items-center py-3 px-4 bg-[var(--button-bg-color)] border-b border-[var(--border-color)] sticky top-0 z-10">
			<div className="flex-1">
				<h2 className="text-sm font-semibold text-[var(--text-color)]/900]">标题</h2>
			</div>
			<div className="flex-1 px-4 hidden md:block">
				<h2 className="text-sm font-semibold text-[var(--text-color)]/900">专辑</h2>
			</div>
			<div className="w-16 text-center">
				<h2 className="text-sm font-semibold text-[var(--text-color)]/900]">操作</h2>
			</div>
			<div className="w-16 text-right">
				<h2 className="text-sm font-semibold text-[var(--text-color)]/900">时长</h2>
			</div>
		</div>
	);

	// 如果启用虚拟化且歌曲数量较多
	if (virtualized && songs.length > 50) {
		return (
			<div
				className={`bg-[var(--button-bg-color)] rounded-lg shadow-sm border border-[var(--border-color)] ${className}`}
				style={style}
			>
				{renderHeader()}
				<List
					height={height - 60} // 减去表头高度
					itemCount={songs.length}
					width={'100%'}
					itemSize={64} // 每项高度
				>
					{VirtualizedItem}
				</List>
			</div>
		);
	}

	// 常规渲染
	return (
		<div
			className={`bg-[var(--button-bg-color)] rounded-lg shadow-sm border border-[var(--border-color)] ${className}`}
			style={style}
		>
			{renderHeader()}
			<div className="max-h-96 overflow-y-auto">
				{songs.map((song, index) => {
					const songMenuItems = menuItems(song);
					return (
						<SongItem
							key={song.id}
							song={song}
							index={index}
							isActive={currentID === song.id}
							isLiked={isLikeSong(song.id)}
							onPlay={play}
							onLike={handleLike}
							onNavigate={handleNavigate}
							menuItems={songMenuItems}
						/>
					);
				})}
			</div>
		</div>
	);
};

export default React.memo(SongList);
