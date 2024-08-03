use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct TileMessage {
    pub active: bool,
    pub can_rotate: bool,
    pub sections: u32,
    pub tile_id: u32,
    pub weight: f64,
    pub north: Vec<u16>,
    pub east: Vec<u16>,
    pub south: Vec<u16>,
    pub west: Vec<u16>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BuildMessage {
    pub result_width: u32,
    pub result_height: u32,
    pub categories: Vec<String>,
    pub tiles: Vec<TileMessage>,
}

//
// #[derive(Serialize, Debug)]
// pub struct StepMessage {
//     pub indices: Vec<usize>,
//     pub values: Vec<usize>,
// }
//
// pub fn handle_build(socket: SocketRef, data: BuildMessage) {
//     info!("Recieved data {:?}", data);
//     let _ = socket.emit("building", "OK");
//     let socket = Arc::new(socket);
//     let socket_step = Arc::clone(&socket);
//     let socket_err = Arc::clone(&socket);
//     let _ = spawn_blocking(move || {
//         start_new_generation(
//             data.info,
//             data.graph,
//             Box::new(move |indices: Vec<usize>, values: Vec<usize>| send_step(socket_step.as_ref(), indices, values)), // Step Callback
//             Box::new(move |msg: String| send_error(socket_err.as_ref(), msg))
//         );
//     });
// }
//
// fn send_step(socket: &SocketRef, indices: Vec<usize>, values: Vec<usize>) {
//     let data = serde_json::to_string(&StepMessage { indices, values}).unwrap();
//     info!("Sending step");
//     let _ = socket.emit("step", data);
// }
//
// fn send_error(socket: &SocketRef, msg: String) {
//     let _ = socket.emit("wfc error", msg);
// }
