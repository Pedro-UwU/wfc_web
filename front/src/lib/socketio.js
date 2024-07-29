import { io } from "socket.io-client"
import { tile_sections } from "../stores/tiles_store";
import { categories } from "../stores/categories_store";
import { readable, get } from "svelte/store"

let socket;

const connect_socket = () => {
  socket = io("http://127.0.0.1:3000"); // TODO: Repalce with env var
  socket.on("gen", (msg) => read_gen(msg))
  socket.on("delete", (msg) => read_delete(msg))
  socket.on("err", (msg) => read_error(msg))
  socket.on("building", read_building)
};

const build = (result_width, result_height, tiles) => {
  let cats = get(categories)
  let tiles_copy = tiles
    .map(tile => tile.clone())
    .filter((tile) => {
      return tile.active
    })
    .map((tile) => {
      for (let i = 0; i < get(tile_sections); i++) {
        console.log(tile.north[i]);
        tile.north[i] = cats.indexOf(tile.north[i]);
        tile.east[i] = cats.indexOf(tile.east[i]);
        tile.south[i] = cats.indexOf(tile.south[i]);
        tile.west[i] = cats.indexOf(tile.west[i]);
      }
      return tile;
    });
  for (let t of tiles_copy) {
    if (t.has_null_category()) {
      console.error("Active tiles cannot have null categories")
      return "Active tiles cannot have null categories"
    }
  }
  const dto = {
    result_width: result_width,
    result_height: result_height,
    categories: cats,
    tiles: tiles_copy
  };
  console.log(dto);
  // socket.emit("build", JSON.stringify(dto))
  return "";
}

const read_gen = (msg) => {
  try {
    let data = JSON.parse(msg);
    console.log(data); // TODO

  } catch (error) {
    console.error("Recieved invalid msg: ", msg);
  }
}

const read_error = (msg) => {
  console.log(msg);
}

const read_building = () => {
  console.log("Building")
}

const read_delete = (msg) => {
  try {
    let data = JSON.parse(msg);
    console.log(data); // TODO

  } catch (error) {
    console.error("Recieved invalid msg: ", msg);
  }
}

export { build }
