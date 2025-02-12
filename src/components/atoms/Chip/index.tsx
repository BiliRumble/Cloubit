import styles from './Chip.module.scss';

interface ChipProps {
	children: React.ReactNode | string;
	onClick?: () => void;
	onContextMenu?: () => void;
	style?: React.CSSProperties;
	className?: string;
}

/**
 * Chip 组件
 *
 * @param {React.ReactNode | string} children - Chip 内容
 * @param {() => void} onClick - 点击事件
 * @param {() => void} onContextMenu - 右键点击事件
 * @param {string} className - 自定义类名
 * @param {React.CSSProperties} style - 自定义样式
 */
const Chip: React.FC<ChipProps> = ({
	children,
	onClick,
	onContextMenu,
	className = '',
	style = {},
}) => {
	return (
		<div
			className={`${styles.chip} ${className}`}
			style={style}
			onClick={onClick}
			onContextMenu={onContextMenu}
		>
			<p>{children}</p>
		</div>
	);
};

export default Chip;
