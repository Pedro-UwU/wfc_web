<script>
  import Button from "../ui/button.svelte";
  import UploadModal from "./upload_modal.svelte";
  import {
    image,
    image_width,
    image_height,
    image_name,
  } from "../stores/image_store.js";
  import { tile_width, tile_height } from "../stores/tiles_store.js";
    import CategoriesModal from "./categories_modal.svelte";

  let tileset_width;
  let tileset_height;
  let tiles_width_px = "-";
  let tiles_height_px = "-";
  $: if ($image) {
    tileset_width = $image_width.toString();
    tileset_height = $image_height.toString();

    tiles_width_px = $tile_width.toString();
    tiles_height_px = $tile_height.toString();
  } else {
    tileset_width = "-";
    tileset_height = "-";
    tiles_width_px = "-";
    tiles_height_px = "-";
  }

  let show_modal_tileset = false;
  let show_modal_categories = false;

  const show_modal_tileset_on = () => {
    show_modal_tileset = true;
  };

  const show_modal_categories_on = () => {
    show_modal_categories  = true;
  }
</script>

<UploadModal bind:show_modal={show_modal_tileset} />
<CategoriesModal bind:show_modal={show_modal_categories} />
<div class="header">
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions (because of reasons) -->
  <div class="img-uploader" on:click={() => show_modal_tileset_on()}>
    {#if !$image}
      <!-- plus symbol -->
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
        <path
          d="M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 144L48 224c-17.7 0-32 14.3-32 32s14.3 32 32 32l144 0 0 144c0 17.7 14.3 32 32 32s32-14.3 32-32l0-144 144 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-144 0 0-144z"
        />
      </svg>
    {:else}
      <img src={$image.src} alt="tileset" />
    {/if}
  </div>
  <div class="tileset-info">
    {#if $image_name !== ""}
      {#if $image_name.length > 20}
        <h2>{$image_name.substring(20)}...</h2>
      {:else}
        <h2>{$image_name}</h2>
      {/if}
    {:else}
      <h2 class="faded">Upload an image</h2>
    {/if}
    <div class="info-list {$image_name ? '' : 'faded'}">
      <li>Tileset Width: {tileset_width}</li>
      <li>Tileset Height: {tileset_height}</li>
      <li>Tile Width: {tiles_width_px}</li>
      <li>Tile Height: {tiles_height_px}</li>
    </div>
  </div>
  <div class="btn-col">
    <div class="btn-container">
      <Button
        text="CATEGORIES"
        type="outlined"
        size="big"
        onClick={show_modal_categories_on}
      />
      <Button
        text="GENERATE"
        type="accent"
        onClick={() => console.log("Hola")}
        width="wide"
        size="big"
      />
    </div>
  </div>
</div>

<style>
  .header {
    background: var(--background-dim);
    height: calc(4 * var(--text-giant));
    display: flex;
    align-items: center;
    border-radius: var(--border-radius-m);
    flex: 0 0 auto;
  }

  .img-uploader {
    background: var(--secondary-shade);
    height: calc(4 * var(--text-giant) - 2 * (var(--margin-l)));
    width: calc(4 * var(--text-giant) - 2 * (var(--margin-l)));
    display: flex;
    justify-content: center;
    align-items: center;
    border-radius: var(--border-radius-m);
    margin: var(--margin-l);
    transition: filter 0.3s ease-in-out;
    overflow: hidden;
  }

  .img-uploader:hover {
    filter: brightness(110%);
    cursor: pointer;
  }

  .img-uploader:hover svg {
    scale: 1.1;
  }

  .tileset-info {
    flex: 1;
    height: 100%;
    display: flex;
    flex-flow: column;
  }

  .info-list {
    display: flex;
    flex-flow: column;
    height: 100%;
    margin-bottom: var(--margin-l);
    justify-content: space-around;
    color: var(--text-dim);
  }

  .btn-col {
    height: 100%;
    display: flex;
    flex-flow: column;
    justify-content: end;
  }

  .btn-container {
    margin-inline: var(--margin-l);
    margin-bottom: var(--margin-l);
    justify-content: end;
  }

  .faded {
    opacity: 0.5;
  }

  svg {
    width: 50%;
    height: 50%;
    fill: var(--primary);
    transition: scale 0.3s ease-in-out;
  }

  li {
    list-style-type: none;
  }

  img {
    height: 100%;
  }
</style>
