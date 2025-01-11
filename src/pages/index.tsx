import Card from '../components/Card';
import styles from './index.module.scss';

const Home = () => {
	const data = {
		userName: 'rumble',
		id: '123456789',
	};

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

	const playlist = [
		{
			title: 'Card 1',
			cover: 'https://via.placeholder.com/100',
			url: '#',
		},
		{
			title: 'Card 2',
			cover: 'https://via.placeholder.com/100',
			url: '#',
		},
		{
			title: 'Card 3bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb',
			cover: 'https://via.placeholder.com/100',
			url: '#',
		},
		{
			title: 'Card 4',
			cover: 'https://via.placeholder.com/100',
			url: '#',
		},
		{
			title: 'Card 5',
			cover: 'https://via.placeholder.com/100',
			url: '#',
		},
		{
			title: 'Card 6',
			cover: 'https://via.placeholder.com/100',
			url: '#',
		},
		{
			title: 'Card 7',
			cover: 'https://via.placeholder.com/100',
			url: '#',
		},
		{
			title: 'Card 8',
			cover: 'https://via.placeholder.com/100',
			url: '#',
		},
		{
			title: 'Card 9aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
			cover: 'https://via.placeholder.com/100',
			url: '#',
		},
	];

	return (
		<div className={styles.recommends}>
			<div id="test">
				<button
					onClick={async () => {
						//
					}}
				>
					Login
				</button>
			</div>
			<h1 className={styles.title}>
				{welcome}, {data.userName}
			</h1>
			<div className={styles.permanent}>
				<div className={styles.permanent__left}>
					<div className={styles.daily + ' ' + styles.card}>
						<img src="https://via.placeholder.com/100" alt="每日30封面" />
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
			<h2 className={styles.title}>歌单</h2>
			<div className={styles.playlist}>
				{playlist.map((list) => {
					return (
						<Card
							key={list.title}
							className={styles.card}
							text={list.title}
							cover={list.cover}
							url={list.url}
						/>
					);
				})}
			</div>
		</div>
	);
};

export default Home;
