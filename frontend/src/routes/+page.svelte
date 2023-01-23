<script lang="ts">
	import Rooms from "$lib/components/rooms/Rooms.svelte";
  import { serverDomain } from "$lib/helper/connect";
	import { onMount } from "svelte";

  interface IRoom{
    name: string,
    id: string
  }

  let rooms: IRoom[] = [];

  let newChat = "";

  const getRooms = async () => {
    await fetch(serverDomain + "list")
      .then(res => res.json())
      .then(data => {
        console.log(data);
        rooms = data;
      })
  }

  const newRoom = async () => {
    await fetch(serverDomain + "room/new", {
      method: "post",
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      //make sure to serialize your JSON body
      body: JSON.stringify({
        name: newChat,
      })
    }).then(res => {
      if (res.redirected) {
        window.location.href = res.url
      }
    });
  }

  onMount( () => {
    getRooms();
  })

</script>

<div class="page">
  <h1>Available rooms</h1>
  <Rooms {rooms} />
  <input type="text" bind:value={newChat} />
  <button on:click={newRoom}>New</button>
</div>

<style>
  .page {
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

</style>