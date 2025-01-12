import { useTheme } from '../../../../hooks/useTheme';
import { useSettingStore } from '../../../../store/setting';
import Control, { Checkbox, Select } from './components/Controls';

const GeneralSettings: React.FC = () => {
	const [themeOption, setThemeOption] = useSettingStore((state) => [state.theme, state.setTheme]);
	const { switchMode } = useTheme();
	const settingStore = useSettingStore();

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
				<Select
					options={options}
					title="懒狗是这样的，只做了黑白的"
					value={themeOption}
					onChange={switchTheme}
				/>
			</Control>
			<h2>搜索设置</h2>
			<Control label="启用搜索栏自动补全">
				<Checkbox
					value={settingStore.searchAutoComplete}
					title="去除牛皮癣小广告..?"
					onChange={() =>
						settingStore.setSearchAutoComplete(!settingStore.searchAutoComplete)
					}
				/>
			</Control>
			<Control label="启用搜索栏历史记录">
				<Checkbox
					value={settingStore.searchHistoryRecord}
					title="关闭不会删除历史记录，需要自己删除"
					onChange={() =>
						settingStore.setSearchHistoryRecord(!settingStore.searchHistoryRecord)
					}
				/>
			</Control>
		</div>
	);
};

export default GeneralSettings;
