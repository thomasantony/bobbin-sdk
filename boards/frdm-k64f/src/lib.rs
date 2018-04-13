#![no_std]
#![feature(asm, lang_items, use_extern_macros, core_intrinsics, const_fn)]

#[cfg(target_os="none")]
pub extern crate cortex_m_rt;
pub extern crate k64 as mcu;


pub use mcu::bobbin_common::{print, println};
pub use mcu::bobbin_common as common;

#[cfg(target_os="none")]
pub use cortex_m_rt::default_handler;

#[cfg(target_os="none")]
mod lang_items;
pub mod cache;
pub mod clock;
pub mod console;
pub mod led;
pub mod btn;
pub mod delay;
pub mod sys;

pub use delay::delay;

pub fn init() -> System {    
    System::init()
}

pub type System = sys::System<
        Mcu,
        Clock,
>;

pub type Mcu = mcu::K64;
pub type Clock = clock::SystemClock;
pub type Memory = mcu::bobbin_common::memory::Memory;
pub type Heap = mcu::bobbin_common::heap::Heap;
pub type Logger = mcu::bobbin_common::logger::Logger;
pub type Dispatcher = mcu::dispatch::Dispatcher<mcu::dispatch::ExcHandlers8>;

pub fn handle_exception() {
    unsafe {
        if !Dispatcher::dispatch(mcu::scb::SCB.icsr().vectactive().value()) {
            console::write_str("EXCEPTION\n");
            asm!("bkpt");
            loop {}
        }
    }
}

#[cfg(target_os="none")]
default_handler!(handle_exception);

#[derive(Debug, Default)]
pub struct FrdmK64f {}

impl common::board::Board for FrdmK64f {
   type Mcu = mcu::K64;
   fn id(&self) -> &'static str { "frdm-k64f" }
   fn mcu(&self) -> Self::Mcu { Self::Mcu::default() }
}

pub const fn board() -> FrdmK64f { FrdmK64f{} }

pub type Board = FrdmK64f;