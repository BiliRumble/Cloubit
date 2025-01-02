import { useRoutes } from 'react-router-dom';
import { generateRouteConfig } from './utils/fileRouter';

const constantRoutes = generateRouteConfig();

// 创建一个可以被 React 应用程序使用的路由实例
const router = () => {
	const routes = useRoutes(constantRoutes);
	return routes;
};

export default router;
