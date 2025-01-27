import { useState } from 'react';
import About from './subPages/About';
import GeneralSettings from './subPages/General';
import PlayerSettings from './subPages/Player';
import Shortcut from './subPages/Shortcut';
import styles from './Settings.module.scss';

const Settings = () => {
	const [page, setPage] = useState<'general' | 'player' | 'shortcut' | 'cache' | 'about'>(
		'general'
	);

	return (
		<div className={styles.settings}>
			<div className={styles.settings__sidebar}>
				<div className={styles.logo}>
					<h1>设置</h1>
				</div>
				<div className={styles.settings__sidebar__list}>
					<button
						className={styles.settings__sidebar__list__item}
						onClick={() => setPage('general')}
					>
						通用设置
					</button>
					<button
						className={styles.settings__sidebar__list__item}
						onClick={() => setPage('player')}
					>
						播放器设置
					</button>
					<button
						className={styles.settings__sidebar__list__item}
						onClick={() => setPage('shortcut')}
					>
						全局快捷键
					</button>
					<button
						className={styles.settings__sidebar__list__item}
						onClick={() => setPage('cache')}
					>
						缓存设置
					</button>
					<button
						className={styles.settings__sidebar__list__item}
						onClick={() => setPage('about')}
					>
						关于
					</button>
				</div>
			</div>
			<div className={styles.settings__content}>
				{page === 'general' && (
					<GeneralSettings className={styles.settings__content__item} />
				)}
				{page === 'player' && <PlayerSettings className={styles.settings__content__item} />}
				{page === 'shortcut' && <Shortcut className={styles.settings__content__item} />}
				{page === 'about' && <About className={styles.settings__content__item} />}
			</div>
		</div>
	);
};

export default Settings;
