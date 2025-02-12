import { useEffect, useState } from 'react';
import { Button } from '../../atoms/Button';
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
		<Button onClick={startListening} className={styles.button + ' ' + className} style={style}>
			{keys.length > 0 ? keys.join(' + ') : '点击设置'}
		</Button>
	);
};

export default Control;
