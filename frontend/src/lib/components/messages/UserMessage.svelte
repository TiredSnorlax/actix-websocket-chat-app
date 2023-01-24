<script lang="ts">
    import type { IUserMessage} from "$lib/types";
    const current_session = localStorage.getItem("session");
    export let msg: IUserMessage;
</script>

<div class="msg" class:mine={current_session === msg.user_id}>
    <p>User: {msg.user_id}</p>
    <p>{msg.msg}</p>
</div>

<style>
    .msg {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: flex-start;
        max-width: 100%;

        font-size: 1.2rem;

        padding: 0.5rem 1rem;
        background: rgb(150, 150, 255);
        color: white;
        border-radius: 0.5rem;
    }

    .msg.mine {
        background:rgb(100, 255, 100);
    }

    .msg p:first-child {
        font-size: 0.9rem;
        font-weight: bold;
        color: rbg(200, 200, 200);
    }

    .msg p:last-child {
        flex: 1 1 auto;
    }
</style>