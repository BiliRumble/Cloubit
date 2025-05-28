import { useEffect, useState } from 'react';

const LoadingProgress = () => {
	const [progress, setProgress] = useState(0);

	useEffect(() => {
		const timer = setInterval(() => {
			setProgress((prev) => {
				if (prev >= 90) return prev;
				return prev + Math.random() * 15;
			});
		}, 200);

		return () => clearInterval(timer);
	}, []);

	return (
		<div className="fixed top-0 left-0 w-full h-1 bg-gray-200 z-50">
			<div
				className="h-full bg-[var(--primary-color)]/500 transition-all duration-300 ease-out"
				style={{ width: `${progress}%` }}
			/>
		</div>
	);
};

export default LoadingProgress;
