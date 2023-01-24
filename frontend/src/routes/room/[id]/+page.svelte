<script lang="ts">
	import MsgContainer from '$lib/components/messages/MsgContainer.svelte';
	import MsgInput from '$lib/components/messages/MsgInput.svelte';
	import { getSession } from '$lib/helper/connect';
	import { onDestroy, onMount } from 'svelte';
	import { page } from '$app/stores';
	import type { PageData } from './$types';

	export let data: PageData;

	let socket: WebSocket | null;

	let msgList: string[] = [];
	let newText = '';

	const connect = async () => {
		let session = localStorage.getItem('session');
		console.log(session);

		if (!session) {
			session = await getSession();
		}

		if (socket) return;
		const { location } = window;

		const proto = location.protocol.startsWith('https') ? 'wss' : 'ws';
		const params = new URLSearchParams();
		params.append('session', session!);
		const wsUri = `${proto}://127.0.0.1:8080/ws/${$page.params.id}?${params.toString()}`;

		socket = new WebSocket(wsUri);

		socket.onopen = () => {
			console.log('Connected');
		};

		socket.onclose = () => {
			console.log('Disconnected');
		};

		socket.onmessage = (ev) => {
			console.log(`Received: ${ev.data}`);
			msgList = [...msgList, ev.data];
		};
	};

	const disconnect = () => {
		if (!socket) return;
		socket.close();
		socket = null;
	};

	const send = () => {
		if (!socket) return;
		socket.send(newText);
		newText = '';
	};

	onMount(() => {
		connect();
	});

	onDestroy(() => {
		disconnect();
	});
</script>

<div class="page">
	<h1>Room: {data.room.name}</h1>
	<MsgContainer {msgList} />
	<MsgInput bind:newText {send} />
</div>

<style>
  h1 {
    padding: 1rem;
  }

	.page {
		height: 100vh;

		display: flex;
		flex-direction: column;
		justify-content: space-between;
		gap: 1rem;
	}

</style>
