import LazyImage from '@/components/atoms/Image';

interface AvatarProps {
	url: string;
	name?: string;
	size?: number;
	className?: string;
	style?: React.CSSProperties;
}

const Avatar: React.FC<AvatarProps> = ({ url, name = '', size = 40, className = '', style }) => {
	return (
		<div
			className={
				'relative round-full overflow-hidden bg-[var(--button-bg-color)] ' + className
			}
			style={{ width: size, height: size, ...style }}
		>
			<LazyImage
				src={url}
				alt={name}
				className="w-full h-full object-cover object-cover"
				rounded="full"
			/>
		</div>
	);
};

export default Avatar;
