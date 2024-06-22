import { writable } from 'svelte/store';

let image = writable(null);
let tiles = writable([]);
let selected = writable(-1);

export { image, tiles, selected };
