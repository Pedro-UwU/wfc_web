// Import socket
import { io } from "socket.io-client"
import { readable } from "svelte/store";

const socket = io("http://127.0.0.1:3000"); // TODO make URL an env variable
const greet = () => {
  socket.emit("greet", "Hi");
}

const greet_bad = () => {
  socket.emit("greet", "UwU");
}

const setup_socket = () => {
  socket.on("greet back", () => { console.log("Greet back received") })
  socket.on("rude", (msg) => {console.log("Server said ", msg)})
}

setup_socket()
console.log("Initializing socket");
const socket_store = readable(socket);

export { socket_store, greet, greet_bad };



