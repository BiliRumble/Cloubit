import { memo, RefObject, useCallback, useEffect, useRef, useState } from 'react';

interface LazyImageProps {
	src: string;
	alt: string;
	placeholder?: string;
	className?: string;
	onError?: () => void;
	rounded?: string;
	srcSet?: string;
	sizes?: string;
}

const useIntersectionObserver = (ref: RefObject<Element>, callback: () => void) => {
	const cachedCallback = useCallback(callback, [callback]);

	useEffect(() => {
		const observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						cachedCallback();
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
	}, [ref, cachedCallback]);
};

const LazyImage: React.FC<LazyImageProps> = memo(
	({
		src,
		alt,
		placeholder = 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkqAcAAIUAgUW0RjgAAAAASUVORK5CYII=',
		className = '',
		onError,
		rounded = '8px',
		srcSet,
		sizes,
	}) => {
		const [isLoaded, setIsLoaded] = useState(false);
		const [error, setError] = useState(false);
		const imgRef = useRef<HTMLImageElement>(null);
		const imgInstanceRef = useRef<HTMLImageElement | null>(null);

		const loadImage = useCallback(() => {
			if (imgInstanceRef.current) {
				imgInstanceRef.current.onload = null;
				imgInstanceRef.current.onerror = null;
			}

			const img = new Image();
			imgInstanceRef.current = img;

			img.src = src;
			if (srcSet) img.srcset = srcSet;
			if (sizes) img.sizes = sizes;

			img.onload = () => {
				setIsLoaded(true);
				setError(false);
			};

			img.onerror = () => {
				setError(true);
				onError?.();
				imgInstanceRef.current = null;
			};
		}, [src, srcSet, sizes, onError]);

		useIntersectionObserver(imgRef, loadImage);

		return (
			<div
				className={`relative bg-[var(--button-bg-color)] ${
					!isLoaded && !error
						? "before:content-[''] before:absolute before:top-0 before:w-full before:h-full before:bg-gradient-to-r before:from-[var(--button-hover-bg-color)] before:to-[var(--button-focus-bg-color)] before:animate-pulse before:z-2"
						: ''
				} ${
					error
						? "after:content-['⚠️'] after:absolute after:top-1/2 after:left-1/2 after:transform after:-translate-x-1/2 after:-translate-y-1/2 after:text-[#e74c3c] after:bg-[rgba(255,255,255,0.9)] after:px-4 after:py-2 after:rounded after:z-1"
						: ''
				} ${className}`}
			>
				<img
					ref={imgRef}
					src={isLoaded && !error ? src : placeholder}
					srcSet={isLoaded && !error ? srcSet : ''}
					sizes={sizes}
					alt={alt}
					className={`w-full min-w-1px h-full object-cover rounded-${rounded} transition-all duration-500 ${
						isLoaded ? 'opacity-100' : 'opacity-0'
					}`}
					loading="lazy"
				/>
			</div>
		);
	}
);

LazyImage.displayName = 'LazyImage';

export default LazyImage;
