import { memo, useCallback, useEffect, useMemo } from 'react';

interface SelectProps {
	options: { value: string; label: string }[];
	value: string;
	title?: string;
	onChange: (value: any) => void;
	className?: string;
	style?: React.CSSProperties;
	ref?: React.RefObject<HTMLSelectElement>;
}

// 创建选项样式的 hook
const useSelectStyles = () => {
	useEffect(() => {
		const styleId = 'select-option-styles';
		if (!document.getElementById(styleId)) {
			const style = document.createElement('style');
			style.id = styleId;
			style.textContent = `
				.custom-select option {
					background: var(--button-bg-color);
					color: var(--text-color);
				}
			`;
			document.head.appendChild(style);
		}
	}, []);
};

const Select: React.FC<SelectProps> = memo(
	({ options, value, title, onChange, className = '', style, ref }) => {
		useSelectStyles();

		const handleChange = useCallback(
			(e: React.ChangeEvent<HTMLSelectElement>) => {
				onChange(e.target.value);
			},
			[onChange]
		);

		// 使用useMemo优化options渲染，并确保key的唯一性
		const renderedOptions = useMemo(() => {
			return options.map((option, index) => (
				<option key={`${option.value}-${index}`} value={option.value}>
					{option.label}
				</option>
			));
		}, [options]);

		return (
			<select
				className={`custom-select w-full p-2 bg-[var(--button-bg-color)] text-[var(--text-color)] border-none min-w-32 rounded-6px outline-none shadow-[var(--shadow)] transition-shadow duration-300 hover:shadow-[var(--shadow-hover)] ${className}`}
				value={value}
				onChange={handleChange}
				title={title}
				style={style}
				ref={ref}
			>
				{renderedOptions}
			</select>
		);
	}
);

Select.displayName = 'Select';

export default Select;
