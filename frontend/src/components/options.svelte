<script>
  import { onMount } from "svelte";
  import { image_width, image_height } from "../stores/image_store";
  import { tiles_width, tiles_height } from "../stores/config_store";
  import { get_divisors } from "../lib/utils.js";

  let possible_tiles_width = [];
  let possible_tiles_height = [];
  let select_width;
  let select_height;

  
  onMount(() => {
    let unsuscribed_img_w = image_width.subscribe((val) => {
      possible_tiles_width = get_divisors(val).reverse();
      tiles_width.set(possible_tiles_width[0]);
    });
    let unsuscribed_img_h = image_height.subscribe((val) => {
      possible_tiles_height = get_divisors(val).reverse();
      tiles_height.set(possible_tiles_height[0]);
    });

    return () => {
      unsuscribed_img_w();
      unsuscribed_img_h();
    };
  });

  const updateWidth = (e) => {
    console.log("Updating Tiles Width");
    tiles_width.set(e.target.value);
  };

  const updateHeight = (e) => {
    console.log("Updating Tiles Height");
    tiles_height.set(e.target.value);
  };

</script>

<div class="options-wrapper">
  <div class="option-select">
    <label for="tiles-width">Tiles Width:</label>
    <select bind:this={select_width} name="tiles-width" on:change={(e) => updateWidth(e)}  id="tiles-width">
      {#each possible_tiles_width as p }
      <option value={p}>{p}</option>
      {/each}
    </select>
  </div>
  <div class="option-select">
    <label for="tiles-width">Tiles Height:</label>
    <select bind:this={select_height} name="tiles-height" on:change={(e) => updateHeight(e)} id="tiles-height">
      {#each possible_tiles_height as p }
      <option value={p}>{p}</option>
      {/each}
    </select>
  </div>
</div>
