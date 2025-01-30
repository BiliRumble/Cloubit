import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getUserDailySongs } from '../../apis/user';
import SongList from '../../components/Common/SongList';
import { usePlayerManager } from '../../context/PlayerContext';
import { DailySongsResult } from '../../models/song';
import { useUserStore } from '../../store/user';
import styles from './Playlist.module.scss';

const Playlist = () => {
	const navigate = useNavigate();
	const usePlayer = usePlayerManager();

	const [data, setData] = useState<DailySongsResult['data'] | null>(null);
	const [loading, setLoading] = useState(true);
	const playlistTracks = data?.dailySongs;

	useEffect(() => {
		getUserDailySongs().then((res) => {
			if (!res) return navigate('/');
			setData(res.data);
			setLoading(false);
		});
	}, []);

	const setPlaylist = async (tracks: any) => {
		usePlayer.clearPlaylist();
		let index = 0;
		let isPlayed = false;
		await tracks.forEach((track: any) => {
			usePlayer.addToPlaylist({
				index,
				id: track.id,
				name: track.name,
				cover: track.al.picUrl,
				source: -1, // 每日推荐
				artists: track.ar.map((a: any) => a.name),
			});
			index++;
			if (!isPlayed)
				setTimeout(() => {
					usePlayer.setCurrentSong(track.id, true);
				}, 300);
			isPlayed = true;
		});
		console.debug('playlist', usePlayer.playlist);
	};

	return (
		<>
			{!loading && (
				<div className={styles.playlist}>
					<div className={styles.playlist__dailyHeader}>
						<h1>每日推荐</h1>
						<p>
							根据你的口味生成 · 更新于
							{new Date(
								useUserStore.getState().dailySong.timestamp
							).toLocaleTimeString()}
						</p>
						<div className={styles.playlist__dailyHeader__operator}>
							<button
								className={styles.playlist__dailyHeader__operator__play}
								onClick={() => setPlaylist(playlistTracks)}
							>
								<i className="i-solar-play-line-duotone" />
								播放全部
							</button>
						</div>
					</div>
					{playlistTracks && <SongList songs={playlistTracks} />}
				</div>
			)}
		</>
	);
};

export default Playlist;
