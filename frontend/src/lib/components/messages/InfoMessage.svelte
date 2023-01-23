<script lang="ts">
    import type { IInfoMessage } from "$lib/types/messages";

    export let msg: IInfoMessage;
    const MSG_COLORS: {[key: string]: string } = {
        Join: "green",
        Leave: "red",
        Alert: "rgb(200, 200, 200)"
    }
</script>

<div class="msg" style="background: {MSG_COLORS[msg.info_type]};">
    <p>{msg.body}</p>
</div>

<style>
    .msg {
        padding: 0.5rem 0.5rem;
        border-radius: 10rem;
        color: white;
        max-width: 100%;

        text-align: center;

        font-size: 0.9rem;
    }
</style>