import { useEffect, useState } from 'react';
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

interface KeyBinderProps {
	value?: string[];
	onKeysChange: (keys: string[]) => void;
	style?: React.CSSProperties;
	className?: string;
}

export const KeyBinder: React.FC<KeyBinderProps> = ({
	value,
	onKeysChange,
	style,
	className = '',
}) => {
	const [keys, setKeys] = useState<string[]>(value || []);
	const [isListening, setIsListening] = useState(false);

	useEffect(() => {
		const handleKeyDown = (event: KeyboardEvent) => {
			event.preventDefault();
			if (!isListening) return;

			let key = event.key;

			if (key === ' ') {
				key = 'Space';
			}

			if (key === 'Control') {
				key = 'Ctrl';
			}

			if (!keys.includes(key)) {
				setKeys((prevKeys) => [...prevKeys, key]);
			}
		};

		const handleKeyUp = () => {
			setIsListening(false);
			onKeysChange(keys);
		};

		if (isListening) {
			window.addEventListener('keydown', handleKeyDown);
			window.addEventListener('keyup', handleKeyUp);
		}

		return () => {
			window.removeEventListener('keydown', handleKeyDown);
			window.removeEventListener('keyup', handleKeyUp);
		};
	}, [isListening, keys, onKeysChange]);

	const startListening = () => {
		setIsListening(true);
		setKeys([]);
	};

	return (
		<button onClick={startListening} className={styles.button + ' ' + className} style={style}>
			{keys.length > 0 ? keys.join(' + ') : '点击设置'}
		</button>
	);
};

export default Control;
