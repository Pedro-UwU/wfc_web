import { writable } from 'svelte/store';

let grid_width = writable(5);
let grid_height = writable(5);
let grid = writable([]);

let tiles_width = writable(16);
let tiles_height = writable(16);

let result_width = writable(30);
let result_height = writable(30);
let result_values = writable([]);

let img_result_width = writable(800);
let img_result_height = writable(800);


export { grid_width, 
  grid_height, 
  tiles_height, 
  tiles_width, 
  grid, 
  result_width, 
  result_height, 
  result_values,
  img_result_width,
  img_result_height
}
