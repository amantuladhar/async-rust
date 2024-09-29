use event_loop::*;
use std::time::Duration;

//TODO: benchmark timings - good place to learn flame graph and other
// performance check tools
fn main() {
    let el = EventLoop::new();
    el.schedule(|| println!("This is the first task"));
    el.schedule(|| {
        println!("Task 2 started");
        std::thread::sleep(Duration::from_secs(2));
        println!("Task 2 finished");
    });
    el.schedule(|| println!("Task 3 executed"));

    el.join();
}

type Task = Box<dyn FnOnce() + Send + 'static>;

mod thread_pool {
    use crate::Task;
    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::{mpsc, Arc, Condvar, Mutex};
    use std::thread::JoinHandle;

    pub struct Worker {
        id: usize,
        handle: Option<JoinHandle<()>>,
    }

    impl Worker {
        fn new(id: usize, receiver: Arc<(Mutex<Receiver<Task>>, Condvar)>) -> T {
            todo!()
        }
    }

    pub struct ThreadPool {
        workers: Vec<Worker>,
        sender: Sender<Task>,
    }

    impl ThreadPool {
        fn new(size: usize) -> Self {
            assert!(size > 0);
            let (sender, receiver) = mpsc::channel::<Task>();
            let receiver = Arc::new((Mutex::new(receiver), Condvar::new()));

            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            Self { workers, sender }
        }
    }
}

mod event_loop {
    use super::Task;
    use std::time::Duration;
    use std::{
        sync::mpsc::{self, Receiver, Sender},
        thread::JoinHandle,
    };

    pub struct EventLoop {
        scheduler: Sender<Task>,
        join_handle: JoinHandle<()>,
    }

    impl EventLoop {
        pub fn new() -> Self {
            let (scheduler, receiver) = mpsc::channel::<Task>();
            let join_handle = std::thread::spawn(move || Self::run_loop(receiver));
            EventLoop {
                scheduler,
                join_handle,
            }
        }

        pub fn run_loop(receiver: Receiver<Task>) {
            loop {
                match receiver.recv_timeout(Duration::from_secs(5)) {
                    Ok(handler_fn) => {
                        std::thread::spawn(|| handler_fn());
                    }
                    Err(_) => println!("INFO - looks like nothing on the queue"),
                }
            }
        }

        pub fn schedule<F>(&self, task: F)
        where
            F: FnOnce() + Send + 'static,
        {
            self.scheduler.send(Box::new(task)).unwrap();
        }

        pub(crate) fn join(self) {
            self.join_handle.join().unwrap();
        }
    }
}
