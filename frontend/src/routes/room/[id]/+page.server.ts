import { serverDomain } from '$lib/helper/connect';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const res = await fetch(serverDomain + 'room/' + params.id + '/data', {
		method: 'POST'
	});

	const data = await res.json();

	console.log(data);

	return {
		room: data
	};
};
