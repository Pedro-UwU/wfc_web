<script>
  import { tile_sections } from "../stores/tiles_store.js";
  import { categories } from "../stores/categories_store.js";
  export let title;
  export let tile;
  let opened = false;
  let category_select;

  
  const update_tile_cat = (value, section) => {
    if (!tile) {
      return;
    }
    if (value === "-") {
      tile.remove_category(title.toLowerCase(), section);
      return;
    }
    tile.add_category(title.toLowerCase(), section, value);
    console.log(tile);
  }

  
  const open_content = () => {
    opened = !opened
  }
</script>

<div class="title">
  <!-- svelte-ignore a11y-no-static-element-interactions a11y-click-events-have-key-events -->
  <div on:click={open_content} class="expand-btn">
    <svg
      class={opened ? "rotated" : ""}
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 320 512"
      ><!--!Font Awesome Free 6.6.0 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path
        d="M137.4 374.6c12.5 12.5 32.8 12.5 45.3 0l128-128c9.2-9.2 11.9-22.9 6.9-34.9s-16.6-19.8-29.6-19.8L32 192c-12.9 0-24.6 7.8-29.6 19.8s-2.2 25.7 6.9 34.9l128 128z"
      /></svg
    >
  </div>
  <p>{title}</p>
</div>
<div class="content {opened ? '' : 'no-h'}">
  {#each { length: $tile_sections } as _, section}
    <div class="category-row">
      <p>Section {section + 1}:</p>
        <select bind:this={category_select} on:change={(e) => update_tile_cat(e.target.value, section)} class="cat-select">
          <option value="" selected>--</option>
          {#each $categories as cat}
          <option value={cat}>{cat}</option> 
          {/each}
        </select>
    </div>
  {/each}
</div>

<style>
  .title {
    display: flex;
  }
  .expand-btn {
    width: var(--text-medium);
    height: var(--text-medium);
    fill: var(--text-dim);
    cursor: pointer;
  }

  .expand-btn svg {
    transform: rotate(0deg);
    transition: transform 0.1s ease-in;
  }

  .rotated {
    transform: rotate(180deg) !important;
  }

  .no-h {
    height: 0px !important;
  }

  .content {
    overflow: hidden;
  }

  .category-row{
    display: flex;
    margin-block: var(--margin-m);
  }

  .category-row p {
    width: calc(5* var(--text-medium));
  }

  .category-row select {
    width: calc(2 * var(--text-big));
    max-width: calc(2 * var(--text-big));
    margin-inline: var(--margin-m);
  }
</style>
