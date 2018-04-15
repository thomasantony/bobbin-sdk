pub use mcu::bobbin_common::sys::console::*;
use mcu::bobbin_common::periph::IntoPeriph;

use mcu::rcc::*;
use mcu::usart::*;
use mcu::pin::*;

pub const USART: Usart2 = USART2;
pub const USART_TX: Pa2 = PA2;
pub const USART_RX: Pa15 = PA15;
const USART_CLOCK: u32 = 16_000_000; // Use HSI Clock
const USART_BAUD: u32 = 115_200;


pub fn init() {
    USART_TX
        .port_gate_enable()
        .connect_to(USART);

    USART_RX
        .port_gate_enable()
        .connect_to(USART);

    USART
        .set_clock_source(DedicatedClock::Hsi16)
        .gate_enable()
        .set_config(|c| c.set_baud(USART_BAUD, USART_CLOCK))
        .enable();

    set_console(Console::new(USART.into_periph()));
}