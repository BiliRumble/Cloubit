import React, { createContext, useContext, useMemo } from 'react';
import PlayerManager from '../managers/PlayerManager';

// 创建 Context
const PlayerContext = createContext<PlayerManager | null>(null);

// 创建一个 Provider 组件
export const PlayerProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
	const playerManager = useMemo(() => {
		if (location.pathname.startsWith('/windows/')) {
			return null;
		}
		return PlayerManager.getInstance();
	}, [location.pathname]);

	return <PlayerContext.Provider value={playerManager}>{children}</PlayerContext.Provider>;
};

// 自定义 Hook 来使用 Context
export const usePlayerManager = () => {
	const context = useContext(PlayerContext);
	if (!context) {
		throw new Error('usePlayer must be used within a PlayerProvider');
	}
	return context;
};
