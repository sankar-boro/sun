use std::collections::LinkedList;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::future::Future;

struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Task {
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        self.future.as_mut().poll(cx)
    }
}

type TaskQueue = Arc<Mutex<LinkedList<Task>>>;

struct Executor {
    task_queue: TaskQueue,
}

impl Executor {
    fn new() -> Self {
        Executor {
            task_queue: Arc::new(Mutex::new(LinkedList::new())),
        }
    }

    fn spawn(&self, future: impl Future<Output = ()> + 'static) {
        let task = Task::new(future);
        self.task_queue.lock().unwrap().push_back(task);
    }

    fn run(&self) {
        while let Some(mut task) = self.task_queue.lock().unwrap().pop_front() {
            let waker = waker_ref(&task);
            let mut cx = Context::from_waker(&waker);
            if let Poll::Pending = task.poll(&mut cx) {
                self.task_queue.lock().unwrap().push_back(task);
            }
        }
    }
}

fn waker_ref(task: &Task) -> Waker {
    let task_ptr = task as *const Task;
    let raw_waker = RawWaker::new(task_ptr as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}

unsafe fn clone_waker(_: *const ()) -> RawWaker {
    unreachable!("Clone is not needed for our executor")
}

unsafe fn wake(_: *const ()) {
    unreachable!("Wake is not needed for our executor")
}

unsafe fn wake_by_ref(_: *const ()) {
    unreachable!("WakeByRef is not needed for our executor")
}

unsafe fn drop_waker(_: *const ()) {
    // We don't need to do anything since the Task will be dropped naturally
}

const VTABLE: RawWakerVTable = RawWakerVTable::new(clone_waker, wake, wake_by_ref, drop_waker);

// Example usage

async fn do_something() -> String {
    "sankar".to_string()
}

async fn async_task() {
    println!("Starting async task");
    let d = do_something().await;
    println!("{}", d);
    // Simulating some asynchronous work
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Async task completed");
}

fn main() {
    let executor = Executor::new();

    // Spawn an asynchronous task
    executor.spawn(async_task());

    // Run the executor
    executor.run();
}