import { memo } from 'react';

interface ButtonProps {
	title?: string;
	icon: string;
	onClick?: () => void;
	className?: string;
	style?: React.CSSProperties;
	ref?: React.RefObject<HTMLButtonElement>;
}

/**
 * IconButton 组件
 *
 * @param {string} title - 按钮标题
 * @param {string} icon - 按钮图标, 比如i-solar-microphone-linear
 * @param {function} onClick - 点击事件
 * @param {string} className - 自定义类名
 * @param {React.CSSProperties} style - 自定义样式
 * @param {React.RefObject<HTMLButtonElement>} ref - 按钮引用
 */
export const IconButton: React.FC<ButtonProps> = memo(
	({ title, icon, onClick, className = '', style, ref }) => {
		return (
			<button
				className={`border-none flex flex-row items-center flex-nowrap text-1.3rem bg-transparent p-0 cursor-pointer text-[var(--third-text-color)] transition-colors duration-100 ease-in-out hover:text-[var(--primary-color)] focus:outline-none [&:not(:last-child)]:mr-4 outline-none ${className}`}
				title={title}
				onClick={onClick}
				style={style}
				ref={ref}
			>
				<i className={`${icon} block`} />
			</button>
		);
	}
);

IconButton.displayName = 'IconButton';

export default IconButton;
