import { memo, useCallback } from 'react';

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

const Input: React.FC<InputProps> = memo(
	({ value, title, allowType, placeholder, onChange, className = '', style, ref }) => {
		const handleChange = useCallback(
			(e: React.ChangeEvent<HTMLInputElement>) => {
				onChange(e.target.value);
			},
			[onChange]
		);

		return (
			<input
				className={`bg-[var(--button-bg-color)] text-[var(--text-color)] shadow-[var(--shadow)] p-2 border-none rounded-6px outline-none transition-colors duration-300 ease-in-out hover:bg-[var(--button-hover-bg-color)] focus:bg-[var(--button-focus-bg-color)] ${className}`}
				type="text"
				value={value}
				placeholder={placeholder}
				onChange={handleChange}
				title={title}
				inputMode={allowType}
				style={style}
				ref={ref}
			/>
		);
	}
);

Input.displayName = 'Input';

export default Input;
