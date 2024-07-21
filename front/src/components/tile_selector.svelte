<script>
  import { onMount } from "svelte";
  import { tiles, tiles_params, selected_tile } from "../stores/tiles_store";
  import Button from "../ui/button.svelte";

  let tile_items;

  const handle_click = (id) => {
    selected_tile.set(id);
  };

  onMount(() => {
    let unsuscribe_tile_params = tiles_params.subscribe((new_val) => {
      
      for (let i = 0; i < $tiles_params.length; i++) {
        if (!tile_items.children) {
          return;
        }
        if (!tile_items.children[i]) {
          return;
        }
        // Here I need to acces the i child of tile_items and do something
        let child = tile_items.children[i];

        if (new_val[i].active) {
          child.classList.add('active');
        } else {
          child.classList.remove('active');
        }
      }
    });

    return () => {
      unsuscribe_tile_params();
    };
  });

  const handle_import = () => {
    console.log("import");
  };

  const handle_export = () => {
    console.log("export");
  };
</script>

<div class="tile-selector">
  <div class="title">
    <h2>Tiles</h2>
    <div class="buttons">
      <Button
        onClick={handle_export}
        size="big"
        type="secondary"
        text="EXPORT"
      />
      <Button
        onClick={handle_import}
        size="big"
        type="secondary"
        text="IMPORT"
      />
    </div>
  </div>
  <div bind:this={tile_items} class="tile-items">
    {#each $tiles_params as param}
      <!--svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions-->
      <div
        class="tile-option {false ? 'active' : ''}"
        data-index={param.tile_id}
        on:click={() => handle_click(param.tile_id)}
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
  .tile-items {
    overflow-y: auto;
    display: flex;
    flex-wrap: wrap;
    align-content: start;
    width: calc(100% - var(--margin-m) - var(--margin-l));
    height: calc(100% - 3 * var(--margin-l) - var(--text-big));
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

  .active {
    background: var(--accent);
    scale: 1.05;
  }

  img {
    width: 80%;
    height: 80%;
    border-radius: var(--border-radius-m);
  }

</style>
