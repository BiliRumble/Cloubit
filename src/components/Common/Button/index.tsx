import styles from './Button.module.scss';

interface ButtonProps {
	title?: string;
	children: React.ReactNode;
	onClick: () => void;
}

/**
 * Button 组件
 *
 * @param {string} title - 按钮标题
 * @param {React.ReactNode} children - 按钮内容
 * @param {function} onClick - 点击事件
 */
export const Button: React.FC<ButtonProps> = ({ title, children, onClick }) => {
	return (
		<button className={styles.button} title={title} onClick={onClick}>
			{children}
		</button>
	);
};
