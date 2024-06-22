<script>
    import { onMount } from "svelte";
  import UploadDefault from "../assets/upload.png";
  import { tiles_width, tiles_height } from "../stores/config_store.js";
  import { image, image_width, image_height, tiles } from "../stores/image_store.js";

  let image_title = "No image selected";
  let image_url = "";
  let img;
  let selected = false;
  let image_input;

  onMount(() => {
    // Set default tile size
    let unsuscribe_tiles_width = tiles_width.subscribe((val) => {
      $tiles_width = val;
      console.log("Updating tiles width");
      tiles.set(getTileArray(img, $image_width, $image_height));
    });
    let unsuscribe_tiles_height = tiles_height.subscribe((val) => {
      $tiles_height = val;
      tiles.set(getTileArray(img, $image_width, $image_height));
    });

    return () => {
      unsuscribe_tiles_width();
      unsuscribe_tiles_height();
    };
  });

  // @ts-ignore
  const handleUpload = async (e) => {
    let file = e.target.files[0];
    image_url = URL.createObjectURL(file);
    image_title = file.name;
    selected = true;
    $tiles = [];

    // Get dimensions
    try {
      let dimensions = await getImageWidthAndHeight(image_url);
      image_width.set(dimensions.width);
      image_height.set(dimensions.height);
    } catch (err) {
      console.log(err);
      return;
    }

    tiles.set(getTileArray(img, $image_width, $image_height));
  };

  const getImageWidthAndHeight = (objectURL) => {
    return new Promise((resolve, reject) => {
      let img = new Image();
      img.onload = () => {
        resolve({ width: img.naturalWidth, height: img.naturalHeight });
      };
      img.onerror = (err) => {
        reject(err);
      };
      img.src = objectURL;
    });
  };

  const getTileArray = (img, img_w, img_h) => {
    console.log("Calculating tiles");
    // Initialize tiles
    let total_cols = img_w / $tiles_width;
    let total_rows = img_h / $tiles_height;
    let temp_tiles = [];

    for (let i = 0; i < total_cols; i++) {
      for (let j = 0; j < total_rows; j++) {
        let aux_canvas = document.createElement("canvas");
        let aux_context = aux_canvas.getContext("2d");
        aux_canvas.width = $tiles_width;
        aux_canvas.height = $tiles_height;
        aux_context.drawImage(
          img,
          i * $tiles_width,
          j * $tiles_height,
          $tiles_width,
          $tiles_height,
          0,
          0,
          $tiles_width,
          $tiles_height,
        );
        let img_data = aux_canvas.toDataURL();
        temp_tiles.push(img_data);
      }
    }
    return [...new Set(temp_tiles)];
  };

  const getImageBase64 = () => {
    let canvas = document.createElement("canvas");
    let context = canvas.getContext("2d");
    canvas.height = img.naturalHeight;
    canvas.width = img.naturalWidth;
    context.drawImage(img, 0, 0, canvas.height, canvas.width);
    let base64String = canvas.toDataURL();
    console.log(base64String);
  };
</script>

<div class="wrapper">
  <button class={selected ? "img-wrapper" : "img-wrapper container-placeholder"} on:click={image_input.click()}>
    {#if selected}
      <img
        bind:this={img}
        class="img-container"
        src={selected ? image_url : UploadDefault}
        alt="selected tile set"
        on:change={getImageBase64}
      />
    {/if}
    {#if !selected}
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"
        class="placeholder"
        ><!--!Font Awesome Free 6.5.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2024 Fonticons, Inc.--><path
          d="M246.6 9.4c-12.5-12.5-32.8-12.5-45.3 0l-128 128c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L192 109.3 192 320c0 17.7 14.3 32 32 32s32-14.3 32-32l0-210.7 73.4 73.4c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3l-128-128zM64 352c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 64c0 53 43 96 96 96l256 0c53 0 96-43 96-96l0-64c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 64c0 17.7-14.3 32-32 32L96 448c-17.7 0-32-14.3-32-32l0-64z"
        /></svg
      >
    {/if}
  </button>
  <div class={selected ? "img-title" : "unselected img-title"}>
    {image_title}
  </div>

  <input
    bind:this={image_input}
    type="file"
    id="input-file"
    style="display: none"
    on:change={(e) => handleUpload(e)}
    accept="image/png, image/jpeg"
  />
</div>

<style>
  .wrapper {
    display: flex;
    width: 100%;
    justify-content: start;
    align-items: center;
  }

  .img-title {
    margin-left: 1em;
    flex: 1;
    color: var(--fg);
  }

  .unselected {
    color: var(--green);
    opacity: 0.25;
  }

  .img-wrapper {
    all: unset;
    width: 4rem;
    min-width: 4rem;
    overflow: hidden;
    aspect-ratio: 1/1;
    display: flex;
    justify-items: center;
    align-items: center;
    border: 0.5rem solid var(--green-dim);
    border-radius: 0.5rem;
    padding: 0.4rem;
  }

  .container-placeholder {
    background: var(--green);
  }

  .img-wrapper:hover {
    cursor: pointer;
    filter: brightness(1.1);
  }

  img {
    height: 100%;
  }

  .img-container {
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .placeholder {
    fill: var(--green-dim);
    width: 100%;
  } 
</style>
