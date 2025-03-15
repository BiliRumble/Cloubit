import styles from './Checkbox.module.scss';

interface CheckboxProps {
	value: boolean;
	title?: string;
	onChange: (value: boolean) => void;
	className?: string;
	style?: React.CSSProperties;
	ref?: React.RefObject<HTMLInputElement>;
}

const Checkbox: React.FC<CheckboxProps> = ({
	value,
	title,
	onChange,
	className = '',
	style,
	ref,
}) => {
	return (
		<label className={styles.checkbox + ' ' + className} title={title} style={style}>
			<input
				type="checkbox"
				checked={value}
				onChange={(e) => onChange(e.target.checked)}
				ref={ref}
			/>
			<span className={styles.checkbox__checkmark}></span>
		</label>
	);
};

export default Checkbox;
