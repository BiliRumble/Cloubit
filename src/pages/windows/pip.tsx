import { Window } from '@tauri-apps/api/window';
import React from 'react';
import { closePip } from '../../managers/PIPWindowManager';
import styles from './pip.module.scss';
import { getPlayStatus } from '../../managers/playManager';

const Pip = () => {
	const appWindow = new Window('main');

	const info = getPlayStatus().info;
	const status = getPlayStatus().status;

	const progress = status.playTime / status.maxTime;

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
					<img src={info.cover} />
				</div>
				<div data-tauri-drag-region className={styles.pip__info}>
					<h1>{info.name.substring(0, 18)}</h1>
				</div>
				<div data-tauri-drag-region className={styles.pip__control}>
					<div data-tauri-drag-region className={styles.control__buttons}>
						<span className="i-solar-rewind-back-line-duotone" />
						<span
							className={
								status.isPlaying
									? 'i-solar-pause-line-duotone'
									: 'i-solar-play-line-duotone'
							}
						/>
						<span className="i-solar-rewind-forward-line-duotone" />
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
