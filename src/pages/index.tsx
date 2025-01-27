import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { getUserDailyPlaylist, getUserDailySongs } from '../apis/user';
import Card from '../components/Common/Card';
import { DailySongsResult, recommendPlaylist } from '../models/song';
import { useAuthStore } from '../store/auth';
import styles from './index.module.scss';

const Home = () => {
	const isLogin = useAuthStore.getState().isLogin;
	const userAccount = useAuthStore.getState().userData;
	const navigate = useNavigate();

	const [playlist, setPlaylist] = useState<recommendPlaylist | null>(null);
	const [userDailySongs, setUserDailySongs] = useState<DailySongsResult | null>(null);

	useEffect(() => {
		if (isLogin) {
			getUserDailySongs().then((res) => {
				setUserDailySongs(res);
			});
			getUserDailyPlaylist().then((res) => {
				setPlaylist(res);
			});
		}
	}, [isLogin]);

	const hour = new Date().getHours();
	const welcome =
		hour < 6
			? '夜深了'
			: hour < 9
				? '早上好'
				: hour < 12
					? '上午好'
					: hour < 18
						? '下午好'
						: '晚上好';

	return (
		<div className={styles.recommends}>
			<h1 className={styles.title}>
				{welcome + (isLogin ? ', ' + userAccount?.profile.nickname + '!' : '!')}
			</h1>
			{isLogin && (
				<div className={styles.permanent}>
					<div className={styles.permanent__left}>
						<div className={styles.permanent__left__daily + ' ' + styles.card}>
							<div className={styles.permanent__left__daily__cover}>
								<img
									src={userDailySongs?.data.dailySongs[0].al.picUrl}
									alt="每日30封面"
								/>
								<div
									className={styles.mask}
									onClick={() => navigate('/playlist/daily')}
								>
									<i className="i-solar-play-linear" />
								</div>
							</div>
							<div className={styles.info}>
								<h2 className={styles.daily__title}>每日推荐</h2>
								<p>根据你的口味生成 · 每天6:00更新</p>
							</div>
						</div>
						<div className={styles.favorites + ' ' + styles.card}>
							<div className={styles.info}>
								<h2 className={styles.favorites__title}>Favorites</h2>
								<p>Recommend for you</p>
							</div>
						</div>
					</div>
					<div className={styles.permanent__right}>
						<div className={styles.fm + ' ' + styles.card}>
							<img src="https://via.placeholder.com/100" alt="私人FM封面" />
							<div className={styles.info}>
								<h2 className={styles.fm__title}>FM</h2>
								<p>Your private FM</p>
							</div>
						</div>
					</div>
				</div>
			)}
			<h2 className={styles.title}>歌单</h2>
			<div className={styles.recommends__playlist}>
				{playlist?.result.map((list) => {
					return (
						<Card
							key={list.id}
							className={styles.recommends__playlist__card}
							text={list.name}
							cover={list.picUrl}
							onClick={() => navigate('/playlist/' + list.id)}
						/>
					);
				})}
			</div>
		</div>
	);
};

export default Home;
