import styles from './Button.module.scss';

interface ButtonProps {
	title?: string;
	icon: string;
	onClick?: () => void;
	className?: string;
	style?: React.CSSProperties;
    ref?: React.RefObject<HTMLButtonElement>;
}

/**
 * Button 组件
 *
 * @param {string} title - 按钮标题
 * @param {string} icon - 按钮图标, 比如i-solar-microphone-linear
 * @param {function} onClick - 点击事件
 * @param {string} className - 自定义类名
 * @param {React.CSSProperties} style - 自定义样式
 * @param {React.RefObject<HTMLButtonElement>} ref - 按钮引用
 */
export const IconButton: React.FC<ButtonProps> = ({
	title,
	icon,
	onClick,
	className = '',
	style,
    ref,
}) => {
	return (
		<button
			className={styles.button__icon + ' ' + className}
			title={title}
			onClick={onClick}
			style={style}
            ref={ref}
		>
			<i className={icon} />
		</button>
	);
};
