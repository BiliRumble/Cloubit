import { useNavigate } from 'react-router-dom';
import { usePlayerManager } from '../../../context/PlayerContext';
import { Artist } from '../../../models/search';
import styles from './SongList.module.scss';

interface SongListProps {
	songs: any[];
	className?: string;
	style?: React.CSSProperties;
}

const SongList: React.FC<SongListProps> = ({ songs, className = '', style }) => {
	const navigate = useNavigate();
	const usePlayer = usePlayerManager();

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

	return (
		<div className={styles.song + ' ' + className} style={style}>
			<div className={styles.song__header}>
				<h2 className={styles.song__header__name}>标题</h2>
				<h2 className={styles.song__header__album}>专辑</h2>
				<h2 className={styles.song__header__operator}>操作</h2>
				<h2 className={styles.song__header__duration}>时长</h2>
			</div>
			{songs.map((song) => (
				<div
					className={styles.song__item}
					key={song.id}
					onClick={() => play(song.id, song.name, song.al.picUrl, song.ar)}
				>
					<div className={styles.song__item__title}>
						<img src={song.al.picUrl} alt={song.name} />
						<div className={styles.song__item__title__info}>
							<h3>{song.name}</h3>
							<p>{song.ar.map((artist: any) => artist.name).join(' / ')}</p>
						</div>
					</div>
					<div className={styles.song__item__album}>
						<h3 onClick={() => navigate(`/album/${song.al.id}`)}>{song.al.name}</h3>
					</div>
					<div className={styles.song__item__operation}>
						<span className={`i-solar-heart-angle-line-duotone`} />
					</div>
					<div className={styles.song__item__duration}>
						{song.dt / 1000 / 60 < 10 ? '0' : ''}
						{Math.floor(song.dt / 1000 / 60)}:{(song.dt / 1000) % 60 < 10 ? '0' : ''}
						{Math.floor((song.dt / 1000) % 60)}
					</div>
				</div>
			))}
		</div>
	);
};

export default SongList;
