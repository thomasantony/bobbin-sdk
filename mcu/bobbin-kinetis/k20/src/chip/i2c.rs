#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::i2c::*;

periph!( I2C0, I2c0, _I2C0, I2cPeriph, 0x40066000);
periph!( I2C1, I2c1, _I2C1, I2cPeriph, 0x40067000);




