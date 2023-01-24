<script lang="ts">
	import { clientDomain } from '$lib/helper/connect';
	import type { IRoom } from '$lib/types';
	import { slide } from 'svelte/transition';

	export let room: IRoom;
	export let index: number;

    console.log(room)

	let open = false;
</script>

<div class="item">
	<div class="upper">
		<a href="{clientDomain}room/{room.id}">{index + 1}. {room.name}</a>
		<button on:click={() => (open = !open)}>
			<span class="material-icons-outlined expand" class:open> expand_more </span>
		</button>
	</div>
	{#if open}
		<div transition:slide class="lower">
			<p>{room.description}</p>
			<a class="enterBtn" href="{clientDomain}room/{room.id}">Enter</a>
		</div>
	{/if}
</div>

<style>
	button {
		background: none;
		outline: none;
		border: none;

		cursor: pointer;
	}

	a {
		text-decoration: none;
		color: initial;
		font-size: 1.2rem;
		font-weight: bold;
	}

	a:hover {
		text-decoration: underline;
	}

	.item {
		background: rgb(240, 240, 240);
		width: 100%;
		border-radius: 5px;
		border: 1px solid grey;
		padding: 0.5rem 1rem;

		display: flex;
		flex-direction: column;
	}

	.item .upper {
		display: flex;
		justify-content: space-between;
	}

	.item .lower {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.lower p {
		padding: 1rem;
	}

	.item button .expand {
		transition: 0.3s;
	}

	.item button .expand.open {
		transform: rotate(180deg);
	}

	.enterBtn {
		background: blueviolet;
		color: white;
		padding: 0.5rem 1rem;
		border-radius: 5px;

		align-self: flex-end;

		font-size: 1rem;
		text-decoration: none !important;
	}

</style>
