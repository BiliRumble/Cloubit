import styles from './Chip.module.scss';

interface ChipProps {
	text: string;
	onClick?: () => void;
	classNames?: string;
}
const Chip: React.FC<ChipProps> = ({ text, onClick, classNames = '' }) => {
	return (
		<div className={`${styles.chip} ${classNames}`} onClick={onClick}>
			<p>{text}</p>
		</div>
	);
};

export default Chip;
