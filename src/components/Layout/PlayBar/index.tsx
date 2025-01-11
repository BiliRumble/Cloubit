import { getPlayStatus, setPlayStatus } from '../../../managers/playManager';
import styles from './PlayBar.module.scss';

interface PlayBarProps {
	className?: string;
}

const PlayBar: React.FC<PlayBarProps> = ({ className }) => {
	const data = getPlayStatus().info;
	const status = getPlayStatus().status;

	return (
		<div className={`${className || ''} ${styles.playbar}`.trim()}>
			<audio id="audio" src={data.song} />
			<div className={styles.playbar__left}>
				<div className={styles.info}>
					<img src={data.cover} alt={`${data.name}的封面`} />
					<div className={styles.info__text}>
						<h1 id="title">{data.name}</h1>
						<h2>{data.artist}</h2>
					</div>
				</div>
				<div className={styles.action}>
					<span className={`i-solar-heart-angle-line-duotone ${styles.like}`} />
					{!data.isFromPlayList && <span className="i-solar-heart-broken-line-duotone" />}
					<span className="i-solar-dialog-2-line-duotone" />
					<span className="i-solar-menu-dots-circle-line-duotone" />
				</div>
			</div>
			<div className={styles.playbar__control}>
				<div className={styles.buttons}>
					<span
						className={
							status.mode === 'loop'
								? 'i-solar-repeat-line-duotone'
								: status.mode === 'random'
									? 'i-solar-shuffle-line-duotone'
									: 'i-solar-repeat-one-line-duotone'
						}
					/>
					<span className="i-solar-rewind-back-line-duotone" />
					<span
						className={
							status.isPlaying
								? 'i-solar-pause-line-duotone'
								: 'i-solar-play-line-duotone'
						}
						onClick={() => {
							const audio = document.getElementById('audio') as HTMLAudioElement;
							if (status.isPlaying) {
								audio.pause();
								setPlayStatus({ ...status, isPlaying: false });
							} else {
								audio.play();
								setPlayStatus({ ...status, isPlaying: true });
							}
						}}
					/>
					<span className="i-solar-rewind-forward-line-duotone" />
					<span
						className={
							status.isMuted
								? 'i-solar-muted-line-duotone'
								: status.volume < 0.25
									? 'i-solar-volume-line-duotone'
									: status.volume < 0.75
										? 'i-solar-volume-small-line-duotone'
										: 'i-solar-volume-loud-line-duotone'
						}
					/>
				</div>
				<div className={styles.progress}>
					<span>
						{`${Math.floor(status.playTime / 60)
							.toString()
							.padStart(2, '0')}:${Math.floor(status.playTime % 60)
							.toString()
							.padStart(2, '0')}`}
					</span>
					<input
						type="range"
						min={0}
						max={styles.maxTime}
						value={styles.playTime}
						onChange={(e) => {
							console.log(e);
						}}
					/>
					<span>
						{`${Math.floor(status.maxTime / 60)
							.toString()
							.padStart(2, '0')}:${Math.floor(status.maxTime % 60)
							.toString()
							.padStart(2, '0')}`}
					</span>
				</div>
			</div>
			<div className={styles.playbar__list}>
				<span className="i-solar-hamburger-menu-line-duotone" />
			</div>
		</div>
	);
};

export default PlayBar;
