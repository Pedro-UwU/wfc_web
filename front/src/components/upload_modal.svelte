<script>
  import Button from "../ui/button.svelte";
  import { get_divisors, get_image_width_and_height } from "../lib/utils.js";
  import { image, image_name, image_width, image_height } from "../stores/image_store.js";
  import { tile_width, tile_height, tile_sections } from "../stores/tiles_store";
  import { categories, max_categories } from "../stores/categories_store";

  export let show_modal; // It is false
  let dialog;
  let image_input;
  let image_loaded = false;
  let img_w = 0;
  let img_h = 0;
  let img_title = "";
  let img;
  let img_url;

  let tile_w = 0;
  let tile_h = 0;
  let tile_secs = 0;

  let possible_img_w = [];
  let possible_img_h = [];
  let error_tw;
  let error_th;

  //@ts-ignore
  const get_possible_divisors = (width, height) => {
    possible_img_w = get_divisors(width);
    possible_img_h = get_divisors(height);
  };

  //@ts-ignore
  const handle_upload = async (e) => {
    let file = e.target.files[0];
    img_url = URL.createObjectURL(file);
    img_title = file.name;
    image_loaded = true;
    let dimensions = await get_image_width_and_height(img_url);
    img_w = dimensions.width;
    img_h = dimensions.height;
    get_possible_divisors(img_w, img_h);
  };

  const handle_save = async () => {
    let valid = true;
    if (tile_w === 0) {
      valid = false;
      error_tw.classList.remove("invisible");
    } else {
      error_tw.classList.add("invisible");
    }

    if (tile_h === 0) {
      valid = false;
      error_th.classList.remove("invisible");
    } else {
      error_th.classList.add("invisible");
    }

    if (!valid) {
     return;
    }
    let img_elem = new Image(img_w, img_h);
    img_elem.src = img_url;

    tile_width.set(tile_w);
    tile_height.set(tile_h);
    tile_sections.set(tile_secs);
    image_name.set(img_title);
    image_width.set(img_w);
    image_height.set(img_h);
    categories.set([]);
    image.set(img_elem); // Is importatnt to call it last
    dialog.close();
  };

  $: if (dialog && show_modal) {
    dialog.showModal();
  } else if (dialog && !show_modal && dialog.hasAttribute("open")) {
    dialog.close();
  }
</script>

{#if show_modal}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
  <dialog
    bind:this={dialog}
    on:close={() => (show_modal = false)}
    on:click|self={() => dialog.close()}
  >
    <h2>Upload Image</h2>
    <!-- svelte-ignore a11y-no-static-element-interactions a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
    <div class="img-container" on:click={image_input.click()}>
      {#if !image_loaded}
        <div class="img_placeholder" />
      {/if}
      {#if image_loaded}
        <img bind:this={img} src={img_url} alt="uploaded" />
      {/if}
    </div>

    <div class="options {image_loaded ? '' : 'disabled'}">
      <small bind:this={error_tw} class="invisible"
        >Enter a valid Tile Width</small
      >
      <div class="option">
        <label for="tile-width"> Tiles Width: </label>
        <select bind:value={tile_w} disabled={!image_loaded}>
          {#each possible_img_w as x}
            <option value={x}>{x}px</option>
          {/each}
        </select>
      </div>
      <small bind:this={error_th} class="invisible"
        >Enter a valid Tile Height</small
      >
      <div class="option {image_loaded ? '' : 'disabled'}">
        <label for="tile-height"> Tiles Height: </label>
        <select bind:value={tile_h} disabled={!image_loaded}>
          {#each possible_img_h as x}
            <option value={x}>{x}px</option>
          {/each}
        </select>
      </div>
      <div class="option {image_loaded ? '' : 'disabled'}">
        <label for="tile-sections"> Sections per edge: </label>
        <select bind:value={tile_secs} disabled={!image_loaded}>
            <option value={1}>1</option>
            <option value={2}>2</option>
            <option value={3}>3</option>
            <option value={4}>4</option>
        </select>
      </div>

    </div>

    <div class="buttons">
      <Button
        size="big"
        type="secondary"
        onClick={() => dialog.close()}
        text="CANCEL"
      />
      <Button
        size="big"
        type="accent"
        onClick={() => handle_save()}
        text="UPLOAD"
        disabled={!image_loaded}
      />
    </div>
    <input
      bind:this={image_input}
      type="file"
      id="input-file"
      style="display: none"
      on:change={(e) => handle_upload(e)}
      accept="image/png, image/jpeg"
    />
  </dialog>
{/if}

<style>
  dialog {
    max-width: var(--modal-width);
    border-radius: var(--border-radius-l);
    background: var(--background-dim);
    border: 0;
    color: var(--text);
    display: flex;
    flex-flow: column;
    align-items: center;
  }

  dialog::backdrop {
    background: rgba(0, 0, 0, 0.75);
  }

  dialog[open] {
    animation: zoom 0.3s ease-in-out;
  }

  @keyframes zoom {
    from {
      transform: scale(0.95);
    }
    to {
      transform: scale(1);
    }
  }

  dialog[open]::backdrop {
    animation: fade 0.3s ease-in-out;
  }

  @keyframes fade {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .img-container {
    width: calc(0.5 * var(--modal-width));
    height: calc(0.5 * var(--modal-width));
    border-radius: var(--border-radius-l);
    overflow: hidden;
    cursor: pointer;
    transition: all 0.3s ease-in-out;
    display: flex;
    justify-content: center;
  }

  .img-container:hover {
    filter: brightness(1.1);
    scale: 0.995;
  }

  .img_placeholder {
    background: var(--secondary-shade);
    width: 100%;
    height: 100%;
  }

  .options {
    margin-top: var(--margin-l);
    width: 100%;
  }

  .options select {
    width: var(--text-giant);
  }

  .disabled select {
    background: var(--secondary-shade);
    color: var(--text-dim-50);
  }

  .disabled label {
    color: var(--text-dim-50);
  }

  .option {
    display: flex;
    margin-block: var(--margin-m);
  }

  .option label {
    flex: 1;
  }

  .buttons {
    margin-top: var(--margin-l);
  }

  small {
    color: var(--error);
  }

  .invisible {
    display: none;
  }
</style>
