#![allow(unused)]

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed};
use std::thread;
use std::time::Duration;

fn test() {
    let num: AtomicUsize = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        s.spawn(||{
            for i in 0..100 {
                println!("num: {i}");
                num.store(i + 1, Relaxed);
                main_thread.unpark();
            }
        });

        loop {
            let c = num.load(Relaxed);
            if c == 100 {
                break;
            }
            println!("Working.. {c}/100 done.");
            thread::park_timeout(Duration::from_secs(1));
            println!("timeout_done");
        }
    });
}

fn main() {
    let num: &AtomicUsize = &AtomicUsize::new(0);

    thread::scope(|s| {
        for x in 0..4 {
            s.spawn(move ||{
                for i in 0..25 {
                    num.fetch_add(1, Relaxed);
                }
            });
        }

        loop {
            let c = num.load(Relaxed);
            if c == 100 {
                break;
            }
            println!("Working.. {c}/100 done.");
            thread::sleep(Duration::from_secs(1));
            println!("woke up after 1 second.");
        }
    });

    println!("All tasks done.");
}