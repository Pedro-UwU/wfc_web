use std::collections::HashMap;
use socketioxide::extract::SocketRef;
use serde::Deserialize;
use tracing::info;
use tokio::task::spawn_blocking;
use crate::{models::wfc::{BuildInfo, Neighbors}, services::wfc::start_new_generation};

#[derive(Deserialize, Debug)]
pub struct BuildMessage {
    pub info: BuildInfo,
    pub graph: HashMap<usize, Neighbors>
}

pub fn handle_build(socket: SocketRef, data: BuildMessage) {
    info!("Recieved data {:?}", data);
    let _ = socket.emit("building", "OK");
    let _ = spawn_blocking(move || {
        start_new_generation(data.info, data.graph);
    });
}
