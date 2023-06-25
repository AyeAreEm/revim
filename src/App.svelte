<script>
  import Editing from './lib/Editing.svelte'
  // import num from './stores.js';
  // import { onMount } from 'svelte';

  // let container;

  // const onloadHandler = () => {
  //   num.subscribe(n => {
  //     // console.log(n);
  //     const div = document.createElement("div");
  //     div.textContent = `${n}`
  //     container.appendChild(div)
  //   })
  // }

  // onMount(() => {
  //   onloadHandler()
  // })
</script>

<main class="container">
  <!-- <div class="sideNav" bind:this={container}>
    <div>{$num}</div>
  </div> -->
  <Editing />
</main>

<style>

</style>
