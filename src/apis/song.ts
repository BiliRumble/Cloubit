import { invoke } from '@tauri-apps/api/core';
import { Lyric, Song, SongDetail } from '../models/song';
import request from '../utils/request';

const { get } = request;

/**
 * 获取歌曲详情
 *
 * @param {number} id 歌曲id
 * @returns {Promise<Song>}
 */
export async function getSongDetail(id: number): Promise<Song[] | null> {
	const response = (await get('song/detail', { ids: id })).data as SongDetail;
	if (response.code === 200) {
		console.debug('🌐 Get Song Detail Success: ', response);
		return response.songs;
	}
	console.error('🌐 Cannot Get Song Detail!');
	return null;
}

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
export async function getSongURL(id: number): Promise<any> {
	const response = await invoke('api_get_song_url', { id: id });
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

export async function getAlbumDetail(id: number): Promise<any> {
	const response = (await get('album', { id: id })).data as any;
	if (response.code === 200) {
		console.debug('🌐 Get Album Detail Success: ', response);
		return response;
	}
	console.error('🌐 Cannot Get Album Detail!');
	return null;
}
