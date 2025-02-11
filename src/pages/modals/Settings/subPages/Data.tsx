import { Button } from '../../../../components/common/Button';
import Control from '../../../../components/common/Controls/Controls';

const DataSettings: React.FC<{ className: string }> = ({ className }) => {
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
		<div className={className}>
			<h2>数据</h2>
			<p>包含私人数据, 请谨慎操作。</p>
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

export default DataSettings;
