import { RefObject, useEffect, useRef, useState } from 'react';
import styles from './LazyImage.module.scss';

interface LazyImageProps {
	src: string;
	alt: string;
	placeholder?: string;
	className?: string;
	onError?: () => void;
	srcSet?: string;
	sizes?: string;
	maxRetries?: number;
}

const useIntersectionObserver = (ref: RefObject<Element>, callback: () => void) => {
	useEffect(() => {
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						callback();
						observer.unobserve(entry.target);
					}
				});
			},
			{
				rootMargin: '200px',
				threshold: 0.01,
			}
		);

		const currentRef = ref.current;
		if (currentRef) observer.observe(currentRef);

		return () => {
			if (currentRef) observer.unobserve(currentRef);
		};
	}, [ref, callback]);
};

const LazyImage: React.FC<LazyImageProps> = ({
	src,
	alt,
	placeholder = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkqAcAAIUAgUW0RjgAAAAASUVORK5CYII=',
	className = '',
	onError,
	srcSet,
	sizes,
	maxRetries = 3,
}) => {
	const [isLoaded, setIsLoaded] = useState(false);
	const [error, setError] = useState(false);
	const [retryCount, setRetryCount] = useState(0);
	const imgRef = useRef<HTMLImageElement>(null);

	const loadImage = () => {
		const img = new Image();
		img.src = src;
		if (srcSet) img.srcset = srcSet;
		if (sizes) img.sizes = sizes;

		img.onload = () => {
			setIsLoaded(true);
			setError(false);
		};

		img.onerror = () => {
			if (retryCount < maxRetries) {
				setTimeout(() => {
					setRetryCount((c) => c + 1);
					loadImage();
				}, 1000);
			} else {
				setError(true);
				onError?.();
			}
		};
	};

	useIntersectionObserver(imgRef, loadImage);

	const handleRetry = () => {
		if (error) {
			setRetryCount(0);
			setError(false);
			loadImage();
		}
	};

	return (
		<div
			className={`${styles.lazyImg} ${className} ${
				!isLoaded && !error ? styles.loading : ''
			} ${error ? styles.error : ''}`}
			onClick={handleRetry}
		>
			<img
				ref={imgRef}
				src={isLoaded && !error ? src : placeholder}
				srcSet={isLoaded && !error ? srcSet : ''}
				sizes={sizes}
				alt={alt}
				className={`${styles.lazyImg__image} ${isLoaded ? styles.loaded : ''}`}
				loading="lazy"
			/>
		</div>
	);
};

export default LazyImage;
