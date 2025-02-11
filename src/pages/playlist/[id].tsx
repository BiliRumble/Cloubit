import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { getPlayListDetail } from '../../apis/playlist';
import { getUserPlaylist } from '../../apis/user';
import Chip from '../../components/common/Chip';
import LazyImage from '../../components/common/LazyImage';
import SongList from '../../components/layout/SongList';
import { usePlayerManager } from '../../context/PlayerContext';
import { toLikePlaylist } from '../../utils/song';
import styles from './Playlist.module.scss';

const Playlist = () => {
	const { id } = useParams<{ id: string }>();
	const navigate = useNavigate();
	const usePlayer = usePlayerManager();

	const [playlistTracks, setPlaylistTracks] = useState<any>([]);
	const [filteredTracks, setFilteredTracks] = useState<any[]>([]);
	const [loading, setLoading] = useState<boolean>(true);
	const [searchKeyword, setSearchKeyword] = useState<string>('');

	const [isLike, setIsLike] = useState<boolean>(false);

	const handleLike = () => {
		setIsLike(!isLike);
		toLikePlaylist(playlistTracks.id, !isLike);
	};

	useEffect(() => {
		if (!id) navigate('/');
		getPlayListDetail(id as unknown as number).then((res) => {
			setPlaylistTracks(res.playlist);
			setFilteredTracks(res.playlist.tracks); // 初始时显示所有歌曲
			getUserPlaylist().then((res) => {
				const likedPlaylistIds = res.playlist.map((playlist: any) => playlist.id);
				setIsLike(likedPlaylistIds.includes(playlistTracks.id));
			});
			setLoading(false);
		});
	}, [id]);

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
						<LazyImage
							src={playlistTracks?.coverImgUrl}
							alt=""
							className={styles.playlist__header__cover}
						/>
						<div className={styles.playlist__header__info}>
							<div className={styles.playlist__header__info__data}>
								<h1
									className={styles.playlist__header__info__data__title}
									title={playlistTracks?.name}
								>
									{playlistTracks?.name}
								</h1>
								<h3
									className={styles.playlist__header__info__data__description}
									title={playlistTracks?.description}
								>
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
											<Chip key={tag} children={tag} />
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
									onClick={() => handleLike()}
									className={
										styles.playlist__header__info__operator__collect +
										' ' +
										(isLike
											? 'i-solar-heart-broken-line-duotone'
											: 'i-solar-heart-angle-line-duotone')
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
					<SongList songs={filteredTracks} />
				</div>
			)}
		</>
	);
};

export default Playlist;
