pub use stm32_common::ext::*;

use bobbin_mcu::mcu::{GetActiveIrq, IrqEnable, Sleep};
use nvic::NVIC;

pub mod rcc;
pub mod clock;

impl GetActiveIrq for ::Mcu {
    fn get_active_irq() -> u8 {
        get_active_irq()
    }
}

impl IrqEnable for ::Mcu {
    fn irq_enabled(irq: u8) -> bool { NVIC.enabled(irq) }
    fn irq_enable(irq: u8) { NVIC.set_enabled(irq, true); }
    fn irq_disable(irq: u8) { NVIC.set_enabled(irq, false); }
}

impl Sleep for ::Mcu {    
    fn sleep() { sleep() }
}

// impl Cortexm for ::Stm32f74x {}


// pub mod flash;
// pub mod pwr;
// pub mod syscfg;
// pub mod dbg;
// pub mod ethernet_mac;
// pub mod ethernet_mmc;
// pub mod ethernet_ptp;
// pub mod ethernet_dma;
// pub mod sdmmc;
// pub mod quadspi;
// pub mod cec;
// pub mod spdif_rx;
// pub mod ltdc;
// pub mod dma2d;
// pub mod hash;
// pub mod cryp;
// pub mod c_adc;
// pub mod dac;
// pub mod dcmi;
// pub mod usb_fs_global;
// pub mod usb_fs_host;
// pub mod usb_fs_device;
// pub mod usb_fs_pwrclk;
// pub mod iwdg;
// pub mod wwdg;
// pub mod crc;
// pub mod exti;
// pub mod tim_bas;
// pub mod tim_gen;
// pub mod tim_adv;
// pub mod lptim;
// pub mod adc;
// pub mod spi;
// pub mod i2c;
// pub mod can;
// pub mod gpio;
// pub mod usart;
// pub mod dma;

