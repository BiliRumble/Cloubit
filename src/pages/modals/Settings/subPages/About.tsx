import Control from '../../../../components/Common/Controls/Controls';

const About: React.FC<{ className: string }> = ({ className }) => {
	return (
		<div className={className}>
			<h2>关于</h2>
			<Control label={`版本 0.2.8`}>
				<a href="https://github.com/BiliRumble/QTMusic/releases" target="_blank">
					更新日志
				</a>
			</Control>
		</div>
	);
};

export default About;
