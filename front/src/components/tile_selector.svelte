<script>
  import { tiles, tiles_params, selected_tile } from "../stores/tiles_store";

  const handle_click = (id) => {
    selected_tile.set(id);
  };
</script>

<div class="tile-selector">
  {#each $tiles_params as param}
    <!--svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions-->
    <div
      class="tile-option"
      data-index={param.tile_id}
      on:click={(e) => handle_click(param.tile_id)}
    >
      <img src={$tiles[param.tile_id]} alt="Tile {param.tile_id}" />
    </div>
  {/each}
</div>

<style>
  .tile-selector {
    background: var(--background-dim);
    border-radius: var(--border-radius-m);
    overflow-y: auto;
    display: flex;
    flex-wrap: wrap;
    align-content: start;
    width: 100%;
    height: 100%;
  }

  .tile-option {
    height: var(--text-giant);
    width: var(--text-giant);
    margin: var(--margin-m);
    border-radius: var(--border-radius-m);
    background: var(--primary);
    display: flex;
    justify-content: center;
    align-items: center;
    transition: scale 0.1s ease-in;
    cursor: pointer;
  }

  .tile-option:hover {
    scale: 1.1;
  }

  img {
    width: 80%;
    height: 80%;
    border-radius: var(--border-radius-m);
  }

  ::-webkit-scrollbar {
    width: var(--text-small);
  }

  ::-webkit-scrollbar-track {
    border-radius: var(--border-radius-l);
    background: var(--primary-shade);
    border: none;
  }

  ::-webkit-scrollbar-button {
    display: none;
  }

  ::-webkit-scrollbar-corner {
    display: none;
  }

  ::-webkit-scrollbar-thumb {
    border-radius: var(--border-radius-l);
    background: var(--secondary);
  }
</style>
