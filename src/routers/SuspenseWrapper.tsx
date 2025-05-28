import { ReactNode, Suspense } from 'react';
import LoadingProgress from '@/components/organisms/LoadingProgress';

interface SuspenseWrapperProps {
	children: ReactNode;
}

const SuspenseWrapper = ({ children }: SuspenseWrapperProps) => {
	return <Suspense fallback={<LoadingProgress />}>{children}</Suspense>;
};

export default SuspenseWrapper;
