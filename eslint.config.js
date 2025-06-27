import js from '@eslint/js';
import tsParser from '@typescript-eslint/parser';
import tsPlugin from '@typescript-eslint/eslint-plugin';
import solidPlugin from 'eslint-plugin-solid';

export default [
	js.configs.recommended,
	{
		files: ['**/*.{ts,tsx}'],
		languageOptions: {
			parser: tsParser,
			parserOptions: {
				ecmaVersion: 'latest',
				sourceType: 'module',
				project: './tsconfig.json',
				jsx: true,
				jsxImportSource: 'solid-js',
			},
			globals: {
				document: 'readonly',
				window: 'readonly',
				HTMLElement: 'readonly',
				Event: 'readonly',
			},
		},
		plugins: {
			'@typescript-eslint': tsPlugin,
			solid: solidPlugin,
		},
		rules: {
			...tsPlugin.configs.recommended.rules,
			...solidPlugin.configs.typescript.rules,

			'@typescript-eslint/no-explicit-any': 'warn',
			'solid/reactivity': 'warn',
			'solid/no-innerhtml': 'error',
			'solid/jsx-no-undef': 'error',
			'no-console': 'warn',
		},
	},
	{
		settings: {
			'import/resolver': {
				typescript: {},
			},
			react: {
				version: 'detect',
			},
		},
	},
	{
		ignores: [
			'dist/**',
			'public/**',
			'node_modules/**',
			'**/*.d.ts',
			'*.cjs',
			'*.config.ts',
			'*.config.js',
			'src/**',
			'target/**',
		],
	},
];
