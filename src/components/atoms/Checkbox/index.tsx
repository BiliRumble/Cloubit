import styles from './Checkbox.module.scss';

interface CheckboxProps {
	value: boolean;
	title?: string;
	onChange: (value: boolean) => void;
	className?: string;
	style?: React.CSSProperties;
}

const Checkbox: React.FC<CheckboxProps> = ({ value, title, onChange, className = '', style }) => {
	return (
		<label className={styles.checkbox + ' ' + className} title={title} style={style}>
			<input type="checkbox" checked={value} onChange={(e) => onChange(e.target.checked)} />
			<span className={styles.checkbox__checkmark}></span>
		</label>
	);
};

export default Checkbox;
