import { writable } from "svelte/store";

let image = writable();
let image_name = writable("");
let image_width = writable(0);
let image_height = writable(0);
export { image, image_name, image_width, image_height };
