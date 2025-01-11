import { useTheme } from '../../../../hooks/useTheme';
import { useSettingStore } from '../../../../store/setting';
import Control, { Select } from './components/Controls';

const GeneralSettings: React.FC = () => {
	const [themeOption, setThemeOption] = useSettingStore((state) => [state.theme, state.setTheme]);
	const { switchMode } = useTheme();

	const switchTheme = (value: 'light' | 'dark' | 'auto') => {
		switchMode(value);
		setThemeOption(value);
	};

	const options = [
		{ value: 'light', label: '浅色模式' },
		{ value: 'dark', label: '深色模式' },
		{ value: 'auto', label: '自动模式' },
	];

	return (
		<div>
			<h2>主题设置</h2>
			<Control label="主题">
				<Select options={options} value={themeOption} onChange={switchTheme} />
			</Control>
		</div>
	);
};

export default GeneralSettings;
