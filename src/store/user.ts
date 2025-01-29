import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { DailySongsResult, recommendPlaylist } from '../models/song';

/**
 * 系统设置store类型
 */
export interface userStoreType {
	dailySong: { timestamp: number; tracks: DailySongsResult | null };
	setDailySong: (dailySong: { timestamp: number; tracks: DailySongsResult }) => void;
	recommendPlaylist: { timestamp: number; playlists: recommendPlaylist | null };
	setRecommendPlaylist: (recommendPlaylist: {
		timestamp: number;
		playlists: recommendPlaylist;
	}) => void;
	likePlaylist: number;
	setLikePlaylist: (likePlaylist: number) => void;
}

export const useUserStore = create(
	persist<userStoreType>(
		(set) => ({
			dailySong: { timestamp: 0, tracks: null },
			setDailySong: (dailySong) => set(() => ({ dailySong })),
			recommendPlaylist: { timestamp: 0, playlists: null },
			setRecommendPlaylist: (recommendPlaylist) => set(() => ({ recommendPlaylist })),
			likePlaylist: 0,
			setLikePlaylist: (likePlaylist) => set(() => ({ likePlaylist })),
		}),
		{
			name: 'user-storage',
		}
	)
);
