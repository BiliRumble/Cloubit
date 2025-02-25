import { Window } from '@tauri-apps/api/window';
import React, { useEffect, useState } from 'react';
import cover from '../../assets/images/song.png';
import { closePip } from '../../managers/PIPWindowManager';
import { PlayListItem } from '../../models/song';
import { eventBus } from '../../utils/EventBus';
import styles from './pip.module.scss';

const Pip = () => {
	const appWindow = new Window('main');

	const [currentSong, setCurrentSong] = useState({ name: '无数据' } as PlayListItem);
	const [playing, setPlaying] = useState(false);
	const [seek, setSeek] = useState(0);
	const [duration, setDuration] = useState(1);
	const [lyric, setLyric] = useState<string | null>(null);

	useEffect(() => {
		eventBus.on('playerCurrent', (data: PlayListItem) => {
			if (!data) return;
			setCurrentSong(data);
		});

		eventBus.on('playerState', (data: boolean) => {
			setPlaying(data);
		});

		eventBus.on('playerSeek', (data: number) => {
			setSeek(data);
		});

		eventBus.on('playerDuration', (data: number) => {
			setDuration(data);
		});

		eventBus.on('playerLyricChange', (data: string) => {
			setLyric(data);
		});

		eventBus.on('systemTheme', (data: string) => {
			const body = document.documentElement;
			body.setAttribute('data-theme', data);
		});

		eventBus.emit('systemPipReady');
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
							onClick={() => eventBus.emit('playerPrev')}
						/>
						<span
							className={
								playing ? 'i-solar-pause-line-duotone' : 'i-solar-play-line-duotone'
							}
							onClick={async () => {
								eventBus.emit('playerSetState');
							}}
						/>
						<span
							className="i-solar-rewind-forward-line-duotone"
							onClick={() => eventBus.emit('playerNext')}
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
