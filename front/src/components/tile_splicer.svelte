<script>
  import {
    image,
    image_width,
    image_height,
  } from "../stores/image_store";
  import { tiles, tile_height, tile_width, tiles_params, tile_sections } from "../stores/tiles_store";
  import { get_tile_array } from "../lib/utils.js";
  import { onMount } from "svelte";
  import { Tile } from "../lib/tile.js";

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
        let params = temp.map((_,  index) => {
          const t = new Tile(index)
          t.set_sections($tile_sections)
          return t;
        })
        tiles_params.set(params);
      };
    });

    return () => {
      unsuscribe_image_store();
    };
  });
</script>
