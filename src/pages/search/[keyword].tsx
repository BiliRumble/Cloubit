import { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { getSearchResult } from '../../api/search';
import { SearchResult, searchType } from '../../models/main';
import styles from './Search.module.scss';

const search = () => {
	const [, setSearchResult] = useState<SearchResult['result'] | null>(null);
	const [searchType, setSearchType] = useState<searchType>('song');
	const { keyword } = useParams<{ keyword: string }>();
	const navgigate = useNavigate();

	useEffect(() => {
		if (!keyword) return navgigate('/');
		getSearchResult(keyword, searchType).then((data) => {
			setSearchResult(data);
		});
	}, []);

	return (
		<div className={styles.search}>
			<h1>{keyword} 的搜索结果</h1>
			<div className={styles.search__typeTab}>
				<button
					className={
						styles.search__typeTab__item + ' ' + searchType == 'song'
							? styles.search__typeTab__item__focus
							: ''
					}
					onClick={() => setSearchType('song')}
				>
					歌曲
				</button>
				<button
					className={searchType == 'playlist' ? styles.active : ''}
					onClick={() => setSearchType('playlist')}
				>
					歌单
				</button>
				<button
					className={searchType == 'album' ? styles.active : ''}
					onClick={() => setSearchType('album')}
				>
					专辑
				</button>
				<button
					className={searchType == 'artist' ? styles.active : ''}
					onClick={() => setSearchType('artist')}
				>
					歌手
				</button>
			</div>
			<div className={styles.searchResult}>
				{}
			</div>
		</div>
	);
};

export default search;
