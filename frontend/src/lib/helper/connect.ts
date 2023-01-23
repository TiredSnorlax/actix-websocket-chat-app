export const serverDomain = 'http://127.0.0.1:8080/';
export const clientDomain = 'http://localhost:5173/';

export const getSession = async () => {
	let _session = '';
	await fetch(serverDomain + 'connect', {
		method: 'post',
		headers: {
			Accept: 'application/json',
			'Content-Type': 'application/json'
		}
	})
		.then((res) => res.json())
		.then((data) => {
			console.log(data);
			localStorage.setItem('session', data.session);
			_session = data.session;
		})
		.catch((err) => {
			console.log(err);
		});

	return _session;
};
