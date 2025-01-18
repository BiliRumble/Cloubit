import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { PlayList, PlayListItem } from '../models/main';

/**
 * 播放器store类型
 */
export interface playerStoreType {
	playlist: PlayList;
	currentSong: PlayListItem;
	seek: number;
	duration: number;
	volume: number;
	mode: 'list' | 'single' | 'random';
	muted: boolean;
}

export const usePlayerStore = create(
	persist<playerStoreType>(
		(set) => ({
			playlist: {
				count: 0,
				data: [],
			},
			currentSong: {
				index: -1,
				id: 0,
				name: '暂无歌曲',
			},
			seek: 0,
			duration: 0,
			volume: 0.5,
			mode: 'list',
			muted: false,
			setPlaylist: (playlist: PlayList) => set({ playlist }),
			setCurrentSong: (song: PlayListItem) => set({ currentSong: song }),
			setSeek: (seek: number) => set({ seek }),
			setDuration: (duration: number) => set({ duration }),
			setVolume: (volume: number) => set({ volume }),
			setMode: (mode: 'list' | 'single' | 'random') => set({ mode }),
			setMuted: (muted: boolean) => set({ muted }),
		}),
		{
			name: 'player-storage',
		}
	)
);
