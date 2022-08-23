#![no_main]
#![windows_subsystem = "windows"]

mod winapi;
mod routines;
mod sounds;
use std::{thread::{self, JoinHandle}};

#[no_mangle]
fn main(_argc: i32, _argv: *const *const u8) {
    let mut thread_pool = Vec::<JoinHandle<()>>::with_capacity(routines::THREADS.len());
    for i in routines::THREADS {
        thread_pool.push(thread::spawn(i));
    }

    for i in thread_pool {
        i.join().unwrap();
    }
}
