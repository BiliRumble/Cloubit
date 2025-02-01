import { debounce } from 'lodash-es';
import { likePlaylist } from '../apis/playlist';
import { getLikeList, likeSong } from '../apis/user';
import { useAuthStore } from '../store/auth';

export const toLikeSong = debounce(
	async (id: number, like: boolean) => {
		if (!useAuthStore.getState().isLogin) {
			alert('请登录后使用');
			return;
		}
		const response = await likeSong(id, like);
		if (response) {
			getLikeList();
		} else {
			alert('操作失败');
		}
	},
	300,
	{ leading: true, trailing: false }
);

export const toLikePlaylist = debounce(async (id: number, like: boolean) => {
	if (!useAuthStore.getState().isLogin) {
		alert('请登录后使用');
		return;
	}
	const response = await likePlaylist(id, like);
	if (!response) {
		alert('操作失败');
	}
}, 300);
