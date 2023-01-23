<script lang="ts">
    export let newText: string;
    export let send: () => void;
</script>

<div class="container">
    <div class="bar">
        <textarea bind:value={newText}></textarea>
        <button on:click={send} disabled={newText.length === 0} >Send</button>
    </div>
</div>

<style>
    .container {
        display: flex;
        justify-content: center;
        align-items: center;

        background: rgb(200, 200, 200);

        padding: 0.5rem;
    }

    .bar {
        display: flex;
        justify-content: center;
        align-items: center;

        overflow: hidden;

        border-radius: 10rem;
        box-shadow: 0 0 2px 0 grey;
        background: white;

        width: 100%;
    }

    .bar:has(textarea:focus) {
        box-shadow: 0 0 0 1px black;
    }

    textarea {
        border: none;
        outline: none;

        flex: 1 1 auto;
        min-width: 100px;
        resize: none;
        padding: 0.5rem 1rem;

        font-size: 1rem;

        cursor: pointer;
    }

    button {
        background: none;
        color: blue;
        border: none;
        outline: none;

        margin-right: 0.5rem;
        font-weight: bold;
    }

    button:disabled {
        color: grey;
    }
</style>
