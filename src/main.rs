mod winapi;
mod routines;
mod sounds;
use std::{thread::{self, JoinHandle}};

fn main() {
    let mut thread_pool = Vec::<JoinHandle<()>>::with_capacity(routines::THREADS.len());
    for i in routines::THREADS {
        thread_pool.push(thread::spawn(i));
    }

    for i in thread_pool {
        i.join().unwrap();
    }
}
