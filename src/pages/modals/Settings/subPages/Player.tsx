import Control, { Checkbox, Input, Select } from '../../../../components/Common/Controls/Controls';
import { useSettingStore } from '../../../../store/setting';

const PlayerSettings: React.FC = () => {
	const settingStore = useSettingStore();

	return (
		<div>
			<h2>播放器设置</h2>
			<Control label="自动播放">
				<Checkbox
					value={settingStore.autoPlay}
					title="播放器打开时自动播放上次播放内容"
					onChange={() => settingStore.setAutoPlay(!settingStore.autoPlay)}
				/>
			</Control>
			<Control label="启用播放进度保存">
				<Checkbox
					value={settingStore.savePlaySeek}
					title="关了后关闭程序播放进度不会保存"
					onChange={() => settingStore.setSavePlaySeek(!settingStore.savePlaySeek)}
				/>
			</Control>
			<Control label="启用听歌打卡">
				<Checkbox
					value={settingStore.scrobble}
					title="向网易云音乐推送听歌打卡信息"
					onChange={() => settingStore.setScrobble(!settingStore.scrobble)}
				/>
			</Control>
			<Control label="底栏显示歌词">
				<Checkbox
					value={settingStore.showLyrics}
					title="在底栏显示歌词, 代替歌手信息"
					onChange={() => settingStore.setShowLyrics(!settingStore.showLyrics)}
				/>
			</Control>
			<Control label="底栏歌词类型">
				<Select
					value={settingStore.lyricsType}
					title="底栏歌词类型"
					options={[
						{
							label: '原词',
							value: 'raw',
						},
						{
							label: '简中翻译',
							value: 'translate',
						},
					]}
					onChange={(v) => settingStore.setLyricsType(v as 'raw' | 'translate')}
				/>
			</Control>
			<Control label="渐入渐出">
				<Checkbox
					value={settingStore.fadeInOut}
					title="播放时启用渐入渐出效果"
					onChange={() => settingStore.setFadeInOut(!settingStore.fadeInOut)}
				/>
			</Control>
			<Control label="渐入渐出时间">
				<Input
					value={`${settingStore.fadeTime}`}
					title="渐入渐出时间(毫秒)"
					lazyPush={true}
					allowType="numeric"
					onChange={(v) => settingStore.setFadeTime(Number(v))}
				/>
			</Control>
			<h2>系统集成</h2>
			<Control label="向 SMTC 推送媒体数据">
				<Checkbox
					value={settingStore.pushToSMTC}
					title="向 SMTC 推送媒体数据"
					onChange={() => settingStore.setPushToSMTC(!settingStore.pushToSMTC)}
				/>
			</Control>
		</div>
	);
};

export default PlayerSettings;
