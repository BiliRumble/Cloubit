import styles from './Input.module.scss';

interface InputProps {
	value: string;
	title?: string;
	// inputmode
	allowType?:
		| 'none'
		| 'text'
		| 'tel'
		| 'url'
		| 'email'
		| 'numeric'
		| 'decimal'
		| 'search'
		| undefined;
	placeholder?: string;
	lazyPush?: boolean; // 如果为true，用户停止输入后才更新值
	onChange: (value: string) => void;
	className?: string;
	style?: React.CSSProperties;
	ref?: React.RefObject<HTMLInputElement>;
}

const Input: React.FC<InputProps> = ({
	value,
	title,
	allowType,
	placeholder,
	onChange,
	className = '',
	style,
	ref,
}) => {
	return (
		<input
			className={styles.textInput + ' ' + className}
			type="text"
			value={value}
			placeholder={placeholder}
			onChange={(e) => onChange(e.target.value)}
			title={title}
			inputMode={allowType}
			style={style}
			ref={ref}
		/>
	);
};

export default Input;
