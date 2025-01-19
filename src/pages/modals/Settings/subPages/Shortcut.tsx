import { event } from '@tauri-apps/api';
import { useEffect, useState } from 'react';
import { registerShortcuts, unregisterShortcuts } from '../../../../managers/ShortcutManager';
import { useSettingStore } from '../../../../store/setting';
import Control, { Checkbox, KeyBinder } from './components/Controls';

const Shortcut: React.FC = () => {
	const settings = useSettingStore.getState();
	const [shortcuts, setShortcuts] = useState({
		enabled: settings.enableGlobalShortcut,
		play: settings.playShortcut,
		prev: settings.prevShortcut,
		next: settings.nextShortcut,
		volumeUp: settings.volumeUpShortcut,
		volumeDown: settings.volumeDownShortcut,
	});

	const bindKeys = async (
		keyValue: string[],
		target: 'play' | 'prev' | 'next' | 'volumeUp' | 'volumeDown'
	) => {
		const shortcut = keyValue.join('+');

		unregisterShortcuts(shortcuts[target]);
		useSettingStore.setState({ [`${target}Shortcut`]: keyValue } as any);
		registerShortcuts(shortcut, () => event.emit(`shortcut-${target}`));
	};

	useEffect(() => {
		setShortcuts({
			enabled: settings.enableGlobalShortcut,
			play: settings.playShortcut,
			prev: settings.prevShortcut,
			next: settings.nextShortcut,
			volumeUp: settings.volumeUpShortcut,
			volumeDown: settings.volumeDownShortcut,
		});
	}, [settings]);

	return (
		<div>
			<h2>设置</h2>
			<Control label="启用全局快捷键">
				<Checkbox
					value={shortcuts.enabled}
					title="启用后, 快捷键将在整个系统范围内生效"
					onChange={(checked) =>
						useSettingStore.setState({ enableGlobalShortcut: checked })
					}
				/>
			</Control>
			<h2>绑定</h2>
			<Control label="播放/暂停">
				<KeyBinder
					value={shortcuts.play}
					onKeysChange={(keyValue) => bindKeys(keyValue, 'play')}
				/>
			</Control>
			<Control label="上一首">
				<KeyBinder
					value={shortcuts.prev}
					onKeysChange={(keyValue) => bindKeys(keyValue, 'prev')}
				/>
			</Control>
			<Control label="下一首">
				<KeyBinder
					value={shortcuts.next}
					onKeysChange={(keyValue) => bindKeys(keyValue, 'next')}
				/>
			</Control>
			<Control label="音量增+">
				<KeyBinder
					value={shortcuts.volumeUp}
					onKeysChange={(keyValue) => bindKeys(keyValue, 'volumeUp')}
				/>
			</Control>
			<Control label="音量减-">
				<KeyBinder
					value={shortcuts.volumeDown}
					onKeysChange={(keyValue) => bindKeys(keyValue, 'volumeDown')}
				/>
			</Control>
		</div>
	);
};

export default Shortcut;
