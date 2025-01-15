import { search_hot_detail } from './NeteaseCloudMusicApi.js';
import {
	DefaultSearchResult,
	HotSearchResult,
	SearchResult,
	searchType,
	SuggestSearchResult,
} from '../models/main';
import request from '../utils/request';

const { get, post } = request;

export async function getHotSearch(): Promise<HotSearchResult['data'] | null> {
	const response = search_hot_detail();
	if (response.code === 200) {
		console.debug('🌐 Get hot search result successfully: ', response);
		return response.data;
	}
	console.error('🌐 Cannot get hot search result');
	return null;
}

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

export async function getDefaultKey(): Promise<DefaultSearchResult['data'] | null> {
	const response = (await get('search/default')).data as DefaultSearchResult;
	if (response.code === 200) {
		console.debug('🌐 Get default search keyword successfully: ', response);
		return response.data;
	}
	console.error('🌐 Cannot get default search keyword');
	return null;
}

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
		await get(
			`cloudsearch?keywords=${keyWord}&limit=${limit}&offset=${offset}&type=${typeMap[type]}`
		)
	).data as SearchResult;
	if (response.code === 200) {
		console.debug('🌐 Get search result successfully: ', response);
		return response.result;
	}
	console.error('🌐 Cannot get search result');
	return null;
}
