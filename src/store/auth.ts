import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { UserAccountResult } from '../models/user';

/**
 * 系统设置store类型
 */
export interface authStoreType {
	isLogin: boolean;
	setIsLogin: (isLogin: boolean) => void;
	cookie: { [key: string]: string } | null;
	setCookie: (cookie: { [key: string]: string } | null) => void;
	userData: UserAccountResult | null;
	setUserData: (userData: UserAccountResult | null) => void;
}

export const useAuthStore = create(
	persist<authStoreType>(
		(set) => ({
			isLogin: false,
			setIsLogin: (isLogin) => set({ isLogin }),
			cookie: null,
			setCookie: (cookie) => set({ cookie }),
			userData: null,
			setUserData: (userData) => set({ userData }),
		}),
		{
			name: 'auth-storage',
		}
	)
);
