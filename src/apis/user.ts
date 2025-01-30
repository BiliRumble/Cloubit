import idMeta from '../assets/data/idMeta.json';
import { DailySongsResult, recommendPlaylist } from '../models/song';
import { UserAccountResult, UserDetailResult } from '../models/user';
import { useAuthStore } from '../store/auth';
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
): Promise<DailySongsResult | recommendPlaylist | null> {
	const response = (await get('/recommend/' + type, { timestamp: Date.now() })).data as
		| DailySongsResult
		| recommendPlaylist;
	if (response.code === 200) {
		console.debug('🌐 Get User Daily Record Success: ', response);
		if (type === 'songs') return response as DailySongsResult;
		return response as recommendPlaylist;
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
	if (response?.code === 200) {
		useUserStore
			.getState()
			.setDailySong({ timestamp: Date.now(), tracks: response as DailySongsResult });
		console.debug('🌐 Get User Daily Songs Success: ', response);
		return response as DailySongsResult;
	}
	return null;
}

// 推荐歌单的封装
export async function getUserDailyPlaylist(limit?: number): Promise<recommendPlaylist | null> {
	const lastRecommendPlaylist = useUserStore.getState().recommendPlaylist;
	if (lastRecommendPlaylist.timestamp + 2 * 60 * 60 * 1000 > Date.now()) {
		console.debug('🌐 Get User Daily Resource From Cache: ', lastRecommendPlaylist);
		return lastRecommendPlaylist?.playlists;
	}
	const response = (await get('personalized/playlist', { limit })).data as recommendPlaylist;
	if (response.code == 200) {
		useUserStore.getState().setRecommendPlaylist({
			timestamp: Date.now(),
			playlists: response as recommendPlaylist,
		});
		return response as recommendPlaylist;
	}
	return null;
}

export async function getUserPlaylist(
	id: number = useAuthStore.getState().userData?.account.id as number
): Promise<any> {
	if (!id) return null;
	const response = (
		await get('user/playlist', {
			timestamp: Date.now(),
			uid: id,
		})
	).data as any;
	if (response.code === 200) {
		console.debug('🌐 Get User Playlist Success: ', response);
		return response;
	}
	console.error('🌐 Get User Playlist Failed!');
	return null;
}

export async function getRadarPlaylist() {
	const allRadar = idMeta.radarPlaylist.map(async (playlist) => {
		return (
			await get('/playlist/detail', {
				id: playlist.id,
			})
		).data as any;
	});
	const result = await Promise.allSettled(allRadar);
	return result.map((res: any) => res?.value.playlist);
}

export async function getLikeList(
	id: number = useAuthStore.getState().userData?.account.id as number
) {
	const lastLikeList = useUserStore.getState().userLikeData;
	if (lastLikeList.timestamp + 5 * 1000 > Date.now()) {
		console.debug('🌐 Get User Daily Resource From Cache: ', lastLikeList);
		return lastLikeList?.likelist;
	}
	console.debug(lastLikeList);
	const response = (
		await get('likelist', {
			timestamp: Date.now(),
			uid: id,
		})
	).data as any;
	if (response.code === 200) {
		useUserStore.getState().setUserLikeData({ timestamp: Date.now(), likelist: response.ids });
		return response;
	}
	console.error('🌐 Get User Like List Failed!');
	return null;
}

export async function scrobble(id: number, sourceId: number, time: number): Promise<boolean> {
	const response = (
		await get('scrobble', {
			id,
			sourceid: sourceId,
			time,
			timestamp: Date.now(),
		})
	).data as any;
	if (response.code === 200) {
		console.debug('🌐 Scrobble Success: ', response);
		return true;
	}
	console.error('🌐 Scrobble Failed!');
	return false;
}
