import React from 'react';
import styles from './Modal.module.scss';

interface ModalProps {
	isOpen: boolean;
	hasCard?: boolean;
	onClose: () => void;
	children: React.ReactNode;
	style?: React.CSSProperties;
	className?: string;
}

const Modal: React.FC<ModalProps> = ({
	isOpen,
	hasCard = true,
	onClose,
	style = {},
	className = '',
	children,
}) => {
	return (
		<div
			className={styles.modal + (isOpen ? ' ' + styles.modal__open : '') + ' ' + className}
			style={style}
		>
			<div className={styles.modal__mask} onClick={onClose} />
			{hasCard ? (
				<div className={styles.modal__content} onClick={(e) => e.stopPropagation()}>
					<button className={styles.closeButton} onClick={onClose}>
						&times;
					</button>
					{isOpen ? children : null}
				</div>
			) : (
				<>{isOpen ? children : null}</>
			)}
		</div>
	);
};

export default Modal;
