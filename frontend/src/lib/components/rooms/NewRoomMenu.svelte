<script lang="ts">
	import { serverDomain } from '$lib/helper/connect';
	import { fade, scale, slide } from 'svelte/transition';

	let open = false;

	let newRoomName = '';
	let newRoomDescription = '';
	const newRoom = async () => {
		await fetch(serverDomain + 'room/new', {
			method: 'post',
			headers: {
				Accept: 'application/json',
				'Content-Type': 'application/json'
			},
			//make sure to serialize your JSON body
			body: JSON.stringify({
				name: newRoomName,
				description: newRoomDescription
			})
		}).then((res) => {
			if (res.redirected) {
				window.location.href = res.url;
			}
		});
	};
</script>

{#if open}
	<div class="bg" on:click|self={() => (open = false)} on:keydown={() => {}} transition:fade>
		<div class="menu" transition:slide>
			<h2>New Room</h2>
			<div>
				<p>Room Name</p>
				<input type="text" bind:value={newRoomName} />
			</div>
			<div>
				<p>Description (optional)</p>
				<textarea bind:value={newRoomDescription} rows="3" />
			</div>
			<button class="createBtn" on:click={newRoom} disabled={newRoomName.length === 0}
				>Create</button
			>
		</div>
	</div>
{:else}
	<button transition:scale on:click={() => (open = true)} class="openBtn"
		>or create a new one</button
	>
{/if}

<style>
	input,
	textarea {
		background: rgb(240, 240, 240);
		border: 1px solid transparent;
		outline: none;
		resize: none;
		padding: 0.5rem;

		border-radius: 5px;
	}

	input:focus,
	textarea:focus {
		background: rgb(250, 250, 250);
		border: 1px solid grey;
	}

	.openBtn {
		background: none;
		outline: none;
		border: none;

		font-size: 1.5rem;
		font-weight: bold;

		cursor: pointer;
	}

    .openBtn:hover {
        color: grey;
    }

	.bg {
		background: rgba(0, 0, 0, 0.2);
		position: fixed;
		inset: 0;

		display: flex;
		justify-content: center;
		align-items: center;
	}

	.menu {
		padding: 2rem;
		background: white;

		display: flex;
		flex-direction: column;
		align-items: center;
		border-radius: 5px;

		gap: 1rem;
	}

	.menu div {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: flex-start;
		gap: 0.5rem;
	}

	.menu div textarea,
	.menu div input {
		flex: 1 1 auto;
		min-width: 0;
	}

	.createBtn {
		background: green;
		color: white;
		padding: 0.5rem 1rem;
		border-radius: 0.5rem;

		font-size: 1rem;
		border: none;
		outline: none;

		transition: 0.3s;
	}

	.createBtn:disabled {
		background: transparent;
		color: grey;
	}
</style>
