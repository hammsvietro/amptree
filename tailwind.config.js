/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				['amptree-bg']: '#2d2d2d',
				['amptree-surface']: '#403e3e',
				['amptree-bg']: '#2d2d2d',
				['amptree-text']: '#ffffff',
				['amptree-accent']: '#4db8ff',
				['amptree-secondary-text']: '#b0b0b0',
				['amptree-highlight']: '#ffcc80',
				['amptree-border']: '#555555',
				['amptree-error']: '#ff5f5f',
				['amptree-success']: '#64dd17',
				['amptree-panel-bg']: '#373737'
			}
		}
	},
	plugins: []
};
