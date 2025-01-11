import { DefaultSearchResult, HotSearchResult } from '../models/main';
import request from '../utils/request';

const { get } = request;

export async function getHotSearch(): Promise<HotSearchResult['data'] | null> {
	const response = (await get('search/hot/detail')).data as HotSearchResult;
	if (response.code === 200) {
		console.debug('🌐 Get hot search result successfully: ', response);
		return response.data;
	}
	console.error('🌐 Cannot get hot search result');
	return null;
}

export async function getSuggestSearch(keyword: string): Promise<any> {}

export async function getDefaultKey(): Promise<DefaultSearchResult['data'] | null> {
	const response = (await get('search/default')).data as DefaultSearchResult;
	if (response.code === 200) {
		console.debug('🌐 Get default search keyword successfully: ', response);
		return response.data;
	}
	console.error('🌐 Cannot get default search keyword');
	return null;
}
