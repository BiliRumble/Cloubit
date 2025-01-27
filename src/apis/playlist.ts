import request from '../utils/request';

const { get } = request;

export async function likePlaylist(id: number, like: boolean): Promise<boolean> {
	const t = like ? 1 : 0;
	const response = (await get('playlist/subscribe', { t, id, timestamp: Date.now() }))
		.data as any;
	if (response.code === 200) {
		console.debug('🌐 Like Playlist Success: ', response);
		return true;
	}
	console.error('🌐 Like Playlist Failed!');
	return false;
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
