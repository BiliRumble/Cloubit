import { useTheme } from '../../../../hooks/useTheme';
import { useSettingStore } from '../../../../store/setting';
import Control, { Button, Checkbox, Select } from './components/Controls';

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

	const exportData = () => {
		const data = {
			localStorageData: localStorage,
			sessionStorageData: sessionStorage,
		};
		if (data) {
			// 创建一个Blob对象
			const blob = new Blob([JSON.stringify(data)], { type: 'application/json' });
			// 创建一个下载链接
			const url = URL.createObjectURL(blob);
			// 打开下载链接
			const link = document.createElement('a');
			link.href = url;
			link.download = `qtme-data-${new Date().toISOString()}.json`;
			link.click();
			link.remove();
		}
	};

	const importData = async () => {
		const fileInput = document.createElement('input');
		fileInput.type = 'file';
		fileInput.accept = '.json';
		fileInput.onchange = async (e) => {
			const target = e.target as HTMLInputElement;
			const file = target.files?.[0];
			if (file) {
				const reader = new FileReader();
				reader.onload = (e) => {
					const fileContent = e.target?.result as string;
					try {
						const data = JSON.parse(fileContent);
						// 将数据写入localStorage
						localStorage.clear();
						sessionStorage.clear();
						const localStorageData = data.localStorageData;
						const sessionStorageData = data.sessionStorageData;
						for (const key in localStorageData) {
							localStorage.setItem(key, localStorageData[key]);
						}
						for (const key in sessionStorageData) {
							sessionStorage.setItem(key, sessionStorageData[key]);
						}
						alert('导入成功');
						window.location.reload();
					} catch (error) {
						alert('导入失败');
					}
				};
				reader.readAsText(file);
			}
		};
		fileInput.click();
	};

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
			<h2>杂项</h2>
			<Control label="导出数据">
				<Button onClick={exportData} title="点击导出数据">
					导出数据
				</Button>
			</Control>
			<Control label="导入数据">
				<Button onClick={importData} title="点击导入数据">
					导入数据
				</Button>
			</Control>
			<Control label="清空数据 (播放音乐时可能存在一些问题)">
				<Button
					onClick={() => {
						localStorage.clear();
						sessionStorage.clear();
						alert('已清空数据, 点击确定后刷新页面');
						window.location.reload();
					}}
					title="能解决一些更新后的问题"
				>
					清空数据
				</Button>
			</Control>
		</div>
	);
};

export default GeneralSettings;
