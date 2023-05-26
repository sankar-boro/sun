use std::sync::mpsc;
use std::thread;

enum RuntimeMessage {
    Task(Box<dyn FnOnce() + Send + 'static>),
    Terminate,
}

struct Runtime {
    sender: mpsc::Sender<RuntimeMessage>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Runtime {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let handle = thread::spawn(move || {
            while let Ok(message) = receiver.recv() {
                match message {
                    RuntimeMessage::Task(task) => task(),
                    RuntimeMessage::Terminate => break,
                }
            }
        });

        Runtime {
            sender,
            handle: Some(handle),
        }
    }

    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let message = RuntimeMessage::Task(Box::new(f));
        self.sender.send(message).expect("Failed to send task to the runtime.");
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        self.sender.send(RuntimeMessage::Terminate).ok();
        if let Some(handle) = self.handle.take() {
            handle.join().ok();
        }
    }
}

// Example usage:
fn main() {
    let runtime = Runtime::new();

    runtime.spawn(|| {
        println!("Task 1 running...");
        // Perform some work here
    });

    runtime.spawn(|| {
        println!("Task 2 running...");
        // Perform some work here
    });

    // The main thread will continue running concurrently with the spawned tasks

    // ...

    // The runtime will automatically terminate and join the worker thread
    // when it goes out of scope at the end of the program
}