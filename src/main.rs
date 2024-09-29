use std::sync::{mpsc, Arc, Mutex, Condvar};
use std::thread::{self, JoinHandle};
use std::time::Duration;

type Task = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Task>,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new((Mutex::new(receiver), Condvar::new()));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<(Mutex<mpsc::Receiver<Task>>, Condvar)>) -> Worker {
        let thread = thread::spawn(move || {
            let (lock, cvar) = &*receiver;
            loop {
                let mut receiver_guard = lock.lock().unwrap();
                let result = cvar.wait_timeout_while(
                    receiver_guard,
                    Duration::from_secs(5),
                    |_| true
                );

                match result {
                    Ok((guard, timeout_result)) => {
                        receiver_guard = guard;
                        if timeout_result.timed_out() {
                            println!("Worker {} timed out waiting for job", id);
                            continue;
                        }
                    }
                    Err(_) => {
                        println!("Worker {} encountered an error while waiting", id);
                        break;
                    }
                }

                match receiver_guard.try_recv() {
                    Ok(job) => {
                        println!("Worker {} got a job; executing.", id);
                        drop(receiver_guard);
                        job();
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        println!("Worker {} woke up but no job available", id);
                    }
                    Err(mpsc::TryRecvError::Disconnected) => {
                        println!("Worker {} channel disconnected", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

struct EventLoop {
    scheduler: mpsc::Sender<Task>,
    join_handle: JoinHandle<()>,
}

impl EventLoop {
    fn new(pool_size: usize) -> Self {
        let (scheduler, receiver) = mpsc::channel::<Task>();
        let pool = ThreadPool::new(pool_size);
        let join_handle = std::thread::spawn(move || Self::run_loop(receiver, pool));
        EventLoop {
            scheduler,
            join_handle,
        }
    }

    fn run_loop(receiver: mpsc::Receiver<Task>, pool: ThreadPool) {
        loop {
            match receiver.recv_timeout(Duration::from_secs(5)) {
                Ok(task) => {
                    pool.execute(task);
                }
                Err(_) => println!("EventLoop - No tasks received in the last 5 seconds"),
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

fn main() {
    let el = EventLoop::new(4); // Create an event loop with a thread pool of 4 threads
    el.schedule(|| println!("This is the first task"));
    el.schedule(|| {
        println!("Task 2 started");
        std::thread::sleep(Duration::from_secs(2));
        println!("Task 2 finished");
    });
    el.schedule(|| println!("Task 3 executed"));

    // Add more tasks to demonstrate concurrent execution
    for i in 4..10 {
        el.schedule(move || {
            println!("Task {} started", i);
            std::thread::sleep(Duration::from_millis(500));
            println!("Task {} finished", i);
        });
    }

    // Keep the main thread alive for a while to allow tasks to complete
    std::thread::sleep(Duration::from_secs(10));

    el.join();
}