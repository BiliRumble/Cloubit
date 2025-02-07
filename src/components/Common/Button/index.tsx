import styles from './Button.module.scss';

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
