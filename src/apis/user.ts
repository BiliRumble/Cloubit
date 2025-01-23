import { DailySongsResult } from '../models/song';
import { UserAccountResult, UserDetailResult } from '../models/user';
import { useUserStore } from '../store/user';
import request from '../utils/request';

const { get } = request;

/**
 * 获取用户账号信息
 *
 * @return UserAccountResult | null 用户账号信息
 */
export async function getUserAccount(): Promise<UserAccountResult | null> {
	const response = (await get('user/account', { timestamp: Date.now() }))
		.data as UserAccountResult;
	if (response.code === 200) {
		console.debug('🌐 Get User Account Info Success: ', response);
		return response;
	}
	console.error('🌐 Get User Account Info Failed!');
	return null;
}

/**
 * 获取用户详细信息
 *
 * @return UserDetailResult | null 用户详细信息
 */
export async function getUserDetail(): Promise<UserDetailResult | null> {
	const response = (await get('user/detail', { timestamp: Date.now() })).data as UserDetailResult;
	if (response.code === 200) {
		console.debug('🌐 Get User Detail Info Success: ', response);
		return response;
	}
	if (response.code === 301) {
		console.debug('🌐 未登录');
	}
	console.error('🌐 Get User Detail Info Failed!');
	return null;
}

/**
 * 获取用户每日歌曲推荐
 *
 * @param type 'songs' | 'resource'
 * @returns DailySongsResult | null
 */
export async function getUserDailyRecord(
	type: 'songs' | 'resource'
): Promise<DailySongsResult | null> {
	const response = (await get('/recommend/' + type, { timestamp: Date.now() }))
		.data as DailySongsResult;
	if (response.code === 200) {
		console.debug('🌐 Get User Daily Record Success: ', response);
		return response;
	}
	console.error('🌐 Get User Daily Record Failed!');
	return null;
}

// 对每日推荐的封装
export async function getUserDailySongs(): Promise<DailySongsResult | null> {
	const lastDailySongs = useUserStore.getState().dailySong;
	if (lastDailySongs.timestamp + 86400000 > Date.now()) {
		console.debug('🌐 Get User Daily Songs From Cache: ', lastDailySongs);
		return useUserStore.getState().dailySong.tracks;
	}
	const response = await getUserDailyRecord('songs');
	if (response) {
		useUserStore.getState().setDailySong({ timestamp: Date.now(), tracks: response });
		return response;
	}
	return null;
}
