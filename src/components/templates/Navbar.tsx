import { event } from '@tauri-apps/api';
import { Window } from '@tauri-apps/api/window';
import { memo, useCallback, useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { logout } from '../../apis/login';
import { usePlayerManager } from '../../context/PlayerContext';
import { closePip, createPip } from '../../managers/PIPWindowManager';
import { useAuthStore } from '../../store/auth';
import { useUserStore } from '../../store/user';
import { IconButton } from '../atoms/IconButton';
import Avatar from '../numerator/Avatar';
import Popover from '../numerator/Popover';
import Search from './Search';

interface NavbarProps {
	className?: string;
	onOpenSettings: () => void;
	onOpenLogin: () => void;
}

const Navbar: React.FC<NavbarProps> = memo(({ className, onOpenSettings, onOpenLogin }) => {
	const appWindow = new Window('main');
	const [maximized, setMaximized] = useState(false);
	const currentSong = usePlayerManager().currentSong;

	const userRef = useRef<HTMLDivElement>(null);
	const navigate = useNavigate();

	// 缓存认证状态
	const authState = useAuthStore.getState();
	const isLogin = authState.isLogin;
	const userData = authState.userData;

	const toggleMaximize = useCallback(async () => {
		try {
			appWindow.toggleMaximize();
			setMaximized(!maximized);
		} catch (e) {
			console.error(e);
		}
	}, [appWindow, maximized]);

	const changePip = useCallback(async () => {
		if (localStorage.getItem('isPip') == 'true') return closePip();
		createPip();
	}, []);

	const handleBack = useCallback(() => window.history.back(), []);
	const handleForward = useCallback(() => window.history.forward(), []);
	const handleRefresh = useCallback(() => navigate(0), [navigate]);
	const handleRecognize = useCallback(() => navigate('/recognize'), [navigate]);
	const handleMinimize = useCallback(() => appWindow.minimize(), [appWindow]);
	const handleClose = useCallback(() => appWindow.hide(), [appWindow]);

	const handlePip = useCallback(() => {
		changePip();
		setTimeout(() => {
			event.emitTo('main', 'player-update-current-song', currentSong);
		}, 1000);
	}, [changePip, currentSong]);

	const handleLogout = useCallback(() => {
		logout();
		const auth = useAuthStore.getState();
		auth.setIsLogin(false);
		auth.setCookie(null);
		auth.setUserData(null);
		const userStore = useUserStore.getState();
		userStore.setDailySong({ timestamp: 0, tracks: null });
		userStore.setRecommendPlaylist({ timestamp: 0, playlists: null });
		userStore.setLikePlaylist(null);
		window.location.reload();
	}, []);

	return (
		<>
			<header data-tauri-drag-region className={`flex w-full ${className || ''}`.trim()}>
				<div data-tauri-drag-region className="flex-1 flex">
					<div className="flex flex-row flex-nowrap items-stretch mr-4">
						<IconButton
							title="返回"
							icon="i-solar-alt-arrow-left-linear"
							onClick={handleBack}
						/>
						<IconButton
							title="继续"
							icon="i-solar-alt-arrow-right-linear"
							onClick={handleForward}
						/>
						<IconButton
							title="刷新"
							icon="i-solar-restart-linear"
							style={{ fontSize: '1.5rem' }}
							onClick={handleRefresh}
						/>
					</div>
					<Search />
					<IconButton
						title="听歌识曲"
						icon="i-solar-microphone-linear"
						onClick={handleRecognize}
					/>
				</div>

				<div
					data-tauri-drag-region
					className="flex-1 flex justify-end flex-row flex-nowrap items-stretch"
				>
					<button className="bg-transparent border-none text-[var(--text-color)] text-1rem flex items-center justify-center flex-row flex-nowrap">
						{isLogin ? (
							<div
								className="flex items-center justify-center flex-row flex-nowrap border-none bg-transparent text-[var(--text-color)] mr-4"
								ref={userRef}
							>
								<Avatar
									url={userData?.profile.avatarUrl || ''}
									name={userData?.profile.nickname || ''}
									className="mr-1.6 [&_img]:w-7 [&_img]:h-7"
									size={28}
								/>
								<span className="text-1rem">{userData?.profile.nickname}</span>
							</div>
						) : (
							<IconButton
								icon="i-solar-user-plus-rounded-linear mr-4"
								onClick={onOpenLogin}
							/>
						)}
					</button>

					<IconButton icon="i-solar-settings-linear" onClick={onOpenSettings} />
					<IconButton icon="i-solar-pip-2-line-duotone" onClick={handlePip} />
					<IconButton
						icon="i-material-symbols-check-indeterminate-small-rounded"
						title="最小化"
						onClick={handleMinimize}
					/>
					<IconButton
						icon="i-material-symbols-chrome-maximize-outline"
						onClick={toggleMaximize}
					/>
					<IconButton icon="i-material-symbols-close-rounded" onClick={handleClose} />
				</div>
			</header>

			<Popover listen={userRef} onClose={() => {}} position="bottom">
				<div className="p-2">
					<button
						onClick={handleLogout}
						className="w-full px-3 py-2 text-left bg-transparent border-none text-[var(--text-color)] hover:bg-[var(--button-hover-bg-color)] rounded transition-colors duration-200 cursor-pointer"
					>
						退出登录
					</button>
				</div>
			</Popover>
		</>
	);
});

Navbar.displayName = 'Navbar';

export default Navbar;
