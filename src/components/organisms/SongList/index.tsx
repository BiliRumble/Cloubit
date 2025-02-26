import { openUrl } from '@tauri-apps/plugin-opener';
import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getLikeList } from '../../../apis/user';
import { usePlayerManager } from '../../../context/PlayerContext';
import { Artist } from '../../../models/search';
import { useUserStore } from '../../../store/user';
import { toLikeSong } from '../../../utils/song';
import LazyImage from '../../atoms/LazyImage';
import ContextMenu, { MenuItem } from '../../numerator/ContextMenu';
import styles from './SongList.module.scss';

interface SongListProps {
	songs: any[];
	className?: string;
	style?: React.CSSProperties;
}

const SongList: React.FC<SongListProps> = ({ songs, className = ' ', style }) => {
	const navigate = useNavigate();
	const usePlayer = usePlayerManager();

	useEffect(() => {
		getLikeList();
	}, []);

	const [currentID, setCurrentID] = useState<number>(-1);

	const { likeSongs } = useUserStore();
	const isLikeSong = (songId: number): boolean => {
		return likeSongs.ids?.includes(songId) ?? false;
	};

	const handleLike = (songId: number) => {
		toLikeSong(songId, !isLikeSong(songId));
	};

	const play = (id: number, name: string, cover: string, artist: Artist[]) => {
		// 获取歌手名字列表
		const artistNames: string[] = artist.map((a) => a.name);
		usePlayer.addToPlaylist({
			index: usePlayer.playlist.count,
			id,
			name,
			cover,
			source: -2, // -2 表示搜索结果
			artists: artistNames,
		});
		usePlayer.setCurrentSong(id, true);
	};

	// Effect莫名其妙侦测不到usePlayer.currentSong.id的变化，所以用setInterval来模拟
	// 测你🐎的React, 其他组件就没问题
	// TODO: 修复这个问题
	const interval = setInterval(() => {
		setCurrentID(usePlayer.currentSong.id);
	}, 250);

	interval;

	useEffect(() => {
		return () => {
			clearInterval(interval);
		};
	}, []);

	const menuItems = function (song: any): MenuItem[] {
		return [
			{
				label: <p>播放</p>,
				onClick: () => play(song.id, song.name, song.al.picUrl, song.ar),
			},
			{
				label: <p>评论</p>,
				onClick: () => navigate('/comment/song/' + song.id),
			},
			{
				label: <p>打开链接</p>,
				onClick: () => openUrl(`https://music.163.com/#/song?id=${song.id}`),
			},
		];
	};

	return (
		<div className={styles.song + ' ' + className} style={style}>
			<div className={styles.song__header}>
				<h2 className={styles.song__header__name}>标题</h2>
				<h2 className={styles.song__header__album}>专辑</h2>
				<h2 className={styles.song__header__operator}>操作</h2>
				<h2 className={styles.song__header__duration}>时长</h2>
			</div>
			{songs.map((song) => (
				<ContextMenu
					className={
						styles.song__item + ' ' + (currentID === song.id ? styles.active : '')
					}
					data-context-data={song}
					items={menuItems(song)}
					key={song.id}
					onClick={() => play(song.id, song.name, song.al.picUrl, song.ar)}
				>
					<div className={styles.song__item__title}>
						<LazyImage
							className={styles.song__item__title__cover}
							src={song.al.picUrl}
							alt={song.name}
						/>
						<div className={styles.song__item__title__info}>
							<h3>{song.name}</h3>
							<p>{song.ar.map((artist: any) => artist.name).join(' / ')}</p>
						</div>
					</div>
					<div className={styles.song__item__album}>
						<h3 onClick={() => navigate(`/album/${song.al.id}`)}>{song.al.name}</h3>
					</div>
					<div className={styles.song__item__operation}>
						<span
							className={
								isLikeSong(song.id)
									? 'i-solar-heart-broken-line-duotone'
									: 'i-solar-heart-angle-line-duotone'
							}
							onClick={() => handleLike(song.id)}
						/>
					</div>
					<div className={styles.song__item__duration}>
						{song.dt / 1000 / 60 < 10 ? '0' : ''}
						{Math.floor(song.dt / 1000 / 60)}:{(song.dt / 1000) % 60 < 10 ? '0' : ''}
						{Math.floor((song.dt / 1000) % 60)}
					</div>
				</ContextMenu>
			))}
		</div>
	);
};

export default SongList;
