import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getDefaultKey, getHotSearch } from '../../../api/search';
import { DefaultSearchResult, HotSearchResult } from '../../../models/main';
import { useSettingStore } from '../../../store/setting';
import Chip from '../../Chip';
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
	const [hotSearch, setHotSearch] = useState<HotSearchResult['data'] | null>([]);
	const [defaultSearch, setDefaultSearch] = useState<DefaultSearchResult['data'] | null>([]);
	const settingStore = useSettingStore();

	const navigate = useNavigate();

	const updateHistory = (newHistory: HistoryItem[]) => {
		localStorage.setItem('history', JSON.stringify(newHistory));
		setHistory(newHistory);
	};

	const clearHistory = () => {
		localStorage.removeItem('history');
		setHistory([]);
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
		getDefaultKey().then((data) => {
			setDefaultSearch(data);
		});
	}, []);

	return (
		<div className={styles.search + ' ' + className}>
			<input
				className={styles.search__input + ` ${isFocus ? styles.search__input__focus : ''}`}
				type="search"
				placeholder={settingStore.searchShowHot ? defaultSearch?.realkeyword || '' : '搜索'}
				onFocus={() => setIsFocus(true)}
				onKeyDown={(e) => {
					if (e.key === 'Enter' && inputValue !== '' && isFocus) {
						e.preventDefault();
						if (!history.some((item) => item.name === inputValue))
							updateHistory([...history, { time: Date.now(), name: inputValue }]);
						navigate(`/search/${inputValue}`);
						setIsFocus(false);
						(e.target as HTMLInputElement).blur();
					} else if (
						e.key === 'Enter' &&
						inputValue === '' &&
						settingStore.searchShowHot &&
						isFocus
					) {
						e.preventDefault();
						if (!history.some((item) => item.name === defaultSearch?.realkeyword))
							updateHistory([
								...history,
								{ time: Date.now(), name: defaultSearch?.realkeyword || '' },
							]);
						navigate(`/search/${defaultSearch?.realkeyword || ''}`);
						setIsFocus(false);
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
											onClick={() => {
												if (
													!history.some(
														(hitem) => hitem.name === item.searchWord
													)
												)
													updateHistory([
														...history,
														{ time: Date.now(), name: item.searchWord },
													]);
												navigate(`/search/${item.searchWord}`);
												setIsFocus(false);
											}}
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
			</div>
		</div>
	);
};

export default Search;
