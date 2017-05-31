use core::fmt::{self, Write, Arguments};
use chip::sig::*;
use hal::gpio::*;
use hal::usart::*;
use hal::rcc;

/// Macro for sending `print!`-formatted messages over the Console
#[macro_export]
macro_rules! print {
    ($s:expr) => {
        $crate::console::write_str($s)
    };
    ($($arg:tt)*) => {
        $crate::console::write_fmt(format_args!($($arg)*))
    };
}

/// Macro for sending `print!`-formatted messages over the Console, with a
/// newline
#[macro_export]
macro_rules! println {
    ($fmt:expr) => {
        print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!(concat!($fmt, "\n"), $($arg)*)
    };
}

pub const CONSOLE: Console = Console {};

pub struct Console {}

impl Console {
    pub fn init(&self) {
        let tx = PD8;
        let rx = PD9;

        rcc::enable(&USART3);
        rcc::enable(&tx.port());
        rcc::enable(&rx.port());
        tx.mode_altfn(AltFn::<Usart3Tx>::alt_fn(&tx));
        rx.mode_altfn(AltFn::<Usart3Rx>::alt_fn(&rx));

        USART3.enable(30_000_000 / 115_200);
    }

    pub fn usart(&self) -> Usart3 {
        USART3
    }
}

impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {        
        let usart = self.usart();
        for byte in s.as_bytes().iter().cloned() {
            if byte == b'\n' {
                usart.putc(b'\r')
            }
            usart.putc(byte)
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn write_fmt(args: Arguments) {    
    CONSOLE.write_fmt(args).ok();
}

#[doc(hidden)]
pub fn write_str(s: &str) {
    CONSOLE.write_str(s).ok();
}

