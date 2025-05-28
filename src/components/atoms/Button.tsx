import { memo } from 'react';

interface ButtonProps {
	title?: string;
	children: React.ReactNode;
	onClick?: () => void;
	className?: string;
	style?: React.CSSProperties;
	ref?: React.RefObject<HTMLButtonElement>;
	variant?: 'default' | 'icon';
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
 * @param {'default' | 'icon'} variant - 按钮变体
 */
export const Button: React.FC<ButtonProps> = memo(
	({ title, children, onClick, className = '', style, ref, variant = 'default' }) => {
		const baseClasses =
			variant === 'icon'
				? 'border-none flex flex-row items-center flex-nowrap text-1.3rem bg-transparent p-0 cursor-pointer text-[var(--third-text-color)] transition-colors duration-100 ease-in-out hover:text-[var(--primary-color)] focus:outline-none [&:not(:last-child)]:mr-4 [&_i]:block'
				: 'px-4 py-2 bg-[var(--button-bg-color)] text-[var(--text-color)] border-none rounded-6px outline-none shadow-[var(--shadow)] transition-colors duration-100 ease-in-out hover:bg-[var(--button-hover-bg-color)] focus:bg-[var(--button-focus-bg-color)]';

		return (
			<button
				className={`${baseClasses} ${className}`}
				title={title}
				onClick={onClick}
				style={style}
				ref={ref}
			>
				{children}
			</button>
		);
	}
);

Button.displayName = 'Button';

export default Button;
