import { openUrl } from '@tauri-apps/plugin-opener';
import { Button } from '../../../../components/atoms/Button';
import Control from '../../../../components/numerator/Controls/Controls';

const About: React.FC<{ className: string }> = ({ className }) => {
	return (
		<div className={className}>
			<h2>关于</h2>
			<Control label={`版本 0.4.0-Preview`}>
				<Button onClick={() => openUrl('https://github.com/BiliRumble/AzusaP/releases')}>
					更新日志
				</Button>
			</Control>
			<Control label={`信息`}>
				<a href="https://wakatime.com/badge/user/347b183a-e02e-464a-a180-ed2963969f84/project/13a71991-4de0-4638-a7e6-5d3d7ed9a754">
					<img
						src="https://wakatime.com/badge/user/347b183a-e02e-464a-a180-ed2963969f84/project/13a71991-4de0-4638-a7e6-5d3d7ed9a754.svg"
						alt="wakatime"
					/>
				</a>
				<a href="https://github.com/BiliRumble/AzusaP">
					<img
						src="https://img.shields.io/github/license/bilirumble/AzusaP"
						alt="License"
					/>
				</a>
			</Control>
		</div>
	);
};

export default About;
