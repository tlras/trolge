use windows::{core::*, Win32::Foundation::*};
use windows::Win32::{Media::{Audio::PlaySoundA, Multimedia::mciSendStringA}, UI::Input::KeyboardAndMouse};
use windows::Win32::UI::WindowsAndMessaging;
use windows::Win32::System::SystemServices::SND_MEMORY;

pub fn get_mouse_position() -> (i32, i32) {
    unsafe {
        let mut p = POINT {
            x: 0,
            y: 0
        };

        WindowsAndMessaging::GetCursorPos(&mut p);
        (p.x, p.y)
    }
}

pub fn set_mouse_position(pos: (i32, i32)) {
    unsafe {
        WindowsAndMessaging::SetCursorPos(pos.0, pos.1);
    }
}

pub fn key_press(vk: KeyboardAndMouse::VIRTUAL_KEY) {
    unsafe {
        KeyboardAndMouse::keybd_event(vk.0 as u8, 0, KeyboardAndMouse::KEYEVENTF_KEYUP, 0);
        KeyboardAndMouse::keybd_event(vk.0 as u8, 0, KeyboardAndMouse::KEYBD_EVENT_FLAGS(0), 0);
    }
}

pub fn open_drive() {
    unsafe {
        let mut empty: [u8; 0] = [0; 0];
        mciSendStringA(s!("set cdaudio door open"), &mut empty, HWND(0));
    }
}

pub fn play_sound(ptr: *const u8) {
    unsafe {
        PlaySoundA(PCSTR(ptr), HINSTANCE(0), SND_MEMORY.0);
    }
}