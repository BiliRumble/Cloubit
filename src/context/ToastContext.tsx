import React, { createContext, ReactNode, useContext } from 'react';
import { ToastManager } from '../managers/ToastManager';

interface ToastContextProps {
	children: ReactNode;
}

export const ToastContext = createContext<ToastManager | undefined>(undefined);

export const ToastProvider: React.FC<ToastContextProps> = ({ children }) => {
	const manager = new ToastManager();

	return <ToastContext.Provider value={manager}>{children}</ToastContext.Provider>;
};

export const useToast = () => {
	const context = useContext(ToastContext);
	if (!context) {
		throw new Error('useToast must be used within a ToastProvider');
	}
	return context;
};
