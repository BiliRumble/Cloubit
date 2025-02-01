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
	likePlaylist: number | null;
	setLikePlaylist: (likePlaylist: number | null) => void;
	likeSongs: { timestamp: number; ids: number[] | null };
	setLikeSongs: (likeSongs: { timestamp: number; ids: number[] | null }) => void;
}

export const useUserStore = create(
	persist<userStoreType>(
		(set) => ({
			dailySong: { timestamp: 0, tracks: null },
			setDailySong: (dailySong) => set(() => ({ dailySong })),
			recommendPlaylist: { timestamp: 0, playlists: null },
			setRecommendPlaylist: (recommendPlaylist) => set(() => ({ recommendPlaylist })),
			likePlaylist: null,
			setLikePlaylist: (likePlaylist) => set(() => ({ likePlaylist })),
			likeSongs: { timestamp: 0, ids: null },
			setLikeSongs: (likeSongs) => set(() => ({ likeSongs })),
		}),
		{
			name: 'user-storage',
		}
	)
);
