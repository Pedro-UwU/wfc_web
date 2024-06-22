import { writable } from 'svelte/store';

let image = writable(null);
let image_width = writable(0);
let image_height = writable(0);
let tiles = writable([]);
let selected = writable(-1);

export { image, image_width, image_height, tiles, selected };
