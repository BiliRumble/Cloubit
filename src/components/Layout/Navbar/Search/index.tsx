import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getDefaultKey, getHotSearch, getSuggestSearch } from '../../../../api/search';
import { DefaultSearchResult, HotSearchResult, SuggestSearchResult } from '../../../../models/main';
import { useSettingStore } from '../../../../store/setting';
import Chip from '../../../Chip';
import styles from './Search.module.scss';

interface PlayBarProps {
	className?: string;
}

interface HistoryItem {
	time: number;
	name: string;
}

const Search: React.FC<PlayBarProps> = ({ className = '' }) => {
	const [isFocus, setIsFocus] = useState(false);
	const [inputValue, setInputValue] = useState('');
	const [history, setHistory] = useState<HistoryItem[]>(() => {
		return JSON.parse(localStorage.getItem('history') || '[]');
	});
	const [hotSearch, setHotSearch] = useState<HotSearchResult['data'] | null>(null);
	const [defaultSearch, setDefaultSearch] = useState<DefaultSearchResult['data'] | null>(null);
	const [suggestSearch, setSuggestSearch] = useState<SuggestSearchResult['result'] | null>(null);
	const [lastCallTime, setLastCallTime] = useState<number | null>(null); // for suggest search debounce
	const settingStore = useSettingStore();

	const navigate = useNavigate();

	const updateHistory = (newHistory: HistoryItem[]) => {
		if (!settingStore.searchHistoryRecord) return;
		localStorage.setItem('history', JSON.stringify(newHistory));
		setHistory(newHistory);
	};

	const clearHistory = () => {
		localStorage.removeItem('history');
		setHistory([]);
	};

	const handleSearch = (name: string) => {
		if (!history.some((item) => item.name === name))
			updateHistory([...history, { time: Date.now(), name: name }]);
		navigate(`/search/${name}`);
		setIsFocus(false);
	};

	useEffect(() => {
		const handleStorageChange = (event: StorageEvent) => {
			if (event.key === 'history') {
				setHistory(JSON.parse(event.newValue || '[]'));
			}
		};

		window.addEventListener('storage', handleStorageChange);

		return () => {
			window.removeEventListener('storage', handleStorageChange);
		};
	}, []);

	useEffect(() => {
		getHotSearch().then((data) => {
			setHotSearch(data);
		});
		// 间隔一分钟获取一次
		const autocompleteInterval = setInterval(() => {
			if (!settingStore.searchAutoComplete) return setDefaultSearch(null);
			getDefaultKey().then((data) => {
				setDefaultSearch(data);
			});
		}, 60000);
		autocompleteInterval;
	}, []);

	useEffect(() => {
		if (!settingStore.searchAutoComplete) return setSuggestSearch(null);
		getDefaultKey().then((data) => {
			setDefaultSearch(data);
		});
	}, [settingStore.searchAutoComplete]);

	useEffect(() => {
		const delayDebounceFn = setTimeout(() => {
			const now = Date.now();
			if (lastCallTime === null || now - lastCallTime > 300) {
				// 300ms间隔
				getSuggestSearch(inputValue).then((data) => {
					setSuggestSearch(data);
				});
				setLastCallTime(now);
			}
		}, 500); // 500ms延迟

		return () => clearTimeout(delayDebounceFn);
	}, [inputValue]);

	return (
		<div className={styles.search + ' ' + className}>
			<input
				className={styles.search__input + ` ${isFocus ? styles.search__input__focus : ''}`}
				type="search"
				placeholder={
					settingStore.searchAutoComplete && defaultSearch !== null
						? defaultSearch?.showKeyword || ''
						: '搜索'
				}
				onFocus={() => setIsFocus(true)}
				onKeyDown={(e) => {
					if (e.key === 'Enter' && inputValue !== '' && isFocus) {
						e.preventDefault();
						handleSearch(inputValue);
						(e.target as HTMLInputElement).blur();
					} else if (
						e.key === 'Enter' &&
						inputValue === '' &&
						settingStore.searchAutoComplete &&
						isFocus
					) {
						e.preventDefault();
						handleSearch(defaultSearch?.realkeyword || '');
						(e.target as HTMLInputElement).blur();
					}
				}}
				onChange={(e) => setInputValue(e.target.value)}
			/>
			<div
				className={styles.search__mask + ` ${isFocus ? styles.search__mask__visible : ''}`}
				onClick={() => setIsFocus(false)}
			/>
			<div
				className={styles.search__card + ` ${isFocus ? styles.search__card__visible : ''}`}
			>
				{inputValue.length == 0 && (
					<div className={styles.search__card__default}>
						{history && history.length > 0 && (
							<div className={styles.search__card__default__history}>
								<div className={styles.search__card__default__history__title}>
									<h3>
										<span className="i-solar-history-linear" />
										历史记录
									</h3>
									<span
										onClick={clearHistory}
										className={
											styles.search__card__default__history__title__clear +
											' i-solar-trash-bin-2-linear'
										}
									/>
								</div>
								<div className={styles.search__card__default__history__list}>
									{history.map((item) => {
										return (
											<Chip
												key={item.time}
												text={item.name}
												onClick={() => {
													navigate(`/search/${item.name}`);
													setIsFocus(false);
												}}
												classNames={
													styles.search__card__default__history__list__item
												}
											/>
										);
									})}
								</div>
							</div>
						)}
						<div className={styles.search__card__default__hot}>
							<h3>
								<span className="i-solar-flame-linear" />
								热门搜索
							</h3>
							<div className={styles.search__card__default__hot__list}>
								{hotSearch?.map((item, index) => {
									return (
										<div
											key={item.score}
											className={
												styles.search__card__default__hot__list__item
											}
											onClick={() => handleSearch(item.searchWord)}
										>
											<span
												className={
													styles.search__card__default__hot__list__item__index
												}
											>
												{index + 1}
											</span>
											<div
												className={
													styles.search__card__default__hot__list__item__data
												}
											>
												<h3>{item.searchWord}</h3>
												<h4>{item.content}</h4>
											</div>
											<div
												className={
													styles.search__card__default__hot__list__item__count
												}
											>
												<h4>
													<span className="i-solar-flame-linear" />{' '}
													{item.score}
												</h4>
											</div>
										</div>
									);
								})}
							</div>
						</div>
					</div>
				)}
				{inputValue.length > 0 && (
					<div className={styles.search__card__suggest}>
						<div className={styles.search__card__suggest__list}>
							<div
								className={styles.search__card__suggest__list__item}
								onClick={() => handleSearch(inputValue)}
							>
								<span
									className={
										styles.search__card__suggest__list__item__icon +
										' i-solar-magnifer-linear'
									}
								/>
								{inputValue}
							</div>
							{/* 单曲 */}
							{suggestSearch?.songs != null && (
								<>
									<h3 className={styles.search__card__suggest__list__title}>
										<span className="i-solar-music-note-4-linear" />
										单曲
									</h3>
									{suggestSearch?.songs?.map((item) => {
										return (
											<div
												key={item.id}
												className={styles.search__card__suggest__list__item}
												onClick={() => handleSearch(item.name)}
											>
												<span
													className={
														styles.search__card__suggest__list__item__name
													}
												>
													{item.name}
												</span>
												<span
													className={
														styles.search__card__suggest__list__item__artist
													}
												>
													{item.artists[0]?.name}
												</span>
											</div>
										);
									})}
								</>
							)}
							{/* 专辑 */}
							{suggestSearch?.albums != null && (
								<>
									<h3 className={styles.search__card__suggest__list__title}>
										<span className="i-solar-vinyl-record-linear" />
										专辑
									</h3>
									{suggestSearch?.albums?.map((item) => {
										return (
											<div
												key={item.id}
												className={styles.search__card__suggest__list__item}
												onClick={() => handleSearch(item.name)}
											>
												<span
													className={
														styles.search__card__suggest__list__item__name
													}
												>
													{item.name}
												</span>
												<span
													className={
														styles.search__card__suggest__list__item__artist
													}
												>
													{item.artist?.name}
												</span>
											</div>
										);
									})}
								</>
							)}
							{/* 歌单 */}
							{suggestSearch?.playlists != null && (
								<>
									<h3 className={styles.search__card__suggest__list__title}>
										<span className="i-solar-playlist-2-linear" />
										歌单
									</h3>
									{suggestSearch?.playlists?.map((item) => {
										return (
											<div
												key={item.id}
												className={styles.search__card__suggest__list__item}
												onClick={() => handleSearch(item.name)}
											>
												{item.name}
											</div>
										);
									})}
								</>
							)}
							{/* 歌手 */}
							{suggestSearch?.artists != null && (
								<>
									<h3 className={styles.search__card__suggest__list__title}>
										<span className="i-solar-users-group-rounded-linear" />
										歌手
									</h3>
									{suggestSearch?.artists?.map((item) => {
										return (
											<div
												key={item.id}
												className={styles.search__card__suggest__list__item}
												onClick={() => handleSearch(item.name)}
											>
												{item.name}
											</div>
										);
									})}
								</>
							)}
						</div>
					</div>
				)}
			</div>
		</div>
	);
};

export default Search;
