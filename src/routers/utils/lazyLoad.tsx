import React, { Suspense } from 'react';

/**
 * 路由懒加载
 * @param Component 需要访问的组件
 * @return element
 */
const lazyLoad = (Component: React.LazyExoticComponent<any>): React.ReactNode => {
	return (
		<Suspense
			fallback={
				<div
					style={{
						width: '100%',
						height: '100%',
						display: 'flex',
						justifyContent: 'center',
						alignItems: 'center',
					}}
				>
					加载中...
				</div>
			}
		>
			<Component />
		</Suspense>
	);
};

export default lazyLoad;
