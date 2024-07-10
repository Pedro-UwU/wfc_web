use socketioxide::extract::SocketRef;
use tracing::info;

pub fn heavy_computation(socket: SocketRef) {
    for _i in 1..10000000 {
        for _j in 1..12 {
            
        }
    }
    info!("Heavy computations done");
    let _ = socket.emit("greet back", "Hello!");
}
