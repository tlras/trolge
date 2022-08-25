#![no_main]
#![windows_subsystem = "windows"]

mod winapi;
mod routines;
mod sounds;
use std::{thread::{self, JoinHandle}};

#[no_mangle]
fn main(_argc: i32, _argv: *const *const u8) {
    let mut intensity = 0;
    let mut thread_pool = Vec::<JoinHandle<()>>::with_capacity(routines::THREADS.len());
    
    loop {
        for i in routines::THREADS {
            if i.1 == intensity {
                thread_pool.push(thread::spawn(i.0));
            }
        }

        thread::sleep(std::time::Duration::from_secs(60));
        intensity += 1;
    }
}
