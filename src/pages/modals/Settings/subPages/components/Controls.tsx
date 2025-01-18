import styles from './Controls.module.scss';

interface ControlProps {
	label: string;
	children: React.ReactNode;
}

const Control: React.FC<ControlProps> = ({ label, children }) => {
	return (
		<div className={styles.control}>
			<label className={styles.label}>{label}</label>
			<div className={styles.input}>{children}</div>
		</div>
	);
};

interface CheckboxProps {
	value: boolean;
	title?: string;
	onChange: (value: boolean) => void;
}

export const Checkbox: React.FC<CheckboxProps> = ({ value, title, onChange }) => {
	return (
		<label className={styles.checkbox} title={title}>
			<input type="checkbox" checked={value} onChange={(e) => onChange(e.target.checked)} />
			<span className={styles.checkbox__checkmark}></span>
		</label>
	);
};

interface SelectProps {
	options: { value: string; label: string }[];
	value: string;
	title?: string;
	onChange: (value: any) => void;
}

export const Select: React.FC<SelectProps> = ({ options, value, title, onChange }) => {
	return (
		<select
			className={styles.select}
			value={value}
			onChange={(e) => onChange(e.target.value)}
			title={title}
		>
			{options.map((option) => (
				<option key={option.value} value={option.value}>
					{option.label}
				</option>
			))}
		</select>
	);
};

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
	lazyPush?: boolean; // 如果为true，用户停止输入后才更新值
	onChange: (value: string) => void;
}

export const Input: React.FC<InputProps> = ({ value, title, allowType, onChange }) => {
	return (
		<input
			className={styles.textInput}
			type="text"
			value={value}
			onChange={(e) => onChange(e.target.value)}
			title={title}
			inputMode={allowType}
		/>
	);
};

interface ButtonProps {
	title?: string;
	children: React.ReactNode;
	onClick: () => void;
}

export const Button: React.FC<ButtonProps> = ({ title, children, onClick }) => {
	return (
		<button className={styles.button} title={title} onClick={onClick}>
			{children}
		</button>
	);
};

export default Control;
