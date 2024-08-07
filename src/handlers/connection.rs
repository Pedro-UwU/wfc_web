use std::{sync::Arc, time::Duration};

use super::task_queue::TaskQueue;
use crate::{handlers::events::BuildMessage, services::wfc2::build};
use socketioxide::extract::{Data, SocketRef};
use tokio::sync::Mutex;
use tracing::{error, info};

pub async fn on_connect(socket: SocketRef, task_queue: Arc<Mutex<TaskQueue>>) {
    info!("Socket connected: {}", socket.id);
    {
        let mut task_queue = task_queue.lock().await;
        task_queue.start_processing().await;
    };
    let task_queue_clone = task_queue.clone();
    socket.on(
        "greet",
        |socket: SocketRef, Data::<String>(data)| async move {
            let task_queue_mutex = task_queue_clone.clone();
            let mut task_queue = task_queue_mutex.lock().await;
            task_queue
                .add_task(Box::new(move || {
                    let socket = socket.clone();
                    let _ = socket.emit("Hi", &data);
                    std::thread::sleep(Duration::from_secs(5));
                    let _ = socket.emit("Bye", &data);
                }))
                .await;
        },
    );

    let task_queue_clone = task_queue.clone();
    socket.on(
        "build",
        |socket: SocketRef, Data::<String>(data)| async move {
            info!("New build from socket {}", socket.id);
            match serde_json::from_str::<BuildMessage>(&data) {
                Ok(build_msg) => {
                    let task_queue_mutex = task_queue_clone.clone();
                    let mut task_queue = task_queue_mutex.lock().await;
                    task_queue.add_task(Box::new(move || {
                        build(socket, build_msg);
                    })).await;
                }
                Err(err) => {
                    error!(
                        "Failed to deserealize build message from socket {}: {}",
                        socket.id, err
                    );
                    socket.emit("error", "Invalid build message");
                }
            }
        },
    );
}
