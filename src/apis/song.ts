import { getSongURLResult } from '../models/main';
import request from '../utils/request';

const { get } = request;

export async function getSongURL(id: number): Promise<getSongURLResult | null> {
	const response = (await get('song/url', { id: id })).data as getSongURLResult;
	if (response.code === 200) {
		console.debug('🌐 Get Song URL Success: ', response);
		return response;
	}
	console.error('🌐 Cannot Get Song URL!');
	return response;
}
