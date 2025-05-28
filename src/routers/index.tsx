import { lazy } from 'react';
import { createBrowserRouter } from 'react-router-dom';
import Layout from '@/components/templates/Layout';
import SuspenseWrapper from './SuspenseWrapper';

const Home = lazy(() => import('../pages/index'));
const Search = lazy(() => import('../pages/search/[keyword]'));
const Playlist = lazy(() => import('../pages/playlist/[id]'));
const DailyPlaylist = lazy(() => import('../pages/playlist/daily'));
const LikeSongs = lazy(() => import('../pages/playlist/like'));
const AlbumDetail = lazy(() => import('../pages/album/[id]'));
const SongComment = lazy(() => import('../pages/comment/song/[id]'));

const router = createBrowserRouter([
	{
		path: '/',
		element: <Layout />,
		children: [
			{
				index: true,
				element: (
					<SuspenseWrapper>
						<Home />
					</SuspenseWrapper>
				),
			},
			{
				path: '/search/:keyword',
				element: (
					<SuspenseWrapper>
						<Search />
					</SuspenseWrapper>
				),
			},
			{
				path: '/playlist/:id',
				element: (
					<SuspenseWrapper>
						<Playlist />
					</SuspenseWrapper>
				),
			},
			{
				path: '/playlist/daily',
				element: (
					<SuspenseWrapper>
						<DailyPlaylist />
					</SuspenseWrapper>
				),
			},
			{
				path: '/playlist/like',
				element: (
					<SuspenseWrapper>
						<LikeSongs />
					</SuspenseWrapper>
				),
			},
			{
				path: '/album/:id',
				element: (
					<SuspenseWrapper>
						<AlbumDetail />
					</SuspenseWrapper>
				),
			},
			{
				path: '/comment/song/:id',
				element: (
					<SuspenseWrapper>
						<SongComment />
					</SuspenseWrapper>
				),
			},
			{ path: '/*', element: <div>404</div> },
		],
	},
]);

export default router;
