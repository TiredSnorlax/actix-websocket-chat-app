<script lang="ts">
	import MsgWrapper from "./MsgWrapper.svelte";


    export let msgList: string[];
</script>

<div class="messageContainer">
    {#each msgList as json}
        <MsgWrapper {json} />
    {/each}
</div>

<style>
  .messageContainer {
    flex: 1 1 auto;

    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: stretch;
    gap: 1rem;
    width: 100%;
    padding: 1rem;

    overflow-y: auto;
  }

</style>