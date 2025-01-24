import { create } from 'zustand';
import { persist } from 'zustand/middleware';

/**
 * 系统设置store类型
 */
export interface settingsStoreType {
	theme: 'light' | 'dark' | 'auto';
	setTheme: (value: 'light' | 'dark' | 'auto') => void;

	// 搜索相关
	searchAutoComplete: boolean;
	setSearchAutoComplete: (value: boolean) => void;
	searchHistoryRecord: boolean;
	setSearchHistoryRecord: (value: boolean) => void;

	// 数据请求相关
	realIP: string;
	setRealIP: (value: string) => void;

	// 播放器相关
	autoPlay: boolean;
	setAutoPlay: (value: boolean) => void;
	savePlaySeek: boolean;
	setSavePlaySeek: (value: boolean) => void;
	showLyrics: boolean;
	setShowLyrics: (value: boolean) => void;
	lyricsType: 'raw' | 'translate';
	setLyricsType: (value: 'raw' | 'translate') => void;
	fadeInOut: boolean;
	setFadeInOut: (value: boolean) => void;
	fadeTime: number;
	setFadeTime: (value: number) => void;
	pushToSMTC: boolean;
	setPushToSMTC: (value: boolean) => void;

	// 热键
	enableGlobalShortcut: boolean;
	setEnableGlobalShortcut: (value: boolean) => void;
	playShortcut: string[];
	setPlayShortcut: (value: string[]) => void;
	prevShortcut: string[];
	setPrevShortcut: (value: string[]) => void;
	nextShortcut: string[];
	setNextShortcut: (value: string[]) => void;
	volumeUpShortcut: string[];
	setVolumeUpShortcut: (value: string[]) => void;
	volumeDownShortcut: string[];
	setVolumeDownShortcut: (value: string[]) => void;
}

export const useSettingStore = create(
	persist<settingsStoreType>(
		(set) => ({
			theme: 'auto',
			setTheme: (value) => set(() => ({ theme: value })),
			searchAutoComplete: true,
			setSearchAutoComplete: (value) => set(() => ({ searchAutoComplete: value })),
			searchHistoryRecord: true,
			setSearchHistoryRecord: (value) => set(() => ({ searchHistoryRecord: value })),
			realIP: '116.25.146.177',
			setRealIP: (value) => set(() => ({ realIP: value })),
			autoPlay: false,
			setAutoPlay: (value) => set(() => ({ autoPlay: value })),
			savePlaySeek: true,
			setSavePlaySeek: (value) => set(() => ({ savePlaySeek: value })),
			showLyrics: true,
			setShowLyrics: (value) => set(() => ({ showLyrics: value })),
			lyricsType: 'raw',
			setLyricsType: (value) => set(() => ({ lyricsType: value })),
			fadeInOut: true,
			setFadeInOut: (value) => set(() => ({ fadeInOut: value })),
			fadeTime: 1500,
			setFadeTime: (value) => set(() => ({ fadeTime: value })),
			pushToSMTC: true,
			setPushToSMTC: (value) => set(() => ({ pushToSMTC: value })),
			enableGlobalShortcut: true,
			setEnableGlobalShortcut: (value) => set(() => ({ enableGlobalShortcut: value })),
			playShortcut: ['Ctrl', 'Shift', 'Space'],
			setPlayShortcut: (value) => set(() => ({ playShortcut: value })),
			prevShortcut: ['Ctrl', 'Shift', 'Left'],
			setPrevShortcut: (value) => set(() => ({ prevShortcut: value })),
			nextShortcut: ['Ctrl', 'Shift', 'Right'],
			setNextShortcut: (value) => set(() => ({ nextShortcut: value })),
			volumeUpShortcut: ['Ctrl', 'Shift', 'Up'],
			setVolumeUpShortcut: (value) => set(() => ({ volumeUpShortcut: value })),
			volumeDownShortcut: ['Ctrl', 'Shift', 'Down'],
			setVolumeDownShortcut: (value) => set(() => ({ volumeDownShortcut: value })),
		}),
		{
			name: 'settings-storage',
		}
	)
);
