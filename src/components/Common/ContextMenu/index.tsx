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
}

const ContextMenu: React.FC<ContextMenuProps> = ({ items, children, className }) => {
	const [isVisible, setIsVisible] = useState(false);
	const [position, setPosition] = useState({ x: 0, y: 0 });
	const menuRef = useRef<HTMLDivElement>(null);

	// 处理右键点击事件
	const handleContextMenu = (e: React.MouseEvent) => {
		e.preventDefault();
		setIsVisible(true);
		setPosition({ x: e.clientX, y: e.clientY });
	};

	// 关闭菜单
	const closeMenu = () => setIsVisible(false);

	// 全局点击监听
	useEffect(() => {
		if (!isVisible) return;

		const handleClickOutside = (e: MouseEvent) => {
			if (menuRef.current && !menuRef.current.contains(e.target as Node)) {
				closeMenu();
			}
		};

		document.addEventListener('click', handleClickOutside);
		return () => {
			document.removeEventListener('click', handleClickOutside);
		};
	}, [isVisible]);

	// 调整菜单位置防止溢出视口
	useEffect(() => {
		if (!isVisible || !menuRef.current) return;

		const { innerWidth, innerHeight } = window;
		const { width, height } = menuRef.current.getBoundingClientRect();

		let x = position.x;
		let y = position.y;

		if (x + width > innerWidth) x = innerWidth - width - 10;
		if (y + height > innerHeight) y = innerHeight - height - 10;

		setPosition({ x, y });
	}, [isVisible, position]);

	return (
		<div className={`${styles.contextMenu} ${className}`} onContextMenu={handleContextMenu}>
			{children}
			{isVisible && (
				<div
					ref={menuRef}
					className={styles.contextMenu__box}
					style={{
						left: position.x,
						top: position.y,
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
