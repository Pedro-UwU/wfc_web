import { writable } from 'svelte/store';

let grid_width = writable(5);
let grid_height = writable(5);
let grid = writable([]);

let tiles_width = writable(16);
let tiles_height = writable(16);

let result_width = writable(5);
let result_height = writable(5);


export { grid_width, grid_height, tiles_height, tiles_width, grid, result_width, result_height }
