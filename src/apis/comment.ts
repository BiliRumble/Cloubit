import { CommentResponse } from '../models/comment';
import request from '../utils/request';

const { get } = request;

export async function getSongComment(id: number, limit: number = 30, offset?: number) {
	const response = (await get('/comment/music', { id, limit, offset, timestamp: Date.now() }))
		.data as CommentResponse;

	if (response.code === 200) {
		console.debug('🌐 Get Song Comments Success: ', response);
		return response;
	}
	console.error('🌐 Cannot Get Song Comments!');
	return null;
}
