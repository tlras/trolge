use std::{thread, time};
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use rand::Rng;
use crate::winapi;
use crate::sounds;

pub static THREADS: [(fn(), i32); 6] = [
    (keyboard_leds, 3),
    (drive_opener, 0),
    (mouse_wiggler, 3),
    (windows_noises, 0),
    (pause_play, 5),
    (discord_ping, 3),
];

pub fn keyboard_leds() {
    loop {
        winapi::key_press(VK_NUMLOCK);
        thread::sleep(time::Duration::from_millis(250));
        winapi::key_press(VK_NUMLOCK);

        winapi::key_press(VK_CAPITAL);
        thread::sleep(time::Duration::from_millis(250));
        winapi::key_press(VK_CAPITAL);

        winapi::key_press(VK_SCROLL);
        thread::sleep(time::Duration::from_millis(250));
        winapi::key_press(VK_SCROLL);

        winapi::key_press(VK_CAPITAL);
        thread::sleep(time::Duration::from_millis(250));
        winapi::key_press(VK_CAPITAL);
    }
}

pub fn drive_opener() {
    loop {
        winapi::open_drive();
        thread::sleep(time::Duration::from_millis(1000));
    }
}

pub fn mouse_wiggler() {
    loop {
        let mut pos = winapi::get_mouse_position();
        pos.0 += if rand::random() {-1} else {1};
        pos.1 += if rand::random() {-1} else {1};
        winapi::set_mouse_position(pos);
        thread::sleep(time::Duration::from_millis(18));
    }
}

pub fn windows_noises() {
    loop {
        thread::sleep(time::Duration::from_secs(rand::thread_rng().gen_range(2..=5) * 60));
        winapi::key_press(VK_INSERT);
        sounds::play_win();
    }
}

pub fn pause_play() {
    loop {
        winapi::key_press(VK_MEDIA_PLAY_PAUSE);
        thread::sleep(time::Duration::from_millis(100));
    }
}

pub fn discord_ping() {
    loop {
        thread::sleep(time::Duration::from_secs(rand::thread_rng().gen_range(2..=10) * 60));
        sounds::play_annoy();
    }
}