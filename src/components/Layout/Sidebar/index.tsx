import { useNavigate } from 'react-router-dom';
import styles from './Sidebar.module.scss';

interface SideBarProps {
	className?: string;
}

const Sidebar: React.FC<SideBarProps> = ({ className }) => {
	const url = window.location.pathname;
	const navigate = useNavigate();

	return (
		<nav className={`${className || ''} ${styles.sidebar}`.trim()}>
			<div data-tauri-drag-region className={styles.logo}>
				<img src="/vite.svg" alt="" />
				<h1>QTMusic</h1>
			</div>
			<div className={styles.item}>
				<h1>推荐</h1>
				<button className={url === '/' ? styles.active : ''} onClick={() => navigate('/')}>
					<span className={`${styles.icon} i-solar-home-angle-linear`} />
					推荐
				</button>
			</div>
			<div className={styles.item}>
				<h1>我的</h1>
				<button
					className={url === '/playlist/like' ? styles.active : ''}
					onClick={() => navigate('/playlist/like')}
				>
					<span className={`${styles.icon} i-solar-heart-linear`} />
					收藏
				</button>
				<button
					className={url === '/history' ? styles.active : ''}
					onClick={() => navigate('/history')}
				>
					<span className={`${styles.icon} i-solar-history-linear`} />
					历史
				</button>
				<button
					className={url === '/download' ? styles.active : ''}
					onClick={() => navigate('/download')}
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
