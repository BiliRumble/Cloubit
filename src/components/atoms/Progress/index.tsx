import styles from './Progress.module.scss';

interface InputProps {
	value: number;
	max: number;
	onMouseDown: (e?: React.MouseEvent<HTMLInputElement>) => void;
	onMouseUp: (e?: React.MouseEvent<HTMLInputElement>) => void;
	onInput: (e?: React.FormEvent<HTMLInputElement>) => void;
	className?: string;
	style?: React.CSSProperties;
}

const Progress: React.FC<InputProps> = ({
	value,
	max,
	onMouseDown,
	onMouseUp,
	onInput,
	className = '',
	style,
}) => {
	return (
		<input
			type="range"
			min={0}
			max={max}
			value={value}
			onMouseDown={onMouseDown}
			onMouseUp={onMouseUp}
			onInput={onInput}
			className={styles.progress + ' ' + className}
			style={style}
		/>
	);
};

export default Progress;
