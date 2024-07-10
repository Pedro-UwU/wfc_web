use std::collections::HashMap;

use socketioxide::extract::SocketRef;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct BuildInfo {
    pub width: u32,
    pub height: u32
}

#[derive(Deserialize, Debug)]
pub struct Neighbors {
    pub n: Vec<u32>,
    pub e: Vec<u32>,
    pub s: Vec<u32>,
    pub w: Vec<u32>
}

#[derive(Deserialize, Debug)]
pub struct BuildMessage {
    pub info: BuildInfo,
    pub graph: HashMap<u32, Neighbors>
}

pub fn handle_build(socket: SocketRef, data: BuildMessage) {
    info!("Recieved data {:?}", data);
    let _ = socket.emit("building", "OK");

}
