import { render } from 'solid-js/web';
import { Router } from '@solidjs/router';
import routes from '@/router';
import '@/styles/index.css';
import 'virtual:uno.css';

render(() => <Router>{routes}</Router>, document.getElementById('root') as HTMLElement);
