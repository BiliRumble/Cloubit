import styles from './Button.module.scss';

interface ButtonProps {
	title?: string;
	children: React.ReactNode;
	onClick?: () => void;
	className?: string;
	style?: React.CSSProperties;
	ref?: React.RefObject<HTMLButtonElement>;
}

/**
 * Button 组件
 *
 * @param {string} title - 按钮标题
 * @param {React.ReactNode} children - 按钮内容
 * @param {function} onClick - 点击事件
 * @param {string} className - 自定义类名
 * @param {React.CSSProperties} style - 自定义样式
 * @param {React.RefObject<HTMLButtonElement>} ref - 按钮引用
 */
export const Button: React.FC<ButtonProps> = ({
	title,
	children,
	onClick,
	className = '',
	style,
	ref,
}) => {
	return (
		<button
			className={styles.button + ' ' + className}
			title={title}
			onClick={onClick}
			style={style}
			ref={ref}
		>
			{children}
		</button>
	);
};
