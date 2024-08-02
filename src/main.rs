use std::sync::{Arc};

use socketioxide::SocketIo;
use tokio::{net::TcpListener, sync::Mutex};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, cors::CorsLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use wfc_web::handlers::{connection::on_connect, task_queue::TaskQueue};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let (layer, io) = SocketIo::new_layer();
    let task_queue = Arc::new(Mutex::new(TaskQueue::new(4)));
    io.ns("/", {
        let task_queue = task_queue.clone();
        move |socket| on_connect(socket, task_queue.clone())
    });


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
