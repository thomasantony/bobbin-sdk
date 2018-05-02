#![no_std]
#![feature(asm, lang_items, use_extern_macros, core_intrinsics, const_fn)]

#[cfg(target_os="none")]
#[macro_use]
pub extern crate cortex_m_rt;
pub extern crate bobbin_sys;
pub extern crate bobbin_sys_new;
pub extern crate stm32f74x as mcu;

pub use mcu::bobbin_bits;
pub use mcu::bobbin_mcu;
pub use mcu::bobbin_hal;

#[cfg(target_os="none")]
pub use cortex_m_rt::{default_handler, exception};
pub use bobbin_sys::{system, memory, heap, irq_dispatch, print, println, abort};
#[cfg(feature="logger")]
pub use bobbin_sys::logger;

#[cfg(target_os="none")]
mod lang_items;

pub mod prelude;
pub mod startup;
pub mod clock;
pub mod tick;
pub mod console;
pub mod led;
pub mod btn;
pub mod sys_new;

pub use startup::init;

use system::SystemProvider;

// pub type System = system::System<Mcu, Clk>;

pub type Mcu = mcu::Stm32f74x;
pub type Clk = clock::SystemClock;
pub type Heap = heap::Heap;

#[cfg(feature="logger")]
pub type Logger = logger::Logger;
pub type Dispatcher = irq_dispatch::IrqDispatcher<Mcu>;

#[cfg(target_os="none")]
default_handler!(handle_exception);

fn handle_exception() {
    use prelude::GetActiveIrq;
    let exc = Mcu::get_active_irq();
    if exc > 16 && Dispatcher::dispatch(exc.wrapping_sub(16)) {
        return
    } else {
        ::bobbin_sys::console::write(b"Unhandled Exception: 0x");
        ::bobbin_sys::console::write_u8_hex(exc);
        ::bobbin_sys::console::write(b"\r\n");
        unsafe { asm!("bkpt") };
        loop {}
    }
}

pub struct NucleoF746zg {}

pub type Board = NucleoF746zg;

impl SystemProvider for Board {
    type Mcu = mcu::Stm32f74x;
    type Clk = clock::SystemClock;
}


// New System

// use bobbin_sys_new::System;

pub fn sys_init() -> bobbin_sys_new::System<Board> {
    bobbin_sys_new::System::init()
}


