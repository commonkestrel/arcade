#![allow(binary_asm_labels)]

pub mod graphics;
pub mod keyboard;

use core::arch::asm;
use alloc::fmt;
use embedded_time::Instant;

pub fn set_video_mode(mode: u8) {
    unsafe {
        asm!(
            "int 10h",
            inout("ax") mode as u16 => _
        )
    }
}

pub fn get_keyboard_input() -> u8 {
    let code;
    unsafe {
        asm!(
            "mov ah, 01h",
            "int 16h",
            "jz 1f",
            "mov ah, 00h",
            "int 16h",
            "mov al, ah",
            "xor ah, ah",
            "jmp 2f",
            "1:",
            "xor ax, ax",
            "2:",
            out("al") code,
        );
    }
    code
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemTime {
    hour: u8,
    minute: u8,
    second: u8,
    centisecond: u8,
}

impl SystemTime {
    pub fn now() -> Self {
        let hour: u8;
        let minute: u8;
        let second: u8;
        let centisecond: u8;

        unsafe {
            asm!(
                "int 21h",
                inout("ah") 0x2Ci8 => _,
                out("ch") hour,
                out("cl") minute,
                out("dh") second,
                out("dl") centisecond,
            )
        }

        Self {hour, minute, second, centisecond}
    }

    pub fn second(&self) -> u8 {
        self.second
    }

    pub fn centisecond(&self) -> u8 {
        self.centisecond
    }
}

impl fmt::Display for SystemTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}:{}.{}", self.hour, self.minute, self.second, self.centisecond)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemDate {
    year: u16,
    month: u8,
    day_of_month: u8,
    weekday: u8,
}

pub fn exit() {
    unsafe {
        asm!(
            "int 21h",
            inout("ax") 0x4C00 => _,
        );
    }
}
