#![allow(unused)]

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::runtime::Runtime;

// Your async function
async fn async_function() -> u32 {
    // Perform some asynchronous operations
    // and return a result
    42
}

// Wrapper struct to hold the async function
struct AsyncWrapper {
    inner_future: Pin<Box<dyn Future<Output = u32> + Send>>,
}

impl Future for AsyncWrapper {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Forward the poll call to the inner future
        self.get_mut().inner_future.as_mut().poll(cx)
    }
}

fn main() {
    // Create a new tokio runtime
    let mut runtime = Runtime::new().unwrap();

    // Wrap the async function inside a Future
    let async_wrapper = AsyncWrapper {
        inner_future: Box::pin(async_function()),
    };

    // Run the future using the tokio runtime
    let result = runtime.block_on(async_wrapper);

    println!("Result: {:?}", result);
}