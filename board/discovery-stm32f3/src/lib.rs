#![no_std]
#![feature(asm, lang_items, use_extern_macros, core_intrinsics, const_fn)]

#[cfg(target_os="none")]
#[macro_use]
pub extern crate cortex_m_rt;
pub extern crate bobbin_sys;
pub extern crate stm32f3x as mcu;

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

pub use startup::init;

pub type System = system::System<Mcu, Clk>;

pub type Mcu = mcu::Stm32f3x;
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

pub struct DiscoveryStm32f3 {
    system: System,
}

impl bobbin_sys::board::Board for DiscoveryStm32f3 {
    type System = System;
    fn id(&self) -> &'static str { "discovery-stm32f3" }
    fn sys(&self) -> &System {
        &self.system
    }
    fn sys_mut(&mut self) -> &mut System {
        &mut self.system
    }
}

impl ::core::ops::Deref for DiscoveryStm32f3 {
    type Target = System;
    fn deref(&self) -> &Self::Target {
        &self.system
    }
}

impl ::core::ops::DerefMut for DiscoveryStm32f3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.system
    }
}

pub type Board = DiscoveryStm32f3;