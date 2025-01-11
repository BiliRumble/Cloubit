import React from 'react';
import styles from './Modal.module.scss';

interface ModalProps {
	isOpen: boolean;
	onClose: () => void;
	children: React.ReactNode;
}

const Modal: React.FC<ModalProps> = ({ isOpen, onClose, children }) => {
	if (!isOpen) return null;

	return (
		<div className={styles.modal} onClick={onClose}>
			<div className={styles.modal__content} onClick={(e) => e.stopPropagation()}>
				<button className={styles.closeButton} onClick={onClose}>
					&times;
				</button>
				{children}
			</div>
		</div>
	);
};

export default Modal;
