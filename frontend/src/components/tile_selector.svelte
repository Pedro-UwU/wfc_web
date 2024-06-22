<script>
  import { tiles, selected } from "../stores/image_store.js"

  let wrapper;
  let selected_button;
  $: tiles_with_index = $tiles.map((t, i) => ({src: t, index: i}));
  const scroll = (e) => {
    let delta = e.deltaY || e.detail || e.wheelDelta;
    wrapper.scrollLeft += (delta < 0) ? 1000 : -1000;
  }
  
  const select_tile = (e, i) => {
    // Get selected button
    $selected = i;
    console.log($selected);
  }

</script>

<div bind:this={wrapper} class="wrapper" tabindex="0" role="listbox" on:mousewheel={(e) => scroll(e)}>
{#each tiles_with_index as t }
  <button role="cell" on:click={(e) => select_tile(e, t.index)} class={$selected == t.index ? "selected" : ""}>
    <img src={t.src} alt="tile">
  </button>
{/each}
</div>


<style>
  .wrapper {
    scroll-behavior: smooth;
    background: var(--purple-dim);
    margin-bottom: 1rem;
    width: 100%;
    max-width: 100%;
    max-height: 10vh;
    min-height: 10vh;
    display: flex;
    align-items: center;
    overflow-x: auto;
    border-radius: 0.5em;
  }

  .wrapper::-webkit-scrollbar {
    width: 0;
    display: none;
  }

  .wrapper button {
    background: var(--bg2);
    border-radius: 0.5em;
    border: none;
    padding: 0;
    margin: 0;
    cursor: pointer;
    margin: 0.2%;
    min-width: 4vw;
    max-width: 4vw;
    aspect-ratio: 1/1;
    display: flex;
    justify-content: center;
    align-items: center;
    box-shadow: 1px 5px 3px -3px rgba(0, 0, 0, 0.4);
    transition: background 0.3s ease-in-out;
    transition: scale 0.3s ease-in-out;
  }
  
  .selected {
    scale: 1.1;
    background: var(--bg3) !important;
  }

  .wrapper button img {
    image-rendering: pixelated;
    width: 80%;
    height: 80%;
  }
</style>
