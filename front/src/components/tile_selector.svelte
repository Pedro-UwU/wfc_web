<script>
  import { tiles, tiles_params, selected_tile } from "../stores/tiles_store";
    import Button from "../ui/button.svelte";

  const handle_click = (id) => {
    selected_tile.set(id);
  };

  const handle_import = () => {
    console.log("import");
  }

  const handle_export = () => {
    console.log("export");
  }
</script>

<div class="tile-selector">
  <div class="title">
    <h2>Tiles</h2>
    <div class="buttons">
      <Button onClick={handle_export} size="big" type="secondary" text="EXPORT"/>
      <Button onClick={handle_import} size="big" type="secondary" text="IMPORT"/>
    </div>
  </div>
  <div class="tile-items">
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
</div>

<style>
  .tile-selector {
    background: var(--background-dim);
    border-radius: var(--border-radius-m);
    height: 100%;
  }

  .title {
    margin-inline: var(--margin-l);
    margin-block: var(--margin-l);
    height: var(--text-big);
    display: flex;
  }

  .title h2 {
    margin: 0 !important;
    flex: 1;
  }
  .tile-items{
    overflow-y: auto;
    display: flex;
    flex-wrap: wrap;
    align-content: start;
    width: calc(100% - var(--margin-m) - var(--margin-l));
    height: calc(100% - 3*var(--margin-l) - var(--text-big));
    margin-inline: var(--margin-m);
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
