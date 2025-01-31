import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { DailySongsResult, recommendPlaylist } from '../models/song';

/**
 * 用户数据store类型
 */
export interface userStoreType {
	dailySong: { timestamp: number; tracks: DailySongsResult | null };
	setDailySong: (dailySong: { timestamp: number; tracks: DailySongsResult | null }) => void;
	recommendPlaylist: { timestamp: number; playlists: recommendPlaylist | null };
	setRecommendPlaylist: (recommendPlaylist: {
		timestamp: number;
		playlists: recommendPlaylist | null;
	}) => void;
	likePlaylist: number;
	setLikePlaylist: (likePlaylist: number[] | null) => void;
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
