<script>
  import { onMount } from "svelte";
  import {
    result_width,
    result_height,
    result_values,
    tiles_width,
    tiles_height,
  } from "../stores/config_store";
  import { tiles } from "../stores/image_store.js";

  let canvas;
  let img_src;
  let img;

  export let show_modal = false;
  let dialog;

  $: if (dialog && show_modal) dialog.showModal();

  onMount(() => {
    
    let unsuscribe_result_value = result_values.subscribe((val) => {
      console.log("Result values changed");
      create_canvas();
      draw_canvas();
    });

    
    return () => {
      unsuscribe_result_value();
    };
  });

  const create_canvas = () => {
    canvas = document.createElement("canvas");
    canvas.width = $result_width * $tiles_width;
    canvas.height = $result_height * $tiles_height
    let context = canvas.getContext("2d");
    
    // context.width = result_width * tiles_width;
    // context.height = result_height * tiles_height;
    console.log("canvas created", canvas);
    console.log(canvas.width, canvas.height);

  }

  const draw_canvas = () => {
    console.log("Drawing");
    if (!canvas) {
      console.log("No canvas")
      return
    }
    const context = canvas.getContext("2d");
    const cell_width = canvas.width / $result_width;
    const cell_height = canvas.height / $result_height;
    for (let y = 0; y < $result_height; y++) {
      for (let x = 0; x < $result_width; x++) {
        const index = y * $result_width + x;
        if ($result_values[index] != 0 && !$result_values[index]) {
          //console.log("No values in: ", index)
          console.log($result_values[index]);
          continue;
        }
        let tile_index = $result_values[index];
        let x_coord = x * cell_width;
        let y_coord = y * cell_height;
        console.log("Drawing image tiles[", tile_index, "]:", $tiles[tile_index]);
        let image = new Image();
        image.src = $tiles[tile_index];
        context.drawImage(
          image,
          x_coord,
          y_coord,
          cell_width,
          cell_height,
        );
      }
    }
    img_src = canvas.toDataURL();
    img.src = img_src;
    console.log(canvas.width, canvas.height);
  };
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog
  bind:this={dialog}
  on:close={() => (show_modal= false)}
  on:click|self={() => dialog.close()}
>
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div on:click|stopPropagation>
    <img bind:this={img}/>
    <!-- svelte-ignore a11y-autofocus -->
    <button autofocus on:click={() => dialog.close()}>Close</button>
  </div>
</dialog>
