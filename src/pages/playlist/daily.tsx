import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getUserDailySongs } from '../../apis/user';
import { usePlayerManager } from '../../context/PlayerContext';
import { Artist } from '../../models/search';
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

	const play = (id: number, name: string, cover: string, artist: Artist[]) => {
		const artistNames: string[] = artist.map((a) => a.name);
		usePlayer.addToPlaylist({
			index: usePlayer.playlist.count,
			id,
			name,
			cover,
			artists: artistNames,
		});
		usePlayer.setCurrentSong(id, true);
	};

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
					<div className={styles.playlist__content}>
						<div className={styles.playlist__content__header}>
							<span className={styles.playlist__content__header__name}>标题</span>
							<span className={styles.playlist__content__header__album}>专辑</span>
							<span className={styles.playlist__content__header__operator}>操作</span>
							<span className={styles.playlist__content__header__duration}>时长</span>
						</div>
						<div className={styles.playlist__content__tracks}>
							{playlistTracks?.map((track: any, index: number) => (
								<div
									key={index}
									className={styles.playlist__content__tracks__track}
									onClick={() =>
										play(track.id, track.name, track.al.picUrl, track.ar)
									}
								>
									<div className={styles.playlist__content__tracks__track__title}>
										<img src={track.al.picUrl} alt={track.name} />
										<div
											className={
												styles.playlist__content__tracks__track__info
											}
										>
											<h3>{track.name}</h3>
											<p>
												{track.ar
													.map((artist: any) => artist.name)
													.join(' / ')}
											</p>
										</div>
									</div>
									<div className={styles.playlist__content__tracks__track__album}>
										<h3 onClick={() => navigate(`/album/${track.al.id}`)}>
											{track.al.name}
										</h3>
									</div>
									<div
										className={
											styles.playlist__content__tracks__track__operation
										}
									>
										<span
											className={
												track.subscribed
													? 'i-solar-heart-broken-line-duotone'
													: 'i-solar-heart-angle-line-duotone'
											}
										/>
									</div>
									<div
										className={
											styles.playlist__content__tracks__track__duration
										}
									>
										{track.dt / 1000 / 60 < 10 ? '0' : ''}
										{Math.floor(track.dt / 1000 / 60)}:
										{(track.dt / 1000) % 60 < 10 ? '0' : ''}
										{Math.floor((track.dt / 1000) % 60)}
									</div>
								</div>
							))}
						</div>
					</div>
				</div>
			)}
		</>
	);
};

export default Playlist;
