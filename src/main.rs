use socketioxide::{extract::SocketRef, SocketIo};
use tokio::net::TcpListener;
use tower::{Layer, ServiceBuilder};
use tower_http::{services::ServeDir, cors::CorsLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

async  fn on_connect(socket: SocketRef) {
    info!("Socket connected: {}", socket.id);
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
