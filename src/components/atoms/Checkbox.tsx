import { memo, useCallback } from 'react';

interface CheckboxProps {
	value: boolean;
	title?: string;
	onChange: (value: boolean) => void;
	className?: string;
	style?: React.CSSProperties;
	ref?: React.RefObject<HTMLInputElement>;
}

const Checkbox: React.FC<CheckboxProps> = memo(
	({ value, title, onChange, className = '', style, ref }) => {
		const handleChange = useCallback(
			(e: React.ChangeEvent<HTMLInputElement>) => {
				onChange(e.target.checked);
			},
			[onChange]
		);

		return (
			<label
				className={`flex items-center shadow-[var(--shadow)] transition-shadow duration-300 hover:shadow-[var(--shadow-hover)] ${className}`}
				title={title}
				style={style}
			>
				<input
					type="checkbox"
					checked={value}
					onChange={handleChange}
					ref={ref}
					className="appearance-none w-10 h-6 bg-[var(--button-bg-color)] rounded cursor-pointer relative outline-none shadow-[var(--shadow)] transition-all duration-300 ease-in-out checked:bg-[var(--primary-color)] before:content-[''] before:absolute before:w-6 before:h-6 before:bg-[var(--button-focus-bg-color)] before:rounded before:top-0 before:left-0 before:translate-x-0 before:transition-all before:duration-300 before:ease-in-out checked:before:translate-x-4.8"
				/>
			</label>
		);
	}
);

Checkbox.displayName = 'Checkbox';

export default Checkbox;
