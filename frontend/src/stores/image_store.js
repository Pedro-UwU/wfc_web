import { writable } from 'svelte/store';

let image = writable(null);
let tiles = writable([]);

export { image, tiles };
