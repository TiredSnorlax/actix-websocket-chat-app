<script lang="ts">
	import RoomItem from "./RoomItem.svelte";
    
    export let rooms: { name: string, id: string }[];
</script>

  <div class="rooms">
    {#each rooms as room, i}
        <RoomItem {room} index={i} />
    {/each}

  </div>

<style>
    .rooms {
        flex: 1 1 auto;

        width: 100%;
        padding: 1rem;

        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;

        gap: 1rem;
    }

</style>