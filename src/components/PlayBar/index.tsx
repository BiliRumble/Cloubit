import styles from './PlayBar.module.scss';

interface PlayBarProps {
	className?: string;
}

const PlayBar: React.FC<PlayBarProps> = ({ className }) => {
	// 临时信息
	const data = {
		name: '测试歌曲aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
		cover: 'http://y.gtimg.cn/music/photo_new/T002R500x500M000000vpEZf4eAeeG.jpg?pcachetim',
	};

	return (
		<div className={`${className || ''} ${styles.playbar}`.trim()}>
			<div className={styles.playbar__info}>
				<img src={data.cover} alt={`${data.name}的封面`} />
				<div className={styles.title_content}>
					<h1>{data.name}</h1>
				</div>
			</div>
		</div>
	);
};

export default PlayBar;
