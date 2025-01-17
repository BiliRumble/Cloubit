import { Window } from '@tauri-apps/api/window';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { closePip, createPip } from '../../../managers/PIPWindowManager';
import Settings from '../../../pages/modals/Settings/Settings';
import Modal from '../../Modal';
import Search from './Search';
import styles from './Navbar.module.scss';
import Login from '../../../pages/modals/Login/Login';

interface NavbarProps {
	className?: string;
}

const Navbar: React.FC<NavbarProps> = ({ className }) => {
	const appWindow = new Window('main');
	const [maximized, setMaximized] = useState(false);
	const [isSettingModalOpen, setSettingModalOpen] = useState(false);
	const [isLoginModalOpen, setLoginModalOpen] = useState(false);

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

	const openSettingModal = () => {
		setSettingModalOpen(true);
	};

	const closeSettingModal = () => {
		setSettingModalOpen(false);
	};

	const openLoginModal = () => {
		setLoginModalOpen(true);
	};

	const closeLoginModal = () => {
		setLoginModalOpen(false);
	};

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
					<button id="Login" onClick={openLoginModal}>
						Login
					</button>
					<button id="Setting" onClick={openSettingModal}>
						<span className={`${styles.icon} i-solar-settings-linear`} />
					</button>
					<button onClick={() => changePip()}>
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
			<Modal isOpen={isSettingModalOpen} onClose={closeSettingModal}>
				<Settings />
			</Modal>
			<Modal isOpen={isLoginModalOpen} onClose={closeLoginModal}>
				<Login />
			</Modal>
		</>
	);
};

export default Navbar;
