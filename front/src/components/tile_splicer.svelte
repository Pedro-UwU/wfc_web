<script>
  import {
    image,
    image_width,
    image_height,
    tile_width,
    tile_height,
    tiles,
  } from "../stores/image_store";
  import { get_tile_array, get_image_width_and_height } from "../lib/utils.js";
  import { onMount } from "svelte";

  onMount(() => {
    let unsuscribe_image_store = image.subscribe((new_img) => {
      if (!new_img) {
        return;
      }
      new_img.onload = () => {
        let temp = get_tile_array(
          new_img,
          $image_width,
          $image_height,
          $tile_width,
          $tile_height,
        );
        tiles.set(temp)
      };
    });

    return () => {
      unsuscribe_image_store();
    };
  });
</script>
