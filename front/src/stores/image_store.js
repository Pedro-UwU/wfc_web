import { writable } from "svelte/store";

let image = writable();
let image_name = writable("");
let tile_width = writable(0);
let tile_height = writable(0);
let tiles = writable([]);

export { image, image_name, tile_width, tile_height, tiles };
