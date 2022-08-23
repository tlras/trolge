use crate::winapi;
use rand::prelude::SliceRandom;

static PING_SOUND: &'static [u8] = include_bytes!("../data/discord_ping.wav");
static SOUNDS: [&'static [u8]; 21] = [
    include_bytes!("../data/windows1.wav"),
    include_bytes!("../data/windows2.wav"),
    include_bytes!("../data/windows3.wav"),
    include_bytes!("../data/windows4.wav"),
    include_bytes!("../data/windows5.wav"),
    include_bytes!("../data/windows6.wav"),
    include_bytes!("../data/windows6.wav"),
    include_bytes!("../data/windows7.wav"),
    include_bytes!("../data/windows8.wav"),
    include_bytes!("../data/windows9.wav"),
    include_bytes!("../data/windows10.wav"),
    include_bytes!("../data/windows11.wav"),
    include_bytes!("../data/windows12.wav"),
    include_bytes!("../data/windows13.wav"),
    include_bytes!("../data/windows14.wav"),
    include_bytes!("../data/windows15.wav"),
    include_bytes!("../data/windows16.wav"),
    include_bytes!("../data/windows16.wav"),
    include_bytes!("../data/windows17.wav"),
    include_bytes!("../data/windows18.wav"),
    include_bytes!("../data/windows19.wav")
];

pub fn play_random() {
    if let Some(snd) = SOUNDS.choose(&mut rand::thread_rng()) {
        winapi::play_sound(std::ptr::addr_of!(snd[0]));
    }
}

pub fn play_ping() {
    winapi::play_sound(std::ptr::addr_of!(PING_SOUND[0]));
}