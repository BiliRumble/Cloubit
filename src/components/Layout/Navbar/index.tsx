import { event } from '@tauri-apps/api';
import { Window } from '@tauri-apps/api/window';
import { useRef, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { usePlayerManager } from '../../../context/PlayerContext';
import { closePip, createPip } from '../../../managers/PIPWindowManager';
import Login from '../../../pages/modals/Login/Login';
import Settings from '../../../pages/modals/Settings/Settings';
import { useAuthStore } from '../../../store/auth';
import Modal from '../../Common/Modal';
import Popover from '../../Common/Popover';
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
						<button onClick={() => window.history.back()}>
							<span className="i-solar-alt-arrow-left-linear" />
						</button>
						<button onClick={() => window.history.forward()}>
							<span className="i-solar-alt-arrow-right-linear" />
						</button>
						<button
							onClick={() => {
								navigate(0);
							}}
						>
							<span className="i-solar-restart-linear" />
						</button>
					</div>
					<Search />
				</div>
				<div data-tauri-drag-region className={styles.navbar__right}>
					<button className={styles.navbar__right__login}>
						{useAuthStore.getState().isLogin ? (
							<div className={styles.navbar__right__login__info} ref={userRef}>
								<img
									src={useAuthStore.getState().userData?.profile.avatarUrl}
									alt="avatar"
								/>
								<span>{useAuthStore.getState().userData?.profile.nickname}</span>
							</div>
						) : (
							<span
								onClick={() => setLoginModalOpen(true)}
								className="i-solar-user-plus-rounded-linear"
							/>
						)}
					</button>
					<button id="Setting" onClick={() => setSettingModalOpen(true)}>
						<span className={`${styles.icon} i-solar-settings-linear`} />
					</button>
					<button
						onClick={() => {
							changePip();
							setTimeout(() => {
								event.emitTo('main', 'player-update-current-song', currentSong);
							}, 1000);
						}}
					>
						<span className="i-solar-pip-2-line-duotone" />
					</button>
					<button onClick={() => appWindow.minimize()}>
						<span className="i-material-symbols-check-indeterminate-small-rounded" />
					</button>
					<button onClick={() => toggleMaximize()}>
						<span className="i-material-symbols-chrome-maximize-outline" />
					</button>
					<button onClick={() => appWindow.hide()}>
						<span className="i-material-symbols-close-rounded" />
					</button>
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
					<button>退出登录</button>
				</div>
			</Popover>
		</>
	);
};

export default Navbar;
