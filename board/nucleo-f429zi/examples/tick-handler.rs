#![no_std]
#![no_main]

extern crate nucleo_f429zi as board;
extern crate examples;

#[no_mangle]
pub extern "C" fn main() -> ! {
    examples::tick_handler::run_with_sys(board::init())
}
