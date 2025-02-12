import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { getAlbumDetail } from '../../apis/song';
import Chip from '../../components/atoms/Chip';
import SongList from '../../components/organisms/SongList';
import { usePlayerManager } from '../../context/PlayerContext';
import styles from './Album.module.scss';

const Album = () => {
	const { id } = useParams<{ id: string }>();
	const navigate = useNavigate();
	const usePlayer = usePlayerManager();

	const [albumTracks, setAlbumTracks] = useState<any>([]);
	const [albumInfo, setAlbumInfo] = useState<any>([]);
	const [filteredTracks, setFilteredTracks] = useState<any[]>([]);
	const [loading, setLoading] = useState<boolean>(true);
	const [searchKeyword, setSearchKeyword] = useState<string>('');

	useEffect(() => {
		if (!id) navigate('/');
		getAlbumDetail(id as unknown as number).then((res) => {
			res.songs = res.songs.map((song: any) => {
				song.al.picUrl = res.album.picUrl;
				return song;
			});
			setAlbumTracks(res.songs);
			setAlbumInfo(res.album);
			setFilteredTracks(res.songs); // 初始时显示所有歌曲
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
				cover: albumInfo.picUrl,
				artists: track.ar.map((a: any) => a.name),
				source: albumInfo.id,
			});
			index++;
			if (!isPlayed)
				setTimeout(() => {
					usePlayer.setCurrentSong(track.id, true);
				}, 300);
			isPlayed = true;
		});
	};

	// 搜索功能
	const handleSearch = (e: React.ChangeEvent<HTMLInputElement>) => {
		const keyword = e.target.value;
		setSearchKeyword(keyword);

		if (keyword == '') {
			setFilteredTracks(albumTracks);
		} else {
			const filtered = albumTracks.filter(
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
				<div className={styles.album}>
					<div className={styles.album__header}>
						<img src={albumInfo?.picUrl} alt="" />
						<div className={styles.album__header__info}>
							<div className={styles.album__header__info__data}>
								<h1
									className={styles.album__header__info__data__title}
									title={albumInfo?.name}
								>
									{albumInfo?.name}
								</h1>
								<h3
									className={styles.album__header__info__data__description}
									title={albumInfo?.description}
								>
									{albumInfo?.description}
								</h3>
								<div className={styles.album__header__info__data__details}>
									<span
										className={
											styles.album__header__info__data__details__author
										}
									>
										<i className="i-solar-user-linear" />
										{albumInfo?.company}
									</span>
									<span
										className={styles.album__header__info__data__details__time}
									>
										<i className="i-solar-clock-circle-linear" />
										{new Date(albumInfo?.publishTime).toLocaleDateString()}
									</span>
								</div>
								{albumInfo?.tags?.length > 0 && (
									<div
										className={styles.album__header__info__data__details__tags}
									>
										{albumInfo?.tags?.map((tag: string) => (
											<Chip children={tag} />
										))}
									</div>
								)}
							</div>
							<div className={styles.album__header__info__operator}>
								<button
									className={
										styles.album__header__info__operator__play +
										' i-solar-play-line-duotone'
									}
									onClick={() => setPlaylist(albumTracks)}
								/>
								<button
									className={
										styles.album__header__info__operator__collect +
										' i-solar-heart-angle-line-duotone'
									}
								/>
								<div className={styles.album__header__info__operator__search}>
									<input
										type="text"
										placeholder="模糊搜索"
										className={
											styles.album__header__info__operator__search__input
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

export default Album;
