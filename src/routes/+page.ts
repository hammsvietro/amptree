// since there's no dynamic data here, we can prerender
// it so that it gets served as a static asset in production

export const prerender = true;

export const load = () => {
	return {
		items: [
			{ id: 1, name: 'Tosin time', length: 280 },
			{ id: 2, name: 'Maynard madness', length: 220 },
			{ id: 3, name: 'The Meshugging', length: 213 },
			{ id: 4, name: 'C.B.T', length: 480 }
		]
	};
};
