import { memo, useCallback, useEffect, useMemo, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getDefaultKey, getHotSearch, getSuggestSearch } from '../../apis/search';
import { DefaultSearchResult, HotSearchResult, SuggestSearchResult } from '../../models/search';
import { useSettingStore } from '../../store/setting';
import Chip from '../atoms/Chip';

interface PlayBarProps {
	className?: string;
}

interface HistoryItem {
	time: number;
	name: string;
}

const Search: React.FC<PlayBarProps> = memo(({ className = '' }) => {
	const [isFocus, setIsFocus] = useState(false);
	const [inputValue, setInputValue] = useState('');
	const [history, setHistory] = useState<HistoryItem[]>(() => {
		return JSON.parse(localStorage.getItem('history') || '[]');
	});
	const [hotSearch, setHotSearch] = useState<HotSearchResult['data'] | null>(null);
	const [defaultSearch, setDefaultSearch] = useState<DefaultSearchResult['data'] | null>(null);
	const [suggestSearch, setSuggestSearch] = useState<SuggestSearchResult['result'] | null>(null);
	const settingStore = useSettingStore();

	const navigate = useNavigate();

	const updateHistory = useCallback(
		(newHistory: HistoryItem[]) => {
			if (!settingStore.searchHistoryRecord) return;
			localStorage.setItem('history', JSON.stringify(newHistory));
			setHistory(newHistory);
		},
		[settingStore.searchHistoryRecord]
	);

	const clearHistory = useCallback(() => {
		localStorage.removeItem('history');
		setHistory([]);
	}, []);

	const handleSearch = useCallback(
		(name: string) => {
			if (!history.some((item) => item.name === name))
				updateHistory([...history, { time: Date.now(), name: name }]);
			navigate(`/search/${name}`);
			setIsFocus(false);
		},
		[history, updateHistory, navigate]
	);

	const handleFocus = useCallback(() => setIsFocus(true), []);
	const handleBlur = useCallback(() => setIsFocus(false), []);

	const handleKeyDown = useCallback(
		(e: React.KeyboardEvent<HTMLInputElement>) => {
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
		},
		[inputValue, isFocus, handleSearch, settingStore.searchAutoComplete, defaultSearch]
	);

	const handleInputChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
		setInputValue(e.target.value);
	}, []);

	// 缓存历史记录渲染
	const renderedHistory = useMemo(() => {
		return history.map((item, index) => (
			<Chip
				key={`${item.time}-${index}`}
				onContextMenu={() => {
					const newHistory = history.filter((i) => i.time !== item.time);
					setHistory(newHistory);
					localStorage.setItem('history', JSON.stringify(newHistory));
				}}
				onClick={() => {
					(document.getElementById('searchInput') as HTMLInputElement).value = item.name;
					setInputValue(item.name);
					navigate(`/search/${item.name}`);
					setIsFocus(false);
				}}
				className="mb-2 max-w-[275px]"
			>
				{item.name}
			</Chip>
		));
	}, [history, navigate]);

	// 缓存热门搜索渲染
	const renderedHotSearch = useMemo(() => {
		return hotSearch?.map((item, index) => (
			<div
				key={`${item.score}-${index}`}
				className="flex items-center justify-between mb-2 rounded-8px p-1.5 transition-colors duration-300 w-[275px] cursor-pointer hover:bg-[var(--hr-color)]"
				onClick={() => handleSearch(item.searchWord)}
			>
				<span className="w-7.5 h-7.5 min-w-7.5 font-bold flex items-center justify-center text-1rem mr-2 text-[var(--second-text-color)]">
					{index + 1}
				</span>
				<div className="flex-1 w-full pr-2">
					<h3 className="m-0 text-1rem">{item.searchWord}</h3>
					<h4 className="text-[var(--second-text-color)] text-0.9rem">{item.content}</h4>
				</div>
				<div className="flex flex-row items-center">
					<h4 className="text-0.8rem">
						<span className="i-solar-flame-linear" /> {item.score}
					</h4>
				</div>
			</div>
		));
	}, [hotSearch, handleSearch]);

	useEffect(() => {
		const handleStorageChange = (event: StorageEvent) => {
			if (event.key === 'history') {
				setHistory(JSON.parse(event.newValue || '[]'));
			}
		};

		window.addEventListener('storage', handleStorageChange);
		return () => window.removeEventListener('storage', handleStorageChange);
	}, []);

	useEffect(() => {
		getHotSearch().then(setHotSearch);
		const autocompleteInterval = setInterval(() => {
			if (!settingStore.searchAutoComplete) return setDefaultSearch(null);
			getDefaultKey().then(setDefaultSearch);
		}, 120000);
		return () => clearInterval(autocompleteInterval);
	}, [settingStore.searchAutoComplete]);

	useEffect(() => {
		if (!settingStore.searchAutoComplete) return setSuggestSearch(null);
		getDefaultKey().then(setDefaultSearch);
	}, [settingStore.searchAutoComplete]);

	useEffect(() => {
		getSuggestSearch(inputValue).then(setSuggestSearch);
	}, [inputValue]);

	return (
		<div className={`flex items-center flex-row h-screen ${className}`}>
			<input
				id="searchInput"
				className={`border-none bg-[var(--button-focus-bg-color)] rounded-50px px-4 py-2 fixed z-110 transition-all duration-200 ease-out text-[var(--text-color)] outline-none ${
					isFocus ? 'w-75' : 'w-50'
				} [&::-webkit-search-cancel-button]:h-4 [&::-webkit-search-cancel-button]:w-4 [&::-webkit-search-cancel-button]:cursor-pointer`}
				type="search"
				autoComplete="off"
				placeholder={
					settingStore.searchAutoComplete && defaultSearch !== null
						? defaultSearch?.showKeyword || ''
						: '搜索'
				}
				onFocus={handleFocus}
				onKeyDown={handleKeyDown}
				onChange={handleInputChange}
			/>

			{/* 遮罩层 - 添加平滑动画 */}
			<div
				className={`fixed inset-0 bg-black/50 backdrop-blur-20px z-100 transition-all duration-200 ease-out ${
					isFocus ? 'opacity-100 visible' : 'opacity-0 invisible'
				}`}
				onClick={handleBlur}
			/>

			{/* 弹出框 - 添加平滑动画，与输入框同步 */}
			<div
				className={`w-75 max-h-[75vh] p-2.5 rounded-8px bg-[var(--second-background-color)] fixed top-17.5 z-110 overflow-y-auto shadow-[var(--shadow)] hover:shadow-[var(--shadow-hover)] focus:shadow-[var(--shadow-focus)] transition-all duration-200 ease-out [&::-webkit-scrollbar]:w-1.5 [&::-webkit-scrollbar]:transition-all [&::-webkit-scrollbar]:duration-300 [&::-webkit-scrollbar:hover]:w-2.5 ${
					isFocus
						? 'opacity-100 visible translate-y-0 scale-100'
						: 'opacity-0 invisible -translate-y-2 scale-95'
				}`}
			>
				{inputValue.length === 0 && (
					<div className="flex flex-col items-start flex-nowrap w-full">
						{history && history.length > 0 && (
							<div className="flex flex-col items-start w-full">
								<div className="flex mb-2 flex-row items-center justify-between w-full">
									<h3 className="text-center align-middle flex items-center mb-2">
										<span className="i-solar-history-linear mr-1.6 relative top-0.5" />
										历史记录
									</h3>
									<span
										onClick={clearHistory}
										className="i-solar-trash-bin-2-linear cursor-pointer text-1.25rem text-[var(--text-color)] relative top-0.5 transition-colors duration-300 hover:text-[var(--primary-color)]"
									/>
								</div>
								<div className="flex flex-row flex-wrap items-stretch">
									{renderedHistory}
								</div>
							</div>
						)}

						<div className="flex flex-col items-start w-full">
							<h3 className="text-center align-middle flex items-center mb-2">
								<span className="i-solar-flame-linear mr-1.6 relative top-0.5" />
								热门搜索
							</h3>
							<div className="flex w-full h-full flex-col flex-nowrap items-start">
								{renderedHotSearch}
							</div>
						</div>
					</div>
				)}

				{inputValue.length > 0 && (
					<div className="flex flex-col items-start w-full">
						<div className="flex w-full h-full flex-col flex-nowrap items-start">
							<div
								className="mb-2 rounded-8px p-1.5 transition-colors duration-300 w-[275px] cursor-pointer flex flex-row items-center hover:bg-[var(--hr-color)]"
								onClick={() => handleSearch(inputValue)}
							>
								<span className="i-solar-magnifer-linear relative top-0.5 mr-2" />
								{inputValue}
							</div>

							{suggestSearch?.songs != null && (
								<>
									<h3 className="text-center align-middle flex items-center mb-2">
										<span className="i-solar-music-note-4-linear mr-1.6 relative top-0.5" />
										单曲
									</h3>
									{suggestSearch.songs.map((item, index) => (
										<div
											key={`${item.id}-${index}`}
											className="mb-2 rounded-8px p-1.5 transition-colors duration-300 w-[275px] cursor-pointer flex flex-row items-center hover:bg-[var(--hr-color)]"
											onClick={() => handleSearch(item.name)}
										>
											<span className="text-1rem">{item.name}</span>
											<span className="text-[var(--second-text-color)] before:content-['（'] after:content-['）']">
												{item.artists[0]?.name}
											</span>
										</div>
									))}
								</>
							)}

							{suggestSearch?.albums != null && (
								<>
									<h3 className="text-center align-middle flex items-center mb-2">
										<span className="i-solar-vinyl-record-linear mr-1.6 relative top-0.5" />
										专辑
									</h3>
									{suggestSearch.albums.map((item, index) => (
										<div
											key={`${item.id}-${index}`}
											className="mb-2 rounded-8px p-1.5 transition-colors duration-300 w-[275px] cursor-pointer flex flex-row items-center hover:bg-[var(--hr-color)]"
											onClick={() => handleSearch(item.name)}
										>
											<span className="text-1rem">{item.name}</span>
											<span className="text-[var(--second-text-color)] before:content-['（'] after:content-['）']">
												{item.artist?.name}
											</span>
										</div>
									))}
								</>
							)}

							{suggestSearch?.playlists != null && (
								<>
									<h3 className="text-center align-middle flex items-center mb-2">
										<span className="i-solar-playlist-2-linear mr-1.6 relative top-0.5" />
										歌单
									</h3>
									{suggestSearch.playlists.map((item, index) => (
										<div
											key={`${item.id}-${index}`}
											className="mb-2 rounded-8px p-1.5 transition-colors duration-300 w-[275px] cursor-pointer flex flex-row items-center hover:bg-[var(--hr-color)]"
											onClick={() => handleSearch(item.name)}
										>
											{item.name}
										</div>
									))}
								</>
							)}

							{suggestSearch?.artists != null && (
								<>
									<h3 className="text-center align-middle flex items-center mb-2">
										<span className="i-solar-users-group-rounded-linear mr-1.6 relative top-0.5" />
										歌手
									</h3>
									{suggestSearch.artists.map((item, index) => (
										<div
											key={`${item.id}-${index}`}
											className="mb-2 rounded-8px p-1.5 transition-colors duration-300 w-[275px] cursor-pointer flex flex-row items-center hover:bg-[var(--hr-color)]"
											onClick={() => handleSearch(item.name)}
										>
											{item.name}
										</div>
									))}
								</>
							)}
						</div>
					</div>
				)}
			</div>
		</div>
	);
});

Search.displayName = 'Search';

export default Search;
