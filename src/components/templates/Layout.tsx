import { memo, useEffect, useState } from 'react';
import { Outlet } from 'react-router-dom';
import Login from '../../pages/modals/Login/Login';
import Settings from '../../pages/modals/Settings/Settings';
import Modal from '../numerator/Modal';
import Navbar from './Navbar';
import PlayBar from './Playbar';
import Sidebar from './Sidebar';

const Layout: React.FC = memo(() => {
	const [isLoaded, setIsLoaded] = useState(false);
	const [sidebarWidth, setSidebarWidth] = useState(240);
	const [isSettingModalOpen, setSettingModalOpen] = useState(false);
	const [isLoginModalOpen, setLoginModalOpen] = useState(false);

	useEffect(() => {
		setIsLoaded(true);

		// 从localStorage恢复侧栏宽度
		const savedWidth = localStorage.getItem('sidebarWidth');
		if (savedWidth) {
			const width = parseInt(savedWidth, 10);
			if (width >= 180 && width <= 400) {
				setSidebarWidth(width);
			}
		}
	}, []);

	const handleOpenSettings = () => setSettingModalOpen(true);
	const handleCloseSettings = () => setSettingModalOpen(false);
	const handleOpenLogin = () => setLoginModalOpen(true);
	const handleCloseLogin = () => setLoginModalOpen(false);

	return (
		<main
			className={`flex w-screen h-screen bg-[var(--background-color)] overflow-hidden transition-all duration-100 ${
				isLoaded ? 'opacity-100' : 'opacity-0'
			}`}
		>
			{/* 侧栏容器 */}
			<div className="relative flex-shrink-0" style={{ width: `${sidebarWidth}px` }}>
				<Sidebar
					className="flex flex-col h-full w-full"
					sidebarWidth={sidebarWidth}
					setSidebarWidth={setSidebarWidth}
				/>
			</div>

			{/* 主内容区域 */}
			<div className="flex flex-1 flex-col h-full max-w-full relative">
				<div className="flex flex-1 flex-col h-full m-3 p-4 rounded-12px bg-[var(--second-background-color)] shadow-lg border border-[var(--hr-color)]/20 backdrop-blur-sm overflow-hidden">
					<Navbar
						className="flex items-center justify-center h-12 mb-6 animate-in slide-in-from-top duration-100"
						onOpenSettings={handleOpenSettings}
						onOpenLogin={handleOpenLogin}
					/>
					<main className="flex-1 overflow-y-auto overflow-x-hidden mb-20 animate-in fade-in duration-100 delay-200 [&::-webkit-scrollbar]:w-2 [&::-webkit-scrollbar]:transition-all [&::-webkit-scrollbar]:duration-300 [&::-webkit-scrollbar:hover]:w-2.5 [&::-webkit-scrollbar-track]:bg-transparent [&::-webkit-scrollbar-thumb]:bg-[var(--hr-color)] [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb:hover]:bg-[var(--second-text-color)]">
						<div className="min-h-full">
							<Outlet />
						</div>
					</main>
				</div>
			</div>

			<PlayBar className="fixed bottom-0 left-0 right-0 h-20 z-50 border-t border-[var(--hr-color)]/50 bg-[var(--second-background-color)]/95 backdrop-blur-md shadow-2xl animate-in slide-in-from-bottom duration-100 delay-300" />

			{/* 全局Modal - 在最外层 */}
			<Modal isOpen={isSettingModalOpen} onClose={handleCloseSettings}>
				<Settings />
			</Modal>
			<Modal isOpen={isLoginModalOpen} onClose={handleCloseLogin}>
				<Login />
			</Modal>
		</main>
	);
});

Layout.displayName = 'Layout';

export default Layout;
