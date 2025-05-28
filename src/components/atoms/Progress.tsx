import { memo, useCallback, useEffect } from 'react';

interface InputProps {
	value: number;
	max: number;
	onMouseDown: (e?: React.MouseEvent<HTMLInputElement>) => void;
	onMouseUp: (e?: React.MouseEvent<HTMLInputElement>) => void;
	onInput: (e?: React.FormEvent<HTMLInputElement>) => void;
	className?: string;
	style?: React.CSSProperties;
}

// 创建样式的 hook
const useProgressStyles = () => {
	useEffect(() => {
		const styleId = 'progress-webkit-styles';
		if (!document.getElementById(styleId)) {
			const style = document.createElement('style');
			style.id = styleId;
			style.textContent = `
				.progress-slider::-webkit-slider-thumb {
					appearance: none;
					width: 12px;
					height: 12px;
					border-radius: 50%;
					position: relative;
					top: -4px;
					background: transparent;
					border: 1px solid transparent;
					border-image: linear-gradient(var(--primary-color), var(--primary-color)) 0 fill / 4 4 4 0 / 0 0 0 99dvw;
					transition: background 0.1s ease-in-out;
				}
				.progress-slider::-webkit-slider-runnable-track {
					background-color: var(--second-text-color);
					width: 100%;
					height: 4px;
					border-radius: 4px;
				}
				.progress-slider:hover::-webkit-slider-thumb {
					background: var(--primary-color);
				}
			`;
			document.head.appendChild(style);
		}
	}, []);
};

const Progress: React.FC<InputProps> = memo(
	({ value, max, onMouseDown, onMouseUp, onInput, className = '', style }) => {
		useProgressStyles();

		const handleMouseDown = useCallback(
			(e: React.MouseEvent<HTMLInputElement>) => {
				onMouseDown?.(e);
			},
			[onMouseDown]
		);

		const handleMouseUp = useCallback(
			(e: React.MouseEvent<HTMLInputElement>) => {
				onMouseUp?.(e);
			},
			[onMouseUp]
		);

		const handleInput = useCallback(
			(e: React.FormEvent<HTMLInputElement>) => {
				onInput?.(e);
			},
			[onInput]
		);

		return (
			<input
				type="range"
				min={0}
				max={max}
				value={value}
				onMouseDown={handleMouseDown}
				onMouseUp={handleMouseUp}
				onInput={handleInput}
				className={`progress-slider outline-none cursor-pointer bg-transparent appearance-none min-h-3 overflow-hidden ${className}`}
				style={style}
			/>
		);
	}
);

Progress.displayName = 'Progress';

export default Progress;
