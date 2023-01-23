<script lang="ts">
	import InfoMessage from "./InfoMessage.svelte";
import UserMessage from "./UserMessage.svelte";

    export let json: string;

    const data = JSON.parse(json);
    console.log(data.info_type);
</script>

{#if data.info_type}
    <InfoMessage msg={data} />
{:else}
    <UserMessage msg={data} />
{/if}