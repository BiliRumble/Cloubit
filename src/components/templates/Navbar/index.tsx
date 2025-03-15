import { event } from '@tauri-apps/api';
import { Window } from '@tauri-apps/api/window';
import { useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { logout } from '../../../apis/login';
import { usePlayerManager } from '../../../context/PlayerContext';
import { closePip, createPip } from '../../../managers/PIPWindowManager';
import Login from '../../../pages/modals/Login/Login';
import Settings from '../../../pages/modals/Settings/Settings';
import { useAuthStore } from '../../../store/auth';
import { useUserStore } from '../../../store/user';
import { IconButton } from '../../atoms/Button/icon';
import Avatar from '../../numerator/Avatar';
import Modal from '../../numerator/Modal';
import Popover from '../../numerator/Popover';
import Search from './Search';
import styles from './Navbar.module.scss';

interface NavbarProps {
	className?: string;
}

const Navbar: React.FC<NavbarProps> = ({ className }) => {
	const appWindow = new Window('main');
	const [maximized, setMaximized] = useState(false);
	const [isSettingModalOpen, setSettingModalOpen] = useState(false);
	const [isLoginModalOpen, setLoginModalOpen] = useState(false);
	const currentSong = usePlayerManager().currentSong;

	const userRef = useRef<HTMLDivElement>(null);

	const toggleMaximize = async () => {
		try {
			appWindow.toggleMaximize();
			setMaximized(!maximized);
		} catch (e) {
			console.error(e);
		}
	};

	const changePip = async () => {
		if (localStorage.getItem('isPip') == 'true') return closePip();
		createPip();
	};

	const navigate = useNavigate();

	return (
		<>
			<header data-tauri-drag-region className={`${className || ''} ${styles.navbar}`.trim()}>
				<div data-tauri-drag-region className={styles.navbar__left}>
					<div data-tauri-drag-region className={styles.buttons}>
						<IconButton
							title="返回"
							icon="i-solar-alt-arrow-left-linear"
							onClick={() => window.history.back()}
						/>
						<IconButton
							title="继续"
							icon="i-solar-alt-arrow-right-linear"
							onClick={() => window.history.forward()}
						/>
						<IconButton
							title="刷新"
							icon="i-solar-restart-linear"
							onClick={() => {
								navigate(0);
							}}
						/>
					</div>
					<Search />
					<IconButton
						title="听歌识曲"
						icon="i-solar-microphone-linear"
						onClick={() => navigate('/recognize')}
					/>
				</div>
				<div data-tauri-drag-region className={styles.navbar__right}>
					<button className={styles.navbar__right__login}>
						{useAuthStore.getState().isLogin ? (
							<div className={styles.navbar__right__login__info} ref={userRef}>
								<Avatar
									url={useAuthStore.getState().userData?.profile.avatarUrl || ''}
									name={useAuthStore.getState().userData?.profile.nickname || ''}
									className={styles.navbar__right__login__info__avatar}
									size={28}
								/>
								<span>{useAuthStore.getState().userData?.profile.nickname}</span>
							</div>
						) : (
							<IconButton
								icon="i-solar-user-plus-rounded-linear"
								onClick={() => setLoginModalOpen(true)}
							/>
						)}
					</button>
					<IconButton
						icon="i-solar-settings-linear"
						onClick={() => setSettingModalOpen(true)}
					/>
					<IconButton
						icon="i-solar-pip-2-line-duotone"
						onClick={() => {
							changePip();
							setTimeout(() => {
								event.emitTo('main', 'player-update-current-song', currentSong);
							}, 1000);
						}}
					/>
					<IconButton
						icon="i-material-symbols-check-indeterminate-small-rounded"
						title="最小化"
						onClick={() => appWindow.minimize()}
					/>
					<IconButton
						icon="i-material-symbols-chrome-maximize-outline"
						onClick={() => toggleMaximize()}
					/>
					<IconButton
						icon="i-material-symbols-close-rounded"
						onClick={() => appWindow.hide()}
					/>
				</div>
			</header>
			<Modal isOpen={isSettingModalOpen} onClose={() => setSettingModalOpen(false)}>
				<Settings />
			</Modal>
			<Modal isOpen={isLoginModalOpen} onClose={() => setLoginModalOpen(false)}>
				<Login />
			</Modal>
			<Popover listen={userRef} onClose={() => {}} position="bottom">
				<div>
					<button
						onClick={() => {
							logout();
							const auth = useAuthStore.getState();
							auth.setIsLogin(false);
							auth.setCookie(null);
							auth.setUserData(null);
							const userData = useUserStore.getState();
							userData.setDailySong({ timestamp: 0, tracks: null });
							userData.setRecommendPlaylist({ timestamp: 0, playlists: null });
							userData.setLikePlaylist(null);
							window.location.reload();
						}}
					>
						退出登录
					</button>
				</div>
			</Popover>
		</>
	);
};

export default Navbar;
