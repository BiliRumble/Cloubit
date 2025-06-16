import { RouteDefinition } from '@solidjs/router';
import { lazy } from 'solid-js';

const Home = lazy(() => import('../pages/Home'));

const routes: RouteDefinition[] = [
	{
		path: '/',
		component: Home,
	},
];

export default routes;
