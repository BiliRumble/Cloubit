import { memo, useCallback } from 'react';

interface ChipProps {
	children: React.ReactNode | string;
	onClick?: () => void;
	onContextMenu?: () => void;
	style?: React.CSSProperties;
	className?: string;
	ref?: React.RefObject<HTMLDivElement>;
}

/**
 * Chip 组件
 *
 * @param {React.ReactNode | string} children - Chip 内容
 * @param {() => void} onClick - 点击事件
 * @param {() => void} onContextMenu - 右键点击事件
 * @param {string} className - 自定义类名
 * @param {React.CSSProperties} style - 自定义样式
 * @returns {JSX.Element} - Chip 组件
 */
const Chip: React.FC<ChipProps> = memo(
	({ children, onClick, onContextMenu, className = '', style, ref }) => {
		const handleClick = useCallback(() => {
			onClick?.();
		}, [onClick]);

		const handleContextMenu = useCallback(() => {
			onContextMenu?.();
		}, [onContextMenu]);

		return (
			<div
				className={`inline-block max-w-full px-2 py-1 text-[var(--text-color)] bg-[var(--hr-color)] rounded-8px cursor-pointer transition-all duration-200 ease-in-out shadow-[var(--shadow)] hover:shadow-[var(--shadow-hover)] focus:shadow-[var(--shadow-focus)] [&:not(:last-child)]:mr-2 hover:[&>p]:text-[var(--primary-color)] ${className}`}
				style={style}
				onClick={handleClick}
				onContextMenu={handleContextMenu}
				ref={ref}
			>
				<p className="whitespace-nowrap overflow-hidden text-ellipsis relative -top-0.5 transition-colors duration-200 ease-in-out">
					{children}
				</p>
			</div>
		);
	}
);

Chip.displayName = 'Chip';

export default Chip;
