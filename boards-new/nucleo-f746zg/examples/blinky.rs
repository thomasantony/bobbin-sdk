#![no_std]
#![no_main]
#![feature(asm)]

extern crate nucleo_f746zg as board;

use board::led::*;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();

    loop {
        LED0.toggle_output();
        board::delay(500);
    }
}