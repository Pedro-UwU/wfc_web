use std::{
    collections::VecDeque,
    sync::{Arc, Mutex, RwLock},
};
use tokio::{sync::Semaphore, task::JoinHandle};

type BoxedTask = Box<dyn FnOnce() + Send + 'static>;

pub struct TaskQueue {
    queue: Arc<Mutex<VecDeque<BoxedTask>>>,
    semaphore: Arc<Semaphore>,
    running: Arc<RwLock<bool>>,
    handlers: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl TaskQueue {
    pub fn new(max_threads: usize) -> Self {
        TaskQueue {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            semaphore: Arc::new(Semaphore::new(max_threads)),
            running: Arc::new(RwLock::new(false)),
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add_task(&mut self, task: BoxedTask) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(task);
    }

    pub fn start_processing(&mut self) {
        {
            let mut running_value = self.running.write().unwrap();
            *running_value = true;
        }
        let running = self.running.clone();
        let queue = self.queue.clone();
        let semaphore = self.semaphore.clone();
        let handlers = self.handlers.clone();
        let _main_executor = tokio::spawn(async move {
            while *running.read().unwrap() {
                let permit = semaphore.acquire().await.unwrap();

                let maybe_task = {
                    let mut queue = queue.lock().unwrap();
                    queue.pop_front()
                };
                if let Some(task) = maybe_task {
                    let sem_clone = semaphore.clone();
                    let handle = tokio::spawn(async move {
                        let permit = sem_clone.acquire().await.unwrap();
                        (task)();
                        drop(permit);
                    });
                    let mut handlers = handlers.lock().unwrap();
                    handlers.push(handle);
                } else {
                    drop(permit);
                    let _ = tokio::time::sleep(tokio::time::Duration::from_millis(100));
                }

                // Clean the handlers
                {
                    let mut handlers = handlers.lock().unwrap();
                    handlers.retain(|h| !h.is_finished());
                }
            }
        });
    }

    pub async fn shutdown(&mut self) {
        {
            let mut running = self.running.write().unwrap();
            *running = false;
        }

        let handlers = self.handlers.lock().unwrap();
        for handler in handlers.iter() {
            handler.abort();
        }
    }

    pub fn is_finished(&self) -> bool {
        let handlers = self.handlers.lock().unwrap();
        if handlers.len() > 0 {
            return false;
        }

        let queue = self.queue.lock().unwrap(); 

        if queue.len() > 0 {
            return false;
        }
        return true;
    }
}

