import { writable } from "svelte/store";

let tile_width = writable(0);
let tile_height = writable(0);
let tile_sections = writable(0);
let tiles = writable([]);

//** @type {import('svelte/store').Writable<Tile[]>} */
let tiles_params = writable([]);

let selected_tile = writable(-1);

export {
  tile_width,
  tile_height,
  tile_sections,
  tiles, 
  tiles_params,
  selected_tile
};

