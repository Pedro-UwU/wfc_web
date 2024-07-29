import { writable } from "svelte/store";

const states = Object.freeze({
  NOTHING: 0,
  BUILD: 1,
  BUILDING: 2,
  FINISHED: 98,
  ERROR: 99
})

const states_operations = Object.freeze({
  [states.NOTHING]: { 
    "build": () => console.log("TODO: Implement build")
  },
  [states.BUILD]: {
    "building": () => console.log("TODO: Implement building"),
    "error": () => console.log("TODO: Implement error")
  },
  [states.BUILDING]: {
    "gen": () => console.log("TODO: Implement gen"),
    "delete": () => console.log("TODO: Implement delete"),
    "error": () => console.log("TODO: Implement error"),
    "end": () => console.log("TODO: Implement end")
  }
})

const current_state = writable(states.NOTHING);




