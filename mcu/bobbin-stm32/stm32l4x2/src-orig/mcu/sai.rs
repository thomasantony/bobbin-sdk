#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::sai::*;

periph!( SAI1, Sai1, SAI1_PERIPH, SaiPeriph, 0x40015400, 0x1d);
