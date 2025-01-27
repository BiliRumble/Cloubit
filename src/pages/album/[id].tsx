import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { getAlbumDetail } from '../../apis/song';
import Chip from '../../components/Common/Chip';
import { usePlayerManager } from '../../context/PlayerContext';
import { Artist } from '../../models/search';
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
			setAlbumTracks(res.songs);
			setAlbumInfo(res.album);
			setFilteredTracks(res.songs); // 初始时显示所有歌曲
			setLoading(false);
		});
	}, [id]);

	const play = (id: number, name: string, cover: string, artist: Artist[]) => {
		const artistNames: string[] = artist.map((a) => a.name);
		usePlayer.addToPlaylist({
			index: usePlayer.playlist.count,
			id,
			source: albumInfo.id,
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

		if (keyword === '') {
			setFilteredTracks(albumTracks.tracks);
		} else {
			const filtered = albumTracks.tracks.filter(
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
								<h1 className={styles.album__header__info__data__title}>
									{albumInfo?.name}
								</h1>
								<h3 className={styles.album__header__info__data__description}>
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
										{albumInfo?.tags?.map((tag: string) => <Chip text={tag} />)}
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
					<div className={styles.album__content}>
						<div className={styles.album__content__header}>
							<span className={styles.album__content__header__name}>标题</span>
							<span className={styles.album__content__header__operator}>操作</span>
							<span className={styles.album__content__header__duration}>时长</span>
						</div>
						<div className={styles.album__content__tracks}>
							{filteredTracks?.map((track: any, index: number) => (
								<div
									key={index}
									className={styles.album__content__tracks__track}
									onClick={() =>
										play(track.id, track.name, track.al.picUrl, track.ar)
									}
								>
									<div className={styles.album__content__tracks__track__title}>
										<img src={albumInfo.picUrl} alt={track.name} />
										<div className={styles.album__content__tracks__track__info}>
											<h3>{track.name}</h3>
											<p>
												{track.ar
													.map((artist: any) => artist.name)
													.join(' / ')}
											</p>
										</div>
									</div>
									<div
										className={styles.album__content__tracks__track__operation}
									>
										<span
											className={
												track.subscribed
													? 'i-solar-heart-broken-line-duotone'
													: 'i-solar-heart-angle-line-duotone'
											}
										/>
									</div>
									<div className={styles.album__content__tracks__track__duration}>
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

export default Album;
