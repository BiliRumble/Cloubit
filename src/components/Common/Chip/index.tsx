import styles from './Chip.module.scss';

interface ChipProps {
	text: string;
	onClick?: () => void;
	onContextMenu?: () => void;
	style?: React.CSSProperties;
	className?: string;
}
const Chip: React.FC<ChipProps> = ({
	text,
	onClick,
	onContextMenu,
	className = '',
	style = {},
}) => {
	return (
		<div
			className={`${styles.chip} ${className}`}
			style={style}
			onClick={onClick}
			onContextMenu={onContextMenu}
		>
			<p>{text}</p>
		</div>
	);
};

export default Chip;
