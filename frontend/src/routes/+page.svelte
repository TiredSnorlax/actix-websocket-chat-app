<script lang="ts">
	import NewRoomMenu from '$lib/components/rooms/NewRoomMenu.svelte';
	import Rooms from '$lib/components/rooms/Rooms.svelte';
	import { serverDomain } from '$lib/helper/connect';
	import { onMount } from 'svelte';

	interface IRoom {
		name: string;
		id: string;
	}

	let rooms: IRoom[] = [];

	const getRooms = async () => {
		await fetch(serverDomain + 'list')
			.then((res) => res.json())
			.then((data) => {
				console.log(data);
				rooms = data;
			});
	};

	onMount(() => {
		getRooms();
	});
</script>

<div class="page">
	<h1>Some title</h1>
	<h2>Join a room</h2>
	<Rooms {rooms} />
		<NewRoomMenu />
</div>

<style>
	h2 {
		width: 100%;
	}
	.page {
		height: 100vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		gap: 1rem;

		padding: 1.5rem;
	}

</style>
