import ReactDOM from 'react-dom/client';
import { PlayerProvider } from './context/PlayerContext.tsx';
import { createPip } from './managers/PIPWindowManager.ts';
import App from './App.tsx';
import 'virtual:uno.css';
import './styles/index.scss';

document.addEventListener('contextmenu', (e) => {
	e.preventDefault();
});

const isPip = localStorage.getItem('isPip') == 'true';
if (isPip) createPip(true);

ReactDOM.createRoot(document.getElementById('root')!).render(
	<PlayerProvider>
		<App />
	</PlayerProvider>
);
