import { useEffect, useRef, useState } from 'react';
import styles from './ContextMenu.module.scss';

export interface MenuItem {
	label: React.ReactNode | string;
	onClick: () => void;
	disabled?: boolean;
}

interface ContextMenuProps {
	items: MenuItem[];
	children: React.ReactNode;
	className?: string;
	onClick?: (e?: React.MouseEvent) => void;
	style?: React.CSSProperties;
}

let globalCloseMenu: (() => void) | null = null;

const ContextMenu: React.FC<ContextMenuProps> = ({
	items,
	children,
	onClick,
	className = '',
	style,
}) => {
	const [isVisible, setIsVisible] = useState(false);
	const [position, setPosition] = useState({ x: 0, y: 0 });
	const menuRef = useRef<HTMLDivElement>(null);

	const handleContextMenu = (e: React.MouseEvent) => {
		e.preventDefault();
		// 关闭其他菜单
		if (globalCloseMenu) globalCloseMenu();
		// 注册当前菜单
		globalCloseMenu = closeMenu;
		setIsVisible(true);
		setPosition({ x: e.clientX, y: e.clientY });
	};

	const closeMenu = () => {
		setIsVisible(false);
		// 清除全局引用
		if (globalCloseMenu === closeMenu) globalCloseMenu = null;
	};

	useEffect(() => {
		if (!isVisible) return;

		const handleClickOutside = (e: MouseEvent) => {
			if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
				closeMenu();
			}
		};

		document.addEventListener('click', handleClickOutside);
		return () => document.removeEventListener('click', handleClickOutside);
	}, [isVisible]);

	useEffect(() => {
		if (!isVisible || !menuRef.current) return;

		const { innerWidth, innerHeight } = window;
		const rect = menuRef.current.getBoundingClientRect();

		let x = position.x;
		let y = position.y;

		// 水平边界处理
		if (x + rect.width > innerWidth) x = innerWidth - rect.width - 10;
		if (x < 10) x = 10;

		// 垂直边界处理
		if (y + rect.height > innerHeight) y = innerHeight - rect.height - 10;
		if (y < 10) y = 10;

		setPosition({ x, y });
	}, [isVisible, position]);

	useEffect(() => {
		if (!isVisible) return;

		const handleClickOutside = (e: MouseEvent) => {
			if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
				closeMenu();
			}
		};

		// 新增滚动监听
		const handleScroll = () => {
			closeMenu();
		};

		document.addEventListener('click', handleClickOutside);
		window.addEventListener('scroll', handleScroll, true); // 使用捕获阶段确保及时触发

		return () => {
			document.removeEventListener('click', handleClickOutside);
			window.removeEventListener('scroll', handleScroll, true);
		};
	}, [isVisible]);

	const click = (e: React.MouseEvent) => {
		// 点击子内容而不是菜单时执行onClick
		if (!isVisible && onClick) {
			onClick(e);
			closeMenu();
		}
	};

	return (
		<div
			className={`${styles.contextMenu} ${className}`}
			onClick={click}
			onContextMenu={handleContextMenu}
		>
			{children}
			{isVisible && (
				<div
					ref={menuRef}
					className={styles.contextMenu__box}
					style={{
						left: position.x,
						top: position.y,
						// 防止transform导致的溢出
						transform: 'none',
						...style,
					}}
				>
					{items.map((item, index) => (
						<div
							key={index}
							className={`${styles.contextMenu__box__item} ${item.disabled ? styles.disabled : ''}`}
							onClick={() => {
								if (!item.disabled) {
									item.onClick();
									closeMenu();
								}
							}}
						>
							{item.label}
						</div>
					))}
				</div>
			)}
		</div>
	);
};

export default ContextMenu;
