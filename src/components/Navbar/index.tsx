import { Window } from '@tauri-apps/api/window';
import { useState } from 'react';
import styles from './Navbar.module.scss';

interface NavbarProps {
	className?: string;
}

const Navbar: React.FC<NavbarProps> = ({ className }) => {
	const appWindow = new Window('main');
	const [maximized, setMaximized] = useState(false);

	const toggleMaximize = async () => {
		try {
			appWindow.toggleMaximize();
			setMaximized(!maximized);
		} catch (e) {
			console.error(e);
		}
	};

	return (
		<nav data-tauri-drag-region className={`${className || ''} ${styles.navbar}`.trim()}>
			<div data-tauri-drag-region className={styles.leftside}>
				<div data-tauri-drag-region className={styles.leftside__buttons}>
					<button>
						<span className="i-solar-alt-arrow-left-linear" />
					</button>
					<button>
						<span className="i-solar-alt-arrow-right-linear" />
					</button>
					<button>
						<span className="i-solar-restart-linear" />
					</button>
				</div>
				<input className={styles.search} type="text" placeholder="Search" />
			</div>
			<div data-tauri-drag-region className={styles.rightside}>
				<div data-tauri-drag-region className={styles.windowControls}>
					<button onClick={() => appWindow.minimize()}>
						<svg width="16" height="16" viewBox="0 0 10 2">
							<path d="M0 0h10v1H0z" fill="currentColor" />
						</svg>
					</button>
					<button onClick={() => toggleMaximize()}>
						<svg width="16" height="16" viewBox="0 0 10 10">
							{!maximized ? (
								<path d="M0 0v10h10V0H0zm1 1h8v8H1V1z" fill="currentColor" />
							) : (
								<path
									d="M2.1,0v2H0v8.1h8.2v-2h2V0H2.1z M7.2,9.2H1.1V3h6.1V9.2z M9.2,7.1h-1V2H3.1V1h6.1V7.1z"
									fill="currentColor"
								/>
							)}
						</svg>
					</button>
					<button onClick={() => appWindow.close()}>
						<svg width="16" height="16" viewBox="0 0 10 10">
							<path
								d="M0 0l10 10m0-10L0 10"
								stroke="currentColor"
								stroke-width="1.5"
							/>
						</svg>
					</button>
				</div>
			</div>
		</nav>
	);
};

export default Navbar;
