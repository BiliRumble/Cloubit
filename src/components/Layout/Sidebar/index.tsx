import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getUserPlaylist } from '../../../apis/user';
import { useAuthStore } from '../../../store/auth';
import { useUserStore } from '../../../store/user';
import styles from './Sidebar.module.scss';

interface SideBarProps {
	className?: string;
}

const Sidebar: React.FC<SideBarProps> = ({ className }) => {
	const url = window.location.pathname;
	const navigate = useNavigate();

	const [playlist, setPlaylist] = useState<any[]>([]);

	useEffect(() => {
		const timer = setInterval(
			() => {
				getUserPlaylist().then((res) => {
					setPlaylist(res.playlist);
				});
			},
			1000 * 60 * 5
		);

		getUserPlaylist().then((res) => {
			setPlaylist(res.playlist);
		});

		return () => clearInterval(timer);
	}, []);

	return (
		<nav className={`${className || ''} ${styles.sidebar}`.trim()}>
			<div data-tauri-drag-region className={styles.sidebar__logo}>
				<img src="/vite.svg" alt="" />
				<h1>AzusaP</h1>
			</div>
			<div className={styles.sidebar__item}>
				<h1>推荐</h1>
				<button className={url === '/' ? styles.active : ''} onClick={() => navigate('/')}>
					<span className={`${styles.icon} i-solar-home-angle-linear`} />
					推荐
				</button>
			</div>
			<div className={styles.sidebar__item}>
				<h1>我的</h1>
				{useAuthStore.getState().isLogin && (
					<button
						className={url === '/playlist/like' ? styles.active : ''}
						onClick={() => navigate('/playlist/like')}
					>
						<span className={`${styles.icon} i-solar-heart-linear`} />
						收藏
					</button>
				)}
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
			{useAuthStore.getState().isLogin && (
				<div className={styles.sidebar__item}>
					<h1>
						歌单 <span className="i-solar-add-circle-linear" title="创建歌单" />
					</h1>
					{playlist.map((sidebar__item: any) => {
						if (
							sidebar__item.name === '我喜欢的音乐' &&
							sidebar__item.creator.userId ==
								useAuthStore.getState().userData?.profile.userId
						) {
							useUserStore.setState({ likePlaylist: sidebar__item.id });
							return null;
						}
						return (
							<button
								className={styles.sidebar__item__playlist}
								onClick={() => navigate(`/playlist/${sidebar__item.id}`)}
							>
								<img src={sidebar__item.coverImgUrl} alt="" />
								{sidebar__item.name}
							</button>
						);
					})}
				</div>
			)}
		</nav>
	);
};

export default Sidebar;
