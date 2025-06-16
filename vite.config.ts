import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';
import UnoCSS from 'unocss/vite';

// @ts-expect-error process is a nodejs global, so shut up
const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
	plugins: [UnoCSS(), solid()],

	clearScreen: false,
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
					protocol: 'ws',
					host,
					port: 1421,
				}
			: undefined,
		watch: {
			ignored: ['**/src-tauri/**'],
		},
	},
	resolve: {
		alias: {
			'@': '/src',
		},
	},
}));
