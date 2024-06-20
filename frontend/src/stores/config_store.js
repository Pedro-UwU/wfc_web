import { writable } from 'svelte/store';

let grid_width = writable(15);
let grid_height = writable(15);

let tiles_width = writable(16);
let tiles_height = writable(16);


export { grid_width, grid_height, tiles_height, tiles_width }
