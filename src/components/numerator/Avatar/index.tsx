import LazyImage from '../../atoms/LazyImage';
import styles from './Avatar.module.scss';

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
			className={styles.avatar + ' ' + className}
			style={{ width: size, height: size, ...style }}
		>
			<LazyImage src={url} alt={name} className={styles.avatar__image} />
		</div>
	);
};

export default Avatar;
