import { event } from '@tauri-apps/api';
import { Window } from '@tauri-apps/api/window';
import React, { useEffect, useState } from 'react';
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

	useEffect(() => {
		// 监听
		const ucs = event.listen('player-update-current-song', (event) => {
			const data = event.payload as PlayListItem;
			setCurrentSong(data);
		});
		const up = event.listen('player-update-playing', (event) => {
			const data = event.payload as boolean;
			setPlaying(data);
			console.log(`player-update-playing: ${data}`);
		});
		const us = event.listen('player-update-seek', (event) => {
			const data = event.payload as number;
			setSeek(data);
		});
		const ud = event.listen('player-update-duration', (event) => {
			const data = event.payload as number;
			setDuration(data);
		});

		event.emitTo('main', 'pip-request-current-song');

		return () => {
			ucs.then((f) => f());
			up.then((f) => f());
			us.then((f) => f());
			ud.then((f) => f());
		};
	}, []);

	const progress = (seek / duration) * 100;

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
							onClick={() => event.emitTo('main', 'pip-prev')}
						/>
						<span
							className={
								playing ? 'i-solar-pause-line-duotone' : 'i-solar-play-line-duotone'
							}
							onClick={async () => {
								event.emitTo('main', 'pip-play');
							}}
						/>
						<span
							className="i-solar-rewind-forward-line-duotone"
							onClick={() => event.emitTo('main', 'pip-next')}
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
