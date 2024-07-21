import { writable } from "svelte/store";

const categories = writable([]);
const max_categories = writable(0);
const deleted_categories = writable([]);

export { categories, max_categories, deleted_categories }
