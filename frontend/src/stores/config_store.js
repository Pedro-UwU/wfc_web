import { writable } from 'svelte/store';

let grid_width = writable(15);
let grid_height = writable(15);


export { grid_width, grid_height }
