import styles from './Sidebar.module.scss';
import '@mdui/icons/home--outlined.js';
import '@mdui/icons/sentiment-satisfied--outlined.js';
import '@mdui/icons/access-time--outlined.js';
import '@mdui/icons/download--outlined.js';
import '@mdui/icons/settings--outlined.js';
import '@mdui/icons/accessible-forward--outlined.js';

interface SideBarProps {
	className?: string;
}

const Sidebar: React.FC<SideBarProps> = ({ className }) => {
	const url = window.location.pathname;
	return (
		<nav className={`${className || ''} ${styles.sidebar}`.trim()}>
			<div data-tauri-drag-region className={styles.logo}>
				<img src="/vite.svg" alt="" />
				<h1>QTMusic</h1>
			</div>
			<div className={styles.item}>
				<h1>推荐</h1>
				<button
					className={url === '/' ? styles.active : ''}
					onClick={() => (window.location.href = '/')}
				>
					<span className={`${styles.icon} i-solar-home-angle-linear`} />
					推荐
				</button>
			</div>
			<div className={styles.item}>
				<h1>我的</h1>
				<button
					className={url === '/playlist/like' ? styles.active : ''}
					onClick={() => (window.location.href = '/playlist/like')}
				>
					<span className={`${styles.icon} i-solar-heart-linear`} />
					收藏
				</button>
				<button
					className={url === '/history' ? styles.active : ''}
					onClick={() => (window.location.href = '/history')}
				>
					<span className={`${styles.icon} i-solar-history-linear`} />
					历史
				</button>
				<button
					className={url === '/download' ? styles.active : ''}
					onClick={() => (window.location.href = '/download')}
				>
					<span className={`${styles.icon} i-solar-download-minimalistic-linear`} />
					下载
				</button>
			</div>
			<div className={styles.item}>
				<h1>歌单</h1>
				{/* todo... */}
			</div>
		</nav>
	);
};

export default Sidebar;
