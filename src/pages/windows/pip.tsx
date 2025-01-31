import { event } from '@tauri-apps/api';
import { Window } from '@tauri-apps/api/window';
import React, { useEffect, useState } from 'react';
import cover from '../../assets/images/song.png';
import { useTheme } from '../../hooks/useTheme';
import { closePip } from '../../managers/PIPWindowManager';
import { PlayListItem } from '../../models/song';
import styles from './pip.module.scss';

const Pip = () => {
	const appWindow = new Window('main');

	const [currentSong, setCurrentSong] = useState({ name: '无数据' } as PlayListItem);
	const [playing, setPlaying] = useState(false);
	const [seek, setSeek] = useState(0);
	const [duration, setDuration] = useState(1);
	const [lyric, setLyric] = useState<string | null>(null);

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
		const ul = event.listen('player-update-lyric', (event) => {
			setLyric(event.payload as string);
		});
		const ud = event.listen('player-update-duration', (event) => {
			const data = event.payload as number;
			setDuration(data);
		});
		const st = event.listen('switch-theme', (event) => {
			useTheme().initTheme();
		});

		event.emitTo('main', 'pip-request-current-song');

		return () => {
			ucs.then((f) => f());
			up.then((f) => f());
			us.then((f) => f());
			ul.then((f) => f());
			ud.then((f) => f());
			st.then((f) => f());
		};
	}, []);

	const progress = (seek / duration) * 100;

	return (
		<div
			data-tauri-drag-region
			className={styles.pip}
			style={{ '--progress': progress } as React.CSSProperties}
		>
			<div data-tauri-drag-region className={styles.pip__card}>
				<div
					onClick={() => appWindow.show()}
					data-tauri-drag-region
					className={styles.pip__card__cover}
				>
					<img src={currentSong?.cover || cover} alt="" />
				</div>
				<div data-tauri-drag-region className={styles.pip__card__info}>
					<h1>{lyric ? lyric : currentSong?.name}</h1>
				</div>
				<div data-tauri-drag-region className={styles.pip__card__control}>
					<div data-tauri-drag-region className={styles.pip__card__control__buttons}>
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
