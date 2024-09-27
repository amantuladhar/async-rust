use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread::JoinHandle,
    time::Duration,
};

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

struct EventLoop {
    scheduler: Sender<Task>,
    join_handle: JoinHandle<()>,
}

impl EventLoop {
    fn new() -> Self {
        let (scheduler, receiver) = mpsc::channel::<Task>();
        let join_handle = std::thread::spawn(move || Self::run_loop(receiver));
        EventLoop {
            scheduler,
            join_handle,
        }
    }

    fn run_loop(receiver: Receiver<Task>) {
        loop {
            match receiver.recv_timeout(Duration::from_secs(5)) {
                Ok(handler_fn) => handler_fn(),
                Err(_) => println!("INFO - looks like nothing on the queue"),
            }
        }
    }

    fn schedule<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.scheduler.send(Box::new(task)).unwrap();
    }

    fn join(self) {
        self.join_handle.join().unwrap();
    }
}
