import { BrowserRouter } from 'react-router-dom';
import Navbar from './components/Layout/Navbar';
import PlayBar from './components/Layout/PlayBar';
import Sidebar from './components/Layout/Sidebar';
import { useTheme } from './hooks/useTheme';
import Router from './routers';
import styles from './styles/layout.module.scss';

const App = () => {
	const isWindowPath = location.pathname.startsWith('/windows/');
	useTheme();

	return (
		<BrowserRouter>
			{!isWindowPath ? (
				<main className={styles.layout}>
					<Sidebar className={styles.sidebar} />
					<div className={styles.content}>
						<Navbar className={styles.header} />
						<main className={styles.page_content}>
							<Router />
						</main>
					</div>
					<PlayBar className={styles.bottom_bar} />
				</main>
			) : (
				<Router />
			)}
		</BrowserRouter>
	);
};

export default App;
