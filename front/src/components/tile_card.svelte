<script>
  import { onMount } from "svelte";
  import {
    tiles,
    tiles_params,
    selected_tile,
    tile_sections,
  } from "../stores/tiles_store";
  import { categories } from "../stores/categories_store.js";

  let rotate;
  let active;
  let weight;

  let north_selects;
  let east_selects;
  let south_selects;
  let west_selects;

  $: if ($selected_tile !== -1) {
    const tile = $tiles_params[$selected_tile];
    if (rotate && active && weight) {
      rotate.checked = tile.can_rotate;
      active.checked = tile.active;
      weight.value = tile.weight;
    }
    if (north_selects && east_selects && south_selects && west_selects) {
      for (let i = 0; i < $tile_sections; i++) {
        let north_cat = tile.get_category("north", i);
        let east_cat = tile.get_category("east", i);
        let south_cat = tile.get_category("south", i);
        let west_cat = tile.get_category("west", i);
        north_selects.children[i].value = north_cat != null ? north_cat : "--";
        east_selects.children[i].value = east_cat != null ? east_cat : "--";
        south_selects.children[i].value = south_cat != null ? south_cat : "--";
        west_selects.children[i].value = west_cat != null ? west_cat : "--";
      }
    }
  }

  onMount(() => {
    let unsuscribe_selected = selected_tile.subscribe((new_selected) => {
      if (new_selected == -1) {
        return;
      }
      let tile = $tiles_params[new_selected];

      if (rotate && active && weight) {
        rotate.checked = tile.can_rotate;
        active.checked = tile.active;
        weight.value = tile.weight;
      }

      if (north_selects && east_selects && south_selects && west_selects) {
        for (let i = 0; i < $tile_sections; i++) {
          let north_cat = tile.get_category("north", i);
          let east_cat = tile.get_category("east", i);
          let south_cat = tile.get_category("south", i);
          let west_cat = tile.get_category("west", i);
          north_selects.children[i].value = north_cat ? north_cat : "--";
          east_selects.children[i].value = east_cat ? east_cat : "--";
          south_selects.children[i].value = south_cat ? south_cat : "--";
          west_selects.children[i].value = west_cat ? west_cat : "--";
        }
      }
    });

    return () => {
      unsuscribe_selected();
    };
  });

  const update_tile_cat = (direction, value, section) => {
    if ($selected_tile == -1) {
      console.error("Unselected tile");
      return;
    }
    const tile = $tiles_params[$selected_tile];
    if (value === "--" || value === "") {
      tile.remove_category(direction, section);
      return;
    }

    tile.add_category(direction, section, value);
    console.log(tile); // TO_DELETE
  };

  const handle_rotate_check = (e) => {
    if (e.target.checked !== undefined) {
      $tiles_params[$selected_tile].can_rotate = e.target.checked;
    }
    tiles_params.set($tiles_params);
  };

  const handle_active_check = (e) => {
    if (e.target.checked !== undefined) {
      $tiles_params[$selected_tile].active = e.target.checked;
    }
    tiles_params.set($tiles_params);
  };

  const handle_weight_change = (e) => {
    if (e.target.value !== undefined) {
      $tiles_params[$selected_tile].weight = e.target.value;
    }
    tiles_params.set($tiles_params);
  };
</script>

{#if $selected_tile === -1}
  <div class="missing-card">
    <h2>Select a Tile</h2>
  </div>
{:else}
  <div class="tile-card">
    <div class="tile-title">
      <h2>Tile id: {$selected_tile}</h2>
    </div>
    <div class="separator" />
    <div class="tile-overview">
      <div class="options">
        <div>
          <label for={active}>Active</label>
          <input
            on:change={(e) => handle_active_check(e)}
            type="checkbox"
            bind:this={active}
          />
        </div>
        <div>
          <label for={rotate}>Can Rotate</label>
          <input
            on:change={(e) => handle_rotate_check(e)}
            type="checkbox"
            bind:this={rotate}
          />
        </div>
        <div>
          <label for={weight}>Probability Weight</label>
          <input
            on:change={(e) => handle_weight_change(e)}
            type="number"
            bind:this={weight}
          />
        </div>
      </div>
    </div>

    <div class="separator" />
    <div class="cat-map">
      <div bind:this={north_selects} class="cat-select-row">
        {#each { length: $tile_sections } as _, i}
          <select
            on:change={(e) => {
              update_tile_cat("north", e.target.value, i);
            }}
            class="cat-select"
          >
            <option value="--" selected>--</option>
            {#each $categories as cat}
              <option value={cat}>{cat}</option>
            {/each}
          </select>
        {/each}
      </div>
      <div class="rows">
        <div bind:this={west_selects} class="cat-select-column">
          {#each { length: $tile_sections } as _, i}
            <select
              on:change={(e) => {
                update_tile_cat("west", e.target.value, i);
              }}
              class="cat-select deg-90"
            >
              <option value="--" selected>--</option>
              {#each $categories as cat}
                <option value={cat}>{cat}</option>
              {/each}
            </select>
          {/each}
        </div>
        <div class="preview">
          <img
            src={$tiles[$tiles_params[$selected_tile].tile_id]}
            alt="Tile preview"
          />
        </div>
        <div bind:this={east_selects} class="cat-select-column">
          {#each { length: $tile_sections } as _, i}
            <select
              on:change={(e) => {
                update_tile_cat("east", e.target.value, i);
              }}
              class="cat-select deg90"
            >
              <option value="--" selected>--</option>
              {#each $categories as cat}
                <option value={cat}>{cat}</option>
              {/each}
            </select>
          {/each}
        </div>
      </div>
      <div bind:this={south_selects} class="cat-select-row">
        {#each { length: $tile_sections } as _, i}
          <select
            on:change={(e) => {
              update_tile_cat("south", e.target.value, i);
            }}
            class="cat-select"
          >
            <option value="--" selected>--</option>
            {#each $categories as cat}
              <option value={cat}>{cat}</option>
            {/each}
          </select>
        {/each}
      </div>
    </div>
  </div>
{/if}

<style>
  .tile-card {
    background: var(--background-dim);
    width: 100%;
    height: 100%;
    border-radius: var(--border-radius-m);
    overflow-y: auto;
  }

  .tile-title {
    overflow: hidden;
    margin-inline: var(--margin-l);
    height: var(--text-giant);
  }

  .separator {
    height: 2px;
    background: var(--text-dim-50);
    margin-inline: var(--margin-l);
  }

  .tile-overview {
    margin-block: var(--margin-l);
    display: flex;
    align-items: center;
  }

  .options {
    flex: 1;
    height: calc(1.5 * var(--text-giant));
    display: flex;
    margin-inline: var(--margin-l);
    flex-flow: column;
    justify-content: space-between;
  }

  .options div {
    display: flex;
  }

  .options div label {
    flex: 1;
  }

  .preview {
    width: calc(1.5 * var(--text-giant));
    height: calc(1.5 * var(--text-giant));
    height: 100%;
    border-radius: var(--border-radius-m);
    overflow: hidden;
    display: flex;
    justify-content: center;
    align-items: center;
    margin-inline: var(--margin-l);
  }

  .selector-wrapper {
    margin-inline: var(--margin-l);
  }

  .missing-card {
    background: var(--background-dim);
    width: 100%;
    height: 100%;
    border-radius: var(--border-radius-m);
    display: flex;
    justify-content: center;
    align-items: center;
    color: var(--text-dim-50);
  }
  .cat-map {
    display: flex;
    flex-flow: column;
    align-items: center;
  }

  .cat-select-row {
    display: flex;
    justify-content: space-evenly;
    margin-block: var(--margin-m);
    width: calc(6 * var(--text-big));
  }

  .cat-select-column {
    display: flex;
    flex-flow: column;
    height: 100%;
    justify-content: space-evenly;
  }

  .deg90 {
    transform: rotate(90deg);
  }

  .deg-90 {
    transform: rotate(-90deg);
  }

  .rows {
    display: flex;
    align-items: center;
    justify-content: center;
    height: calc(6 * var(--text-big));
  }

  .preview {
    box-sizing: border-box;
    width: 100%;
    padding: calc(100% - 2 * (margin-xl));
  }

  .preview img {
    width: 100%;
    height: 100%;
  }

  .cat-select-column .cat-select {
    margin-block: var(--margin-m);
  }

  .cat-select {
    width: calc(3 * var(--text-medium));
  }
</style>
