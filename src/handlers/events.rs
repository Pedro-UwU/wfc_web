use crate::{
    models::wfc::{BuildInfo, Neighbors},
    services::wfc::start_new_generation,
};
use serde::{Deserialize, Serialize};
use socketioxide::extract::SocketRef;
use std::collections::HashMap;
use tokio::task::spawn_blocking;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct BuildMessage {
    pub info: BuildInfo,
    pub graph: HashMap<usize, Neighbors>,
}

#[derive(Serialize, Debug)]
pub struct StepMessage {
    pub indices: Vec<usize>,
    pub values: Vec<usize>,
}

pub fn handle_build(socket: SocketRef, data: BuildMessage) {
    info!("Recieved data {:?}", data);
    let _ = socket.emit("building", "OK");
    let socket_clone = socket.clone();
    let _ = spawn_blocking(move || {
        start_new_generation(
            data.info,
            data.graph,
            Box::new(move |indices: Vec<usize>, values: Vec<usize>| send_step(socket_clone.clone(), indices, values)) // Step Callback
        );
    });
}

fn send_step(socket: SocketRef, indices: Vec<usize>, values: Vec<usize>) {
    let data = serde_json::to_string(&StepMessage { indices, values}).unwrap();
    info!("Sending step");
    let _ = socket.emit("step", data);
}
