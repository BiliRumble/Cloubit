import {
	DefaultSearchResult,
	HotSearchResult,
	SearchResult,
	searchType,
	SuggestSearchResult,
} from '../models/search';
import request from '../utils/request';

const { get, post } = request;

/**
 * 获取热搜列表
 *
 * @returns HotSearchResult | null
 */
export async function getHotSearch(): Promise<HotSearchResult['data'] | null> {
	const response = (await get('search/hot/detail')).data as HotSearchResult;
	if (response.code === 200) {
		console.debug('🌐 Get hot search result successfully: ', response);
		return response.data;
	}
	console.error('🌐 Cannot get hot search result');
	return null;
}

/**
 * 获取搜索建议
 *
 * @param keyword 关键词
 * @returns SuggestSearchResult['result'] | null
 */
export async function getSuggestSearch(
	keyword: string
): Promise<SuggestSearchResult['result'] | null> {
	if (!keyword) return null;
	const response = (await post('search/suggest?keywords=' + keyword)).data as SuggestSearchResult;
	if (response.code === 200) {
		console.debug('🌐 Get suggest search result successfully: ', response);
		return response.result;
	}
	console.error('🌐 Cannot get suggest search result');
	return null;
}

/**
 * 获取默认搜索关键词
 *
 * @returns DefaultSearchResult['data'] | null
 */
export async function getDefaultKey(): Promise<DefaultSearchResult['data'] | null> {
	const response = (await get('search/default')).data as DefaultSearchResult;
	if (response.code === 200) {
		console.debug('🌐 Get default search keyword successfully: ', response);
		return response.data;
	}
	console.error('🌐 Cannot get default search keyword');
	return null;
}

/**
 * 获取搜索结果
 *
 * @returns SearchResult['result'] | null
 * */
export async function getSearchResult(
	keyWord: string,
	type: searchType = 'song',
	offset: number = 0,
	limit: number = 50
): Promise<SearchResult['result'] | null> {
	if (!keyWord) return null;
	const typeMap = {
		song: 1,
		album: 10,
		playlist: 1000,
		artist: 100,
	};
	const response = (
		await get('cloudsearch', {
			keywords: keyWord,
			limit,
			offset,
			type: typeMap[type],
		})
	).data as SearchResult;
	if (response.code === 200) {
		console.debug('🌐 Get search result successfully: ', response);
		return response.result;
	}
	console.error('🌐 Cannot get search result');
	return null;

	// if (!keyWord) return null;
	// const typeMap = {
	// 	song: 1,
	// 	album: 10,
	// 	playlist: 1000,
	// 	artist: 100,
	// };
	// const response = (await invoke('api_search', {
	// 	keyword: keyWord,
	// 	types: typeMap[type],
	// 	limit: limit,
	// 	offset: offset,
	// })) as MusicSearchResponse;
	// if (response) {
	// 	console.debug('🌐 Get search result successfully: ', response.result);
	// 	return response.result;
	// }
	// console.error('🌐 Cannot get search result');
	// return null;
}
