import React, { useEffect, useRef, useState } from 'react';
import styles from './Popover.module.scss';

type Position = 'top' | 'bottom' | 'left' | 'right';

interface PopoverProps {
	listen: React.RefObject<HTMLElement>;
	onClose: () => void;
	children: React.ReactNode;
	style?: React.CSSProperties;
	className?: string;
	position?: Position;
}

const Popover: React.FC<PopoverProps> = ({
	listen,
	onClose,
	children,
	style = {},
	className = '',
	position = 'top',
}) => {
	const [visible, setVisible] = useState(false);
	const popoverRef = useRef<HTMLDivElement>(null);
	let timer: ReturnType<typeof setTimeout>; // 定义一个变量来存储定时器

	const calculatePosition = () => {
		if (!listen.current || !popoverRef.current) return;

		const listenRect = listen.current.getBoundingClientRect();
		const popoverRect = popoverRef.current.getBoundingClientRect();

		let top, left;

		switch (position) {
			case 'top':
				top = listenRect.top - popoverRect.height;
				left = listenRect.left + listenRect.width / 2 - popoverRect.width / 2;
				break;
			case 'bottom':
				top = listenRect.bottom;
				left = listenRect.left + listenRect.width / 2 - popoverRect.width / 2;
				break;
			case 'left':
				top = listenRect.top + listenRect.height / 2 - popoverRect.height / 2;
				left = listenRect.left - popoverRect.width;
				break;
			case 'right':
				top = listenRect.top + listenRect.height / 2 - popoverRect.height / 2;
				left = listenRect.right;
				break;
		}

		popoverRef.current.style.top = `${top}px`;
		popoverRef.current.style.left = `${left}px`;
	};

	useEffect(() => {
		const handleMouseEnter = () => {
			setVisible(true);
			setTimeout(calculatePosition, 0); // 延迟计算位置以确保popoverRect正确
		};

		const handleMouseLeave = () => {
			timer = setTimeout(() => {
				setVisible(false);
				onClose();
			}, 300); // 延迟300毫秒关闭
		};

		const handlePopoverMouseEnter = () => {
			clearTimeout(timer);
		};

		const handlePopoverMouseLeave = () => {
			timer = setTimeout(() => {
				setVisible(false);
				onClose();
			}, 300); // 延迟300毫秒关闭
		};

		const listenElement = listen.current;
		if (listenElement) {
			listenElement.addEventListener('mouseenter', handleMouseEnter);
			listenElement.addEventListener('mouseleave', handleMouseLeave);

			popoverRef.current?.addEventListener('mouseenter', handlePopoverMouseEnter);
			popoverRef.current?.addEventListener('mouseleave', handlePopoverMouseLeave);

			return () => {
				listenElement.removeEventListener('mouseenter', handleMouseEnter);
				listenElement.removeEventListener('mouseleave', handleMouseLeave);

				popoverRef.current?.removeEventListener('mouseenter', handlePopoverMouseEnter);
				popoverRef.current?.removeEventListener('mouseleave', handlePopoverMouseLeave);
			};
		}
	}, [listen, onClose, position]);

	useEffect(() => {
		const handleClickOutside = (event: MouseEvent) => {
			if (popoverRef.current && !popoverRef.current.contains(event.target as Node)) {
				setVisible(false);
				onClose();
			}
		};

		document.addEventListener('mousedown', handleClickOutside);
		return () => {
			document.removeEventListener('mousedown', handleClickOutside);
		};
	}, [onClose]);

	return (
		<>
			{visible && (
				<div
					ref={popoverRef}
					style={{ display: visible ? 'block' : 'none', position: 'absolute', ...style }}
					className={styles.popover + ' ' + className}
				>
					{children}
				</div>
			)}
		</>
	);
};

export default Popover;
