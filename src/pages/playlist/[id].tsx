import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { getPlayListDetail } from '../../apis/song';
import Chip from '../../components/Common/Chip';
import { usePlayerManager } from '../../context/PlayerContext';
import { Artist } from '../../models/search';
import styles from './Playlist.module.scss';

const Playlist = () => {
	const { id } = useParams<{ id: string }>();
	const navigate = useNavigate();
	const usePlayer = usePlayerManager();

	const [playlistTracks, setPlaylistTracks] = useState<any>([]);
	const [filteredTracks, setFilteredTracks] = useState<any[]>([]);
	const [loading, setLoading] = useState<boolean>(true);
	const [searchKeyword, setSearchKeyword] = useState<string>('');

	useEffect(() => {
		if (!id) navigate('/');
		getPlayListDetail(id as unknown as number).then((res) => {
			setPlaylistTracks(res.playlist);
			setFilteredTracks(res.playlist.tracks); // 初始时显示所有歌曲
			setLoading(false);
		});
	}, [id]);

	const play = (id: number, name: string, cover: string, artist: Artist[]) => {
		const artistNames: string[] = artist.map((a) => a.name);
		usePlayer.addToPlaylist({
			index: usePlayer.playlist.count,
			id,
			name,
			cover,
			artists: artistNames,
			source: playlistTracks.id,
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
				source: playlistTracks.id,
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

	// 搜索功能
	const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
		const keyword = e.target.value;
		setSearchKeyword(keyword);

		if (keyword === '') {
			setFilteredTracks(playlistTracks.tracks);
		} else {
			const filtered = playlistTracks.tracks.filter(
				(track: any) =>
					track.name.toLowerCase().includes(keyword.toLowerCase()) ||
					track.ar.some((artist: any) =>
						artist.name.toLowerCase().includes(keyword.toLowerCase())
					)
			);
			setFilteredTracks(filtered);
		}
	};

	return (
		<>
			{!loading && (
				<div className={styles.playlist}>
					<div className={styles.playlist__header}>
						<img src={playlistTracks?.coverImgUrl} alt="" />
						<div className={styles.playlist__header__info}>
							<div className={styles.playlist__header__info__data}>
								<h1 className={styles.playlist__header__info__data__title}>
									{playlistTracks?.name}
								</h1>
								<h3 className={styles.playlist__header__info__data__description}>
									{playlistTracks?.description}
								</h3>
								<div className={styles.playlist__header__info__data__details}>
									<span
										className={
											styles.playlist__header__info__data__details__author
										}
									>
										<i className="i-solar-user-linear" />
										{playlistTracks?.creator?.nickname}
									</span>
									<span
										className={
											styles.playlist__header__info__data__details__time
										}
									>
										<i className="i-solar-clock-circle-linear" />
										{new Date(playlistTracks?.createTime).toLocaleDateString()}
									</span>
								</div>
								{playlistTracks?.tags?.length > 0 && (
									<div
										className={
											styles.playlist__header__info__data__details__tags
										}
									>
										{playlistTracks?.tags?.map((tag: string) => (
											<Chip text={tag} />
										))}
									</div>
								)}
							</div>
							<div className={styles.playlist__header__info__operator}>
								<button
									className={
										styles.playlist__header__info__operator__play +
										' i-solar-play-line-duotone'
									}
									onClick={() => setPlaylist(playlistTracks?.tracks)}
								/>
								<button
									className={
										styles.playlist__header__info__operator__collect +
										' i-solar-heart-angle-line-duotone'
									}
								/>
								<div className={styles.playlist__header__info__operator__search}>
									<input
										type="text"
										placeholder="模糊搜索"
										className={
											styles.playlist__header__info__operator__search__input
										}
										value={searchKeyword}
										onChange={handleSearch}
									/>
								</div>
							</div>
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
							{filteredTracks?.map((track: any, index: number) => (
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
