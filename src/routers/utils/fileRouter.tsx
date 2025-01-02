import { set } from 'lodash-es';
import { ComponentType, lazy, Suspense } from 'react';
import { RouteObject } from 'react-router-dom';

/**
 * 生成文件路径配置
 */
function generatePathConfig(): Record<string, any> {
	const modules = import.meta.glob('/src/pages/**/*.{ts,tsx}');
	const pathConfig = {};
	let notFoundRoute;

	Object.keys(modules).forEach((filePath) => {
		if (filePath.includes('[...notFound]')) {
			notFoundRoute = modules[filePath];
			return;
		}

		const routePath = filePath
			.replace('/src/pages/', '')
			.replace(/.tsx?/, '')
			.replace(/\$\[([\w-]+)]/, ':$1')
			.replace(/\$([\w-]+)/, '$1')
			.split('/');

		set(pathConfig, routePath, modules[filePath]);
	});

	if (notFoundRoute) {
		pathConfig['*'] = notFoundRoute;
	}

	return pathConfig;
}

/**
 * 为动态 import 包裹 lazy 和 Suspense
 */
function wrapSuspense(importer: () => Promise<{ default: ComponentType }>) {
	if (!importer) {
		return undefined;
	}
	// 使用 React.lazy 包裹 () => import() 语法
	const Component = lazy(importer);
	// 结合 Suspense，这里可以自定义 loading 组件
	return (
		<Suspense fallback={null}>
			<Component />
		</Suspense>
	);
}

/**
 * 将文件路径配置映射为 react-router 路由
 */
function mapPathConfigToRoute(cfg: Record<string, any>): RouteObject[] {
	// route 的子节点为数组
	return Object.entries(cfg).map(([routePath, child]) => {
		// () => import() 语法判断
		if (typeof child === 'function') {
			// 等于 index 则映射为当前根路由
			const isIndex = routePath === 'index';
			return {
				index: isIndex,
				path: isIndex ? undefined : routePath,
				// 转换为组件
				element: wrapSuspense(child),
			};
		}
		// 否则为目录，则查找下一层级
		const { $, ...rest } = child;
		return {
			path: routePath,
			// layout 处理
			element: wrapSuspense($),
			// 递归 children
			children: mapPathConfigToRoute(rest),
		};
	});
}

/**
 * 生成路由配置
 */
export function generateRouteConfig(): RouteObject[] {
	const { $, ...pathConfig } = generatePathConfig();
	// 提取跟路由的 layout
	return [
		{
			path: '/',
			element: wrapSuspense($),
			children: mapPathConfigToRoute(pathConfig),
		},
	];
}
