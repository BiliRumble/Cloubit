import styles from './Select.module.scss';

interface SelectProps {
	options: { value: string; label: string }[];
	value: string;
	title?: string;
	onChange: (value: any) => void;
	className?: string;
	style?: React.CSSProperties;
	ref?: React.RefObject<HTMLSelectElement>;
}

const Select: React.FC<SelectProps> = ({
	options,
	value,
	title,
	onChange,
	className = '',
	style,
	ref,
}) => {
	return (
		<select
			className={styles.select + ' ' + className}
			value={value}
			onChange={(e) => onChange(e.target.value)}
			title={title}
			style={style}
			ref={ref}
		>
			{options.map((option) => (
				<option key={option.value} value={option.value}>
					{option.label}
				</option>
			))}
		</select>
	);
};

export default Select;
