import { event } from '@tauri-apps/api';
import { Window } from '@tauri-apps/api/window';
import React, { useState } from 'react';
import { closePip } from '../../managers/PIPWindowManager';
import { PlayListItem } from '../../models/main';
import styles from './pip.module.scss';

const Pip = () => {
	const appWindow = new Window('main');

	const [currentSong, setCurrentSong] = useState({
		name: '等待SYNC',
		cover: 'https://',
	} as PlayListItem);
	const [playing, setPlaying] = useState(false);
	const [seek, setSeek] = useState(0);
	const [duration, setDuration] = useState(1);

	// 通过tauri拉取主窗口数据
	event.listen('player-updataCurrentSong', (event) => {
		const data = event.payload as PlayListItem;
		setCurrentSong(data);
	});

	event.listen('player-play', () => {
		setPlaying(true);
	});

	event.listen('player-pause', () => {
		setPlaying(false);
	});

	event.listen('player-updataSeek', (event) => {
		const data = event.payload as number;
		setSeek(data);
	});

	event.listen('player-updataDuration', (event) => {
		const data = event.payload as number;
		setDuration(data);
		// ok
	});

	const progress = (seek / duration) * 100;
	console.debug({progress, seek, duration});

	return (
		<div
			data-tauri-drag-region
			className={styles.pip}
			style={{ '--progress': progress } as React.CSSProperties}
		>
			<div data-tauri-drag-region className={styles.card}>
				<div
					onClick={() => appWindow.show()}
					data-tauri-drag-region
					className={styles.cover}
				>
					<img src={currentSong?.cover || 'https://'} alt="" />
				</div>
				<div data-tauri-drag-region className={styles.pip__info}>
					<h1>{currentSong?.name.substring(0, 18)}</h1>
				</div>
				<div data-tauri-drag-region className={styles.pip__control}>
					<div data-tauri-drag-region className={styles.control__buttons}>
						<span
							className="i-solar-rewind-back-line-duotone"
							onClick={async () => await event.emitTo('main', 'pip-prev')}
						/>
						<span
							className={
								playing ? 'i-solar-pause-line-duotone' : 'i-solar-play-line-duotone'
							}
							onClick={async () => {
								if (playing) {
									await event.emitTo('main', 'pip-pause');
								} else {
									await event.emitTo('main', 'pip-play1');
								}
							}}
						/>
						<span
							className="i-solar-rewind-forward-line-duotone"
							onClick={async () => await event.emitTo('main', 'pip-next')}
						/>
					</div>
				</div>
				<div className={styles.closeBtn}>
					<span
						onClick={() => {
							closePip();
						}}
						className="i-material-symbols-close-small-rounded"
					/>
				</div>
			</div>
		</div>
	);
};

export default Pip;
