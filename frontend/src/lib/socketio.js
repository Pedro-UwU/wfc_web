// Import socket
import { io } from "socket.io-client"
import { readable, get } from "svelte/store";
import { result_values } from "../stores/config_store";

const socket = io("http://127.0.0.1:3000"); // TODO make URL an env variable
const greet = () => {
  socket.emit("greet", "Hi");
}

const greet_bad = () => {
  socket.emit("greet", "UwU");
}

const build = (message_info) => {
  console.log("Sending build");
  console.log(message_info);
  socket.emit("build", JSON.stringify(message_info));
}

const setup_socket = () => {
  socket.on("greet back", () => { console.log("Greet back received") })
  socket.on("rude", (msg) => {console.log("Server said ", msg)})
  
  socket.on("step", (msg) => {
    // Convert to js object
    let o = JSON.parse(msg);
    let indices = o["indices"];
    let values = o["values"];
    let current_values = get(result_values);
    indices.forEach((i) => {
      current_values[i] = values[i];
    });
    result_values.set(current_values);
  })
}


setup_socket()
const socket_store = readable(socket);

export { socket_store, greet, build };



