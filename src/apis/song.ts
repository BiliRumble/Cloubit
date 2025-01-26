import { getSongURLResult, Lyric } from '../models/song';
import request from '../utils/request';

const { get } = request;

/**
 * 获取可用性
 *
 * @param id boolean
 */
export async function checkSong(id: number): Promise<boolean> {
	const response = (await get('check/music', { id: id })).data as {
		success: boolean;
		message: string;
	};
	// success 始终为 true, 只能通过 message 判断
	if (response.message !== '亲爱的,暂无版权') {
		return true;
	}
	return false;
}

/**
 * 获取歌曲url
 *
 * @param id 歌曲id
 * @returns getSongURLResult | null
 */
export async function getSongURL(id: number): Promise<getSongURLResult | null> {
	const response = (await get('song/url', { id: id })).data as getSongURLResult;
	if (response.code === 200) {
		console.debug('🌐 Get Song URL Success: ', response);
		return response;
	}
	console.error('🌐 Cannot Get Song URL!');
	return response;
}

/**
 * 获取歌词
 *
 * @param id 歌曲id
 * @returns Lyric | null
 */
export async function getLyric(id: number): Promise<Lyric | null> {
	const response = (await get('lyric', { id: id })).data as Lyric;
	if (response.code === 200) {
		console.debug('🌐 Get Lyric Success: ', response);
		return response;
	}
	console.error('🌐 Cannot Get Lyric!');
	return response;
}

export async function getPlayListDetail(id: number): Promise<any> {
	const response = (await get('playlist/detail', { id: id })).data as any;
	if (response.code === 200) {
		console.debug('🌐 Get PlayList Detail Success: ', response);
		return response;
	}
	console.error('🌐 Cannot Get PlayList Detail!');
	return null;
}

export async function getAlbumDetail(id: number): Promise<any> {
	const response = (await get('album', { id: id })).data as any;
	if (response.code === 200) {
		console.debug('🌐 Get Album Detail Success: ', response);
		return response;
	}
	console.error('🌐 Cannot Get Album Detail!');
	return null;
}
