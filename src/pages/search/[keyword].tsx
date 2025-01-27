import { debounce } from 'lodash-es';
import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { getSearchResult } from '../../apis/search';
import notFoundImg from '../../assets/nodata.png';
import Card from '../../components/Common/Card';
import { usePlayerManager } from '../../context/PlayerContext';
import { Artist, SearchResult, searchType } from '../../models/search';
import styles from './Search.module.scss';

const search = () => {
	const [searchResult, setSearchResult] = useState<SearchResult['result'] | null>(null);
	const [searchType, setSearchType] = useState<searchType>('song');
	const { keyword } = useParams<{ keyword: string }>();
	const navigate = useNavigate();

	const usePlayer = usePlayerManager();

	useEffect(() => {
		if (!keyword) return navigate('/');
		getSearchResult(keyword, searchType).then((data) => {
			setSearchResult(data);
		});
	}, []);

	useEffect(() => {
		if (!keyword) return navigate('/');
		getSearchResult(keyword, searchType).then((data) => {
			setSearchResult(data);
		});
	}, [keyword, searchType]);

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

	const renderResults = () => {
		if (!searchResult)
			return (
				<div className={styles.centBox}>
					<img src={notFoundImg} alt="No data" />
					<h3>没有找到相关内容</h3>
				</div>
			);

		switch (searchType) {
			case 'song':
				return (
					<div className={styles.search__result__song}>
						<div className={styles.search__result__song__header}>
							<h2 className={styles.search__result__song__header__name}>标题</h2>
							<h2 className={styles.search__result__song__header__album}>专辑</h2>
							<h2 className={styles.search__result__song__header__operator}>操作</h2>
							<h2 className={styles.search__result__song__header__duration}>时长</h2>
						</div>
						{searchResult.songs?.map((song) => (
							<div
								className={styles.search__result__song__item}
								key={song.id}
								onClick={() =>
									debounce(
										() => play(song.id, song.name, song.al.picUrl, song.ar),
										300
									)()
								}
							>
								<div className={styles.search__result__song__item__title}>
									<img src={song.al.picUrl} alt={song.name} />
									<div className={styles.search__result__song__item__title__info}>
										<h3>{song.name}</h3>
										<p>
											{song.ar.map((artist: any) => artist.name).join(' / ')}
										</p>
									</div>
								</div>
								<div className={styles.search__result__song__item__album}>
									<h3 onClick={() => navigate(`/album/${song.al.id}`)}>
										{song.al.name}
									</h3>
								</div>
								<div className={styles.search__result__song__item__operation}>
									<span className={`i-solar-heart-angle-line-duotone`} />
								</div>
								<div className={styles.search__result__song__item__duration}>
									{song.dt / 1000 / 60 < 10 ? '0' : ''}
									{Math.floor(song.dt / 1000 / 60)}:
									{(song.dt / 1000) % 60 < 10 ? '0' : ''}
									{Math.floor((song.dt / 1000) % 60)}
								</div>
							</div>
						))}
					</div>
				);
			case 'playlist':
				return (
					<div className={styles.search__result__playlist}>
						{searchResult.playlists?.map((playlist) => (
							<Card
								key={playlist.id}
								cover={playlist.coverImgUrl}
								text={playlist.name}
								className={styles.search__result__playlist__card}
								onClick={() => navigate(`/playlist/${playlist.id}`)}
							/>
						))}
					</div>
				);
			case 'album':
				return (
					<div className={styles.search__result__album}>
						{searchResult.albums?.map((album) => (
							<Card
								key={album.id}
								cover={album.picUrl}
								text={album.name}
								className={styles.search__result__album__card}
								onClick={() => navigate(`/album/${album.id}`)}
							/>
						))}
					</div>
				);
			case 'artist':
				return searchResult.artists?.map((artist) => (
					<div key={artist.id}>{artist.name}</div>
				));
			default:
				return (
					<div className={styles.centBox}>
						<img src={notFoundImg} alt="No data" />
						<h3>没有找到相关内容</h3>
					</div>
				);
		}
	};

	return (
		<div className={styles.search}>
			<div className={styles.search__topbar}>
				<h1>{keyword} 的搜索结果</h1>
				<div className={styles.search__topbar__typeTab}>
					<button
						className={
							styles.search__topbar__typeTab__item +
							' ' +
							(searchType === 'song'
								? styles.search__topbar__typeTab__item__focus
								: '')
						}
						onClick={() => setSearchType('song')}
						disabled={searchType === 'song'}
					>
						歌曲
					</button>
					<button
						className={
							styles.search__topbar__typeTab__item +
							' ' +
							(searchType === 'playlist'
								? styles.search__topbar__typeTab__item__focus
								: '')
						}
						onClick={() => setSearchType('playlist')}
					>
						歌单
					</button>
					<button
						className={
							styles.search__topbar__typeTab__item +
							' ' +
							(searchType === 'album'
								? styles.search__topbar__typeTab__item__focus
								: '')
						}
						onClick={() => setSearchType('album')}
					>
						专辑
					</button>
					<button
						className={
							styles.search__topbar__typeTab__item +
							' ' +
							(searchType === 'artist'
								? styles.search__topbar__typeTab__item__focus
								: '')
						}
						onClick={() => setSearchType('artist')}
					>
						歌手
					</button>
				</div>
			</div>
			<div className={styles.search__result}>{renderResults()}</div>
		</div>
	);
};

export default search;
