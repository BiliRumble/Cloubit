import styles from './Chip.module.scss';

interface ChipProps {
	text: string;
	onClick?: () => void;
	style?: React.CSSProperties;
	className?: string;
}
const Chip: React.FC<ChipProps> = ({ text, onClick, className = '', style = {} }) => {
	return (
		<div className={`${styles.chip} ${className}`} onClick={onClick} style={style}>
			<p>{text}</p>
		</div>
	);
};

export default Chip;
