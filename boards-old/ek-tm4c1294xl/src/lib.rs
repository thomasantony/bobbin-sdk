#![no_std]
#![feature(asm, lang_items, global_allocator)]

extern crate r0;
extern crate log;

#[macro_use] pub mod itm;
#[macro_use] pub mod console;
#[macro_use] pub mod logger;

extern crate tm4c129x;
pub use tm4c129x::{chip, hal, cortexm, common};

pub mod exceptions;
#[cfg(target_os="none")]
pub mod lang_items;

pub mod clock;
pub mod led;
pub mod btn;
//pub mod pin;
pub mod tim;
//pub mod uart;

pub use common::heap::Heap;

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

pub unsafe fn init_allocator(buf: &'static mut [u8]) {
    ALLOCATOR.init(buf);
}

pub use tim::delay;

// pub fn delay(n: u32) {
//     for _ in 0..25_000*n {
//         unsafe { asm!("nop") }
//     }
// }



pub fn init() {
    clock::init();
    led::init();
    btn::init();
    tim::init();
    console::init();
}