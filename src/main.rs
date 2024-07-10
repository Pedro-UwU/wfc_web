use socketioxide::{extract::{Data, SocketRef}, SocketIo};
use tokio::{net::TcpListener, task::spawn_blocking};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, cors::CorsLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

fn heavy_computation(socket: SocketRef) {
    for _i in 1..10000000 {
        for _j in 1..100 {
            
        }
    }
    info!("Heavy computations done");
    let _ = socket.emit("greet back", "Hello!");
}

async fn on_connect(socket: SocketRef) {
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .nest_service("/", ServeDir::new("static"))
        .layer(ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .layer(layer)
        );

    info!("Starting Server");

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
