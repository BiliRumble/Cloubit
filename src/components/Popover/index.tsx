import React from 'react';

interface PopoverProps {
	children?: React.ReactNode;
	show?: boolean;
	classNames?: string;
}

const Popover: React.FC<PopoverProps> = ({ children, show, classNames = '' }) => {
	//const [isVisible, setIsVisible] = useState<boolean>(show);

	return (
		<div className={`popover ${classNames}`}>
			{children}
			{show}
		</div>
	);
};

export default Popover;
