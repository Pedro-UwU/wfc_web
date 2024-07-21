<script>
  import { onMount } from "svelte";
  import { deleted_categories } from "../stores/categories_store.js";
  import { tiles_params } from "../stores/tiles_store.js";

  onMount(() => {
    let unsuscribe_categories = deleted_categories.subscribe((new_cats) => {
      for (let tile of $tiles_params) {
        for (let deleted of new_cats) {
          for (let i = 0; i < 4; i++) {
            tile.remove_category("north", i, deleted);
            tile.remove_category("east", i, deleted);
            tile.remove_category("south", i, deleted);
            tile.remove_category("west", i, deleted);
          }
        }
      }
      console.log($tiles_params);
    });

    return () => {
      unsuscribe_categories();
    };
  });
</script>
