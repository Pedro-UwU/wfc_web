use socketioxide::extract::{Data, SocketRef};
use tokio::task::spawn_blocking;
use tracing::{error, info};

use crate::{handlers::events::{handle_build, BuildMessage}, utils::heavy_computation};

pub async fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);
    // Define the events:
    // - "Build" Client -> Server
    // - "Building" Server -> Client
    // - "Step" Server -> Client
    // - "Finished" Server -> Client
    // - "Error" Server -> Client
   
    
    // Little greeting to test with frontend
    socket.on("greet", |socket: SocketRef, Data::<String>(data)| async move {
        if data == "Hi" {
            info!("Greeting socket {}", socket.id);
            let _ = spawn_blocking(|| {
                heavy_computation(socket)
            }).await.unwrap();
        } else {
            info!("Socket {} was rude!", socket.id);
            let _ = socket.emit("rude", "You have to say 'Hi'");
        }
    });

    socket.on("build", |socket: SocketRef, Data::<String>(data)| async move {
        info!("Received build from socket {}", socket.id);
        match serde_json::from_str::<BuildMessage>(&data) {
            Ok(build_msg) => handle_build(socket, build_msg),
            Err(err) => error!("Failed to deserealize build message from socket {}: {}", socket.id, err)
        };
    });
}

