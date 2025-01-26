import Control from '../../../../components/Common/Controls/Controls';

const About: React.FC = () => {
	return (
		<div>
			<h2>关于</h2>
			<Control label={`版本 0.1.0`}>
				<a href="https://github.com/BiliRumble/QTMusic/releases">更新日志</a>
			</Control>
		</div>
	);
};

export default About;
