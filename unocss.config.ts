import { defineConfig, presetIcons, presetUno } from 'unocss';

export default defineConfig({
	theme: {
		colors: {},
	},
	presets: [
		presetUno({
			dark: {
				// 启用 CSS 变量模式的深色主题
				light: '[data-theme="light"]',
				dark: '[data-theme="dark"]',
			},
		}),
		presetIcons({
			extraProperties: {
				display: 'inline-block',
				'vertical-align': 'middle',
				fill: 'currentColor',
			},
		}),
	],
});
