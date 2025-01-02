import { BrowserRouter } from 'react-router-dom';
import Navbar from './components/Navbar';
import Sidebar from './components/Sidebar';
import Router from './routers';
import styles from './styles/layout.module.scss';
import PlayBar from './components/PlayBar';

const App = () => {
	return (
		<main className={styles.layout}>
			<Sidebar className={styles.sidebar} />
			<div className={styles.content}>
				<Navbar className={styles.header} />
				<div className={styles.page_content}>
					<BrowserRouter>
						<Router />
					</BrowserRouter>
				</div>
			</div>
			<PlayBar className={styles.bottom_bar} />
		</main>
	);
};

export default App;
