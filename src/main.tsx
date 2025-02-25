import ReactDOM from 'react-dom/client';
import { EventBusProvider } from './context/EventBusContext.tsx';
import { PlayerProvider } from './context/PlayerContext.tsx';
import { createPip } from './managers/PIPWindowManager.ts';
import { disableShortcuts } from './utils/init.ts';
import App from './App.tsx';
import 'virtual:uno.css';
import './styles/index.scss';

document.addEventListener('contextmenu', (e) => {
	e.preventDefault();
});
disableShortcuts();

const isPip = localStorage.getItem('isPip') == 'true';
const isWindowPath = location.pathname.startsWith('/windows/');
if (isPip) createPip(true);

const Player: React.FC<{ children: React.ReactNode }> = ({ children }) => {
	if (isWindowPath) return <>{children}</>;
	console.debug('PlayerProvider');
	return <PlayerProvider>{children}</PlayerProvider>;
};

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<EventBusProvider>
		<Player>
			<App />
		</Player>
	</EventBusProvider>
);
