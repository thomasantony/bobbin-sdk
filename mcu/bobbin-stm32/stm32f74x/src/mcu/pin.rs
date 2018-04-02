#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::bobbin_common::pin::*;
pub use ::bobbin_common::gate::*;

pub use super::gpio::*;

pin!(PA0, Pa0, pa0, GPIOA, Gpioa, PA0_PIN, GpioPin, GPIOA_PERIPH, PA0_OWNED, 0);
   pin_source!(Pa0, super::adc::Adc1Ch0, super::sig::SigAdc, 0);
   pin_source!(Pa0, super::adc::Adc2Ch0, super::sig::SigAdc, 0);
   pin_source!(Pa0, super::adc::Adc3Ch0, super::sig::SigAdc, 0);
   pin_source!(Pa0, super::tim_gen::Tim2Ch1, super::sig::SigTim, 1);
   pin_source!(Pa0, super::tim_gen::Tim5Ch1, super::sig::SigTim, 2);
   pin_source!(Pa0, super::usart::Usart2, super::sig::SigUsartCts, 7);
   pin_source!(Pa0, super::usart::Uart4, super::sig::SigUsartTx, 8);
pin!(PA1, Pa1, pa1, GPIOA, Gpioa, PA1_PIN, GpioPin, GPIOA_PERIPH, PA1_OWNED, 1);
   pin_source!(Pa1, super::adc::Adc1Ch1, super::sig::SigAdc, 0);
   pin_source!(Pa1, super::adc::Adc2Ch1, super::sig::SigAdc, 0);
   pin_source!(Pa1, super::adc::Adc3Ch1, super::sig::SigAdc, 0);
   pin_source!(Pa1, super::tim_gen::Tim2Ch2, super::sig::SigTim, 1);
   pin_source!(Pa1, super::tim_gen::Tim5Ch2, super::sig::SigTim, 2);
   pin_source!(Pa1, super::usart::Usart2, super::sig::SigUsartRts, 7);
   pin_source!(Pa1, super::usart::Uart4, super::sig::SigUsartRx, 8);
pin!(PA2, Pa2, pa2, GPIOA, Gpioa, PA2_PIN, GpioPin, GPIOA_PERIPH, PA2_OWNED, 2);
   pin_source!(Pa2, super::adc::Adc1Ch2, super::sig::SigAdc, 0);
   pin_source!(Pa2, super::adc::Adc2Ch2, super::sig::SigAdc, 0);
   pin_source!(Pa2, super::adc::Adc3Ch2, super::sig::SigAdc, 0);
   pin_source!(Pa2, super::tim_gen::Tim2Ch3, super::sig::SigTim, 1);
   pin_source!(Pa2, super::tim_gen::Tim5Ch3, super::sig::SigTim, 2);
   pin_source!(Pa2, super::tim_gen::Tim9Ch1, super::sig::SigTim, 3);
   pin_source!(Pa2, super::usart::Usart2, super::sig::SigUsartTx, 7);
pin!(PA3, Pa3, pa3, GPIOA, Gpioa, PA3_PIN, GpioPin, GPIOA_PERIPH, PA3_OWNED, 3);
   pin_source!(Pa3, super::adc::Adc1Ch3, super::sig::SigAdc, 0);
   pin_source!(Pa3, super::adc::Adc2Ch3, super::sig::SigAdc, 0);
   pin_source!(Pa3, super::adc::Adc3Ch3, super::sig::SigAdc, 0);
   pin_source!(Pa3, super::tim_gen::Tim2Ch4, super::sig::SigTim, 1);
   pin_source!(Pa3, super::tim_gen::Tim5Ch4, super::sig::SigTim, 2);
   pin_source!(Pa3, super::tim_gen::Tim9Ch2, super::sig::SigTim, 3);
   pin_source!(Pa3, super::usart::Usart2, super::sig::SigUsartRx, 7);
pin!(PA4, Pa4, pa4, GPIOA, Gpioa, PA4_PIN, GpioPin, GPIOA_PERIPH, PA4_OWNED, 4);
   pin_source!(Pa4, super::adc::Adc1Ch4, super::sig::SigAdc, 0);
   pin_source!(Pa4, super::adc::Adc2Ch4, super::sig::SigAdc, 0);
   pin_source!(Pa4, super::dac::DacCh1, super::sig::SigDac, 0);
   pin_source!(Pa4, super::spi::Spi1, super::sig::SigSpiNss, 5);
   pin_source!(Pa4, super::spi::Spi3, super::sig::SigSpiNss, 6);
   pin_source!(Pa4, super::usart::Usart2, super::sig::SigUsartCk, 7);
   pin_source!(Pa4, super::spi::Spi6, super::sig::SigSpiNss, 8);
   pin_source!(Pa4, super::dcmi::Dcmi, super::sig::SigDcmiHsync, 13);
pin!(PA5, Pa5, pa5, GPIOA, Gpioa, PA5_PIN, GpioPin, GPIOA_PERIPH, PA5_OWNED, 5);
   pin_source!(Pa5, super::adc::Adc1Ch5, super::sig::SigAdc, 0);
   pin_source!(Pa5, super::adc::Adc2Ch5, super::sig::SigAdc, 0);
   pin_source!(Pa5, super::dac::DacCh2, super::sig::SigDac, 0);
   pin_source!(Pa5, super::tim_gen::Tim2Ch1, super::sig::SigTim, 1);
   pin_source!(Pa5, super::spi::Spi1, super::sig::SigSpiSck, 5);
   pin_source!(Pa5, super::spi::Spi6, super::sig::SigSpiSck, 8);
pin!(PA6, Pa6, pa6, GPIOA, Gpioa, PA6_PIN, GpioPin, GPIOA_PERIPH, PA6_OWNED, 6);
   pin_source!(Pa6, super::adc::Adc1Ch6, super::sig::SigAdc, 0);
   pin_source!(Pa6, super::adc::Adc2Ch6, super::sig::SigAdc, 0);
   pin_source!(Pa6, super::tim_gen::Tim3Ch1, super::sig::SigTim, 2);
   pin_source!(Pa6, super::spi::Spi1, super::sig::SigSpiMiso, 5);
   pin_source!(Pa6, super::spi::Spi6, super::sig::SigSpiMiso, 8);
   pin_source!(Pa6, super::tim_gen::Tim13Ch1, super::sig::SigTim, 9);
   pin_source!(Pa6, super::dcmi::Dcmi, super::sig::SigDcmiPixclk, 13);
pin!(PA7, Pa7, pa7, GPIOA, Gpioa, PA7_PIN, GpioPin, GPIOA_PERIPH, PA7_OWNED, 7);
   pin_source!(Pa7, super::adc::Adc1Ch7, super::sig::SigAdc, 0);
   pin_source!(Pa7, super::adc::Adc2Ch7, super::sig::SigAdc, 0);
   pin_source!(Pa7, super::tim_gen::Tim3Ch2, super::sig::SigTim, 2);
   pin_source!(Pa7, super::spi::Spi1, super::sig::SigSpiMosi, 5);
   pin_source!(Pa7, super::spi::Spi6, super::sig::SigSpiMosi, 8);
   pin_source!(Pa7, super::tim_gen::Tim14Ch1, super::sig::SigTim, 9);
pin!(PA8, Pa8, pa8, GPIOA, Gpioa, PA8_PIN, GpioPin, GPIOA_PERIPH, PA8_OWNED, 8);
   pin_source!(Pa8, super::tim_adv::Tim1Ch1, super::sig::SigTim, 1);
   pin_source!(Pa8, super::i2c::I2c3, super::sig::SigI2cScl, 4);
   pin_source!(Pa8, super::usart::Usart1, super::sig::SigUsartCk, 7);
   pin_source!(Pa8, super::usart::Uart7, super::sig::SigUsartRx, 12);
pin!(PA9, Pa9, pa9, GPIOA, Gpioa, PA9_PIN, GpioPin, GPIOA_PERIPH, PA9_OWNED, 9);
   pin_source!(Pa9, super::tim_adv::Tim1Ch2, super::sig::SigTim, 1);
   pin_source!(Pa9, super::spi::Spi2, super::sig::SigSpiSck, 5);
   pin_source!(Pa9, super::usart::Usart1, super::sig::SigUsartTx, 7);
   pin_source!(Pa9, super::dcmi::Dcmi, super::sig::SigDcmiD0, 13);
pin!(PA10, Pa10, pa10, GPIOA, Gpioa, PA10_PIN, GpioPin, GPIOA_PERIPH, PA10_OWNED, 10);
   pin_source!(Pa10, super::tim_adv::Tim1Ch3, super::sig::SigTim, 1);
   pin_source!(Pa10, super::usart::Usart1, super::sig::SigUsartRx, 7);
   pin_source!(Pa10, super::usb_fs_device::UsbFsDevice, super::sig::SigUsbId, 10);
   pin_source!(Pa10, super::usb_fs_host::UsbFsHost, super::sig::SigUsbId, 10);
   pin_source!(Pa10, super::dcmi::Dcmi, super::sig::SigDcmiD1, 13);
pin!(PA11, Pa11, pa11, GPIOA, Gpioa, PA11_PIN, GpioPin, GPIOA_PERIPH, PA11_OWNED, 11);
   pin_source!(Pa11, super::tim_adv::Tim1Ch4, super::sig::SigTim, 1);
   pin_source!(Pa11, super::spi::Spi2, super::sig::SigSpiNss, 5);
   pin_source!(Pa11, super::usart::Uart4, super::sig::SigUsartRx, 6);
   pin_source!(Pa11, super::usart::Usart1, super::sig::SigUsartCts, 7);
   pin_source!(Pa11, super::can::Can1, super::sig::SigCanRx, 9);
   pin_source!(Pa11, super::usb_fs_device::UsbFsDevice, super::sig::SigUsbDm, 10);
   pin_source!(Pa11, super::usb_fs_host::UsbFsHost, super::sig::SigUsbDm, 10);
pin!(PA12, Pa12, pa12, GPIOA, Gpioa, PA12_PIN, GpioPin, GPIOA_PERIPH, PA12_OWNED, 12);
   pin_source!(Pa12, super::spi::Spi2, super::sig::SigSpiSck, 5);
   pin_source!(Pa12, super::usart::Uart4, super::sig::SigUsartTx, 6);
   pin_source!(Pa12, super::usart::Usart1, super::sig::SigUsartRts, 7);
   pin_source!(Pa12, super::can::Can1, super::sig::SigCanTx, 9);
   pin_source!(Pa12, super::usb_fs_device::UsbFsDevice, super::sig::SigUsbDp, 10);
   pin_source!(Pa12, super::usb_fs_host::UsbFsHost, super::sig::SigUsbDp, 10);
pin!(PA13, Pa13, pa13, GPIOA, Gpioa, PA13_PIN, GpioPin, GPIOA_PERIPH, PA13_OWNED, 13);
pin!(PA14, Pa14, pa14, GPIOA, Gpioa, PA14_PIN, GpioPin, GPIOA_PERIPH, PA14_OWNED, 14);
pin!(PA15, Pa15, pa15, GPIOA, Gpioa, PA15_PIN, GpioPin, GPIOA_PERIPH, PA15_OWNED, 15);
   pin_source!(Pa15, super::tim_gen::Tim2Ch1, super::sig::SigTim, 1);
   pin_source!(Pa15, super::spi::Spi1, super::sig::SigSpiNss, 5);
   pin_source!(Pa15, super::spi::Spi3, super::sig::SigSpiNss, 6);
   pin_source!(Pa15, super::spi::Spi6, super::sig::SigSpiNss, 7);
   pin_source!(Pa15, super::usart::Uart4, super::sig::SigUsartRts, 8);
   pin_source!(Pa15, super::usart::Uart7, super::sig::SigUsartTx, 12);
pin!(PB0, Pb0, pb0, GPIOB, Gpiob, PB0_PIN, GpioPin, GPIOB_PERIPH, PB0_OWNED, 0);
   pin_source!(Pb0, super::adc::Adc1Ch8, super::sig::SigAdc, 0);
   pin_source!(Pb0, super::adc::Adc2Ch8, super::sig::SigAdc, 0);
   pin_source!(Pb0, super::tim_gen::Tim3Ch3, super::sig::SigTim, 2);
   pin_source!(Pb0, super::usart::Uart4, super::sig::SigUsartCts, 8);
pin!(PB1, Pb1, pb1, GPIOB, Gpiob, PB1_PIN, GpioPin, GPIOB_PERIPH, PB1_OWNED, 1);
   pin_source!(Pb1, super::adc::Adc1Ch9, super::sig::SigAdc, 0);
   pin_source!(Pb1, super::adc::Adc2Ch9, super::sig::SigAdc, 0);
   pin_source!(Pb1, super::tim_gen::Tim3Ch4, super::sig::SigTim, 2);
pin!(PB2, Pb2, pb2, GPIOB, Gpiob, PB2_PIN, GpioPin, GPIOB_PERIPH, PB2_OWNED, 2);
   pin_source!(Pb2, super::spi::Spi3, super::sig::SigSpiMosi, 7);
pin!(PB3, Pb3, pb3, GPIOB, Gpiob, PB3_PIN, GpioPin, GPIOB_PERIPH, PB3_OWNED, 3);
   pin_source!(Pb3, super::tim_gen::Tim2Ch2, super::sig::SigTim, 1);
   pin_source!(Pb3, super::spi::Spi1, super::sig::SigSpiSck, 5);
   pin_source!(Pb3, super::spi::Spi3, super::sig::SigSpiSck, 6);
   pin_source!(Pb3, super::spi::Spi6, super::sig::SigSpiSck, 8);
   pin_source!(Pb3, super::usart::Uart7, super::sig::SigUsartRx, 12);
pin!(PB4, Pb4, pb4, GPIOB, Gpiob, PB4_PIN, GpioPin, GPIOB_PERIPH, PB4_OWNED, 4);
   pin_source!(Pb4, super::tim_gen::Tim3Ch1, super::sig::SigTim, 2);
   pin_source!(Pb4, super::spi::Spi1, super::sig::SigSpiMiso, 5);
   pin_source!(Pb4, super::spi::Spi3, super::sig::SigSpiMiso, 6);
   pin_source!(Pb4, super::spi::Spi2, super::sig::SigSpiNss, 7);
   pin_source!(Pb4, super::spi::Spi6, super::sig::SigSpiMiso, 8);
   pin_source!(Pb4, super::usart::Uart7, super::sig::SigUsartTx, 12);
pin!(PB5, Pb5, pb5, GPIOB, Gpiob, PB5_PIN, GpioPin, GPIOB_PERIPH, PB5_OWNED, 5);
   pin_source!(Pb5, super::usart::Uart5, super::sig::SigUsartRx, 1);
   pin_source!(Pb5, super::tim_gen::Tim3Ch2, super::sig::SigTim, 2);
   pin_source!(Pb5, super::spi::Spi1, super::sig::SigSpiMosi, 5);
   pin_source!(Pb5, super::spi::Spi3, super::sig::SigSpiMosi, 6);
   pin_source!(Pb5, super::spi::Spi6, super::sig::SigSpiMosi, 8);
   pin_source!(Pb5, super::can::Can2, super::sig::SigCanRx, 9);
   pin_source!(Pb5, super::dcmi::Dcmi, super::sig::SigDcmiD10, 13);
pin!(PB6, Pb6, pb6, GPIOB, Gpiob, PB6_PIN, GpioPin, GPIOB_PERIPH, PB6_OWNED, 6);
   pin_source!(Pb6, super::usart::Uart5, super::sig::SigUsartTx, 1);
   pin_source!(Pb6, super::tim_gen::Tim4Ch1, super::sig::SigTim, 2);
   pin_source!(Pb6, super::i2c::I2c1, super::sig::SigI2cScl, 4);
   pin_source!(Pb6, super::usart::Usart1, super::sig::SigUsartTx, 7);
   pin_source!(Pb6, super::can::Can2, super::sig::SigCanTx, 9);
   pin_source!(Pb6, super::i2c::I2c4, super::sig::SigI2cScl, 11);
   pin_source!(Pb6, super::dcmi::Dcmi, super::sig::SigDcmiD5, 13);
pin!(PB7, Pb7, pb7, GPIOB, Gpiob, PB7_PIN, GpioPin, GPIOB_PERIPH, PB7_OWNED, 7);
   pin_source!(Pb7, super::tim_gen::Tim4Ch2, super::sig::SigTim, 2);
   pin_source!(Pb7, super::i2c::I2c1, super::sig::SigI2cSda, 4);
   pin_source!(Pb7, super::usart::Usart1, super::sig::SigUsartRx, 7);
   pin_source!(Pb7, super::dcmi::Dcmi, super::sig::SigDcmiVsync, 13);
pin!(PB8, Pb8, pb8, GPIOB, Gpiob, PB8_PIN, GpioPin, GPIOB_PERIPH, PB8_OWNED, 8);
   pin_source!(Pb8, super::i2c::I2c4, super::sig::SigI2cScl, 1);
   pin_source!(Pb8, super::tim_gen::Tim4Ch3, super::sig::SigTim, 2);
   pin_source!(Pb8, super::tim_gen::Tim10Ch1, super::sig::SigTim, 3);
   pin_source!(Pb8, super::i2c::I2c1, super::sig::SigI2cScl, 4);
   pin_source!(Pb8, super::usart::Uart5, super::sig::SigUsartRx, 7);
   pin_source!(Pb8, super::can::Can1, super::sig::SigCanRx, 9);
   pin_source!(Pb8, super::dcmi::Dcmi, super::sig::SigDcmiD6, 13);
pin!(PB9, Pb9, pb9, GPIOB, Gpiob, PB9_PIN, GpioPin, GPIOB_PERIPH, PB9_OWNED, 9);
   pin_source!(Pb9, super::i2c::I2c4, super::sig::SigI2cSda, 1);
   pin_source!(Pb9, super::tim_gen::Tim4Ch4, super::sig::SigTim, 2);
   pin_source!(Pb9, super::tim_gen::Tim11Ch1, super::sig::SigTim, 3);
   pin_source!(Pb9, super::i2c::I2c1, super::sig::SigI2cSda, 4);
   pin_source!(Pb9, super::spi::Spi2, super::sig::SigSpiNss, 5);
   pin_source!(Pb9, super::usart::Uart5, super::sig::SigUsartTx, 7);
   pin_source!(Pb9, super::can::Can1, super::sig::SigCanTx, 9);
   pin_source!(Pb9, super::dcmi::Dcmi, super::sig::SigDcmiD7, 13);
pin!(PB10, Pb10, pb10, GPIOB, Gpiob, PB10_PIN, GpioPin, GPIOB_PERIPH, PB10_OWNED, 10);
   pin_source!(Pb10, super::tim_gen::Tim2Ch3, super::sig::SigTim, 1);
   pin_source!(Pb10, super::i2c::I2c2, super::sig::SigI2cScl, 4);
   pin_source!(Pb10, super::spi::Spi2, super::sig::SigSpiSck, 5);
   pin_source!(Pb10, super::usart::Usart3, super::sig::SigUsartTx, 7);
pin!(PB11, Pb11, pb11, GPIOB, Gpiob, PB11_PIN, GpioPin, GPIOB_PERIPH, PB11_OWNED, 11);
   pin_source!(Pb11, super::tim_gen::Tim2Ch4, super::sig::SigTim, 1);
   pin_source!(Pb11, super::i2c::I2c2, super::sig::SigI2cSda, 4);
   pin_source!(Pb11, super::usart::Usart3, super::sig::SigUsartRx, 7);
pin!(PB12, Pb12, pb12, GPIOB, Gpiob, PB12_PIN, GpioPin, GPIOB_PERIPH, PB12_OWNED, 12);
   pin_source!(Pb12, super::spi::Spi2, super::sig::SigSpiNss, 5);
   pin_source!(Pb12, super::usart::Usart3, super::sig::SigUsartCk, 7);
   pin_source!(Pb12, super::usart::Uart5, super::sig::SigUsartRx, 8);
   pin_source!(Pb12, super::can::Can2, super::sig::SigCanRx, 9);
pin!(PB13, Pb13, pb13, GPIOB, Gpiob, PB13_PIN, GpioPin, GPIOB_PERIPH, PB13_OWNED, 13);
   pin_source!(Pb13, super::spi::Spi2, super::sig::SigSpiSck, 5);
   pin_source!(Pb13, super::usart::Usart3, super::sig::SigUsartCts, 7);
   pin_source!(Pb13, super::usart::Uart5, super::sig::SigUsartTx, 8);
   pin_source!(Pb13, super::can::Can2, super::sig::SigCanTx, 9);
pin!(PB14, Pb14, pb14, GPIOB, Gpiob, PB14_PIN, GpioPin, GPIOB_PERIPH, PB14_OWNED, 14);
   pin_source!(Pb14, super::usart::Usart1, super::sig::SigUsartTx, 4);
   pin_source!(Pb14, super::spi::Spi2, super::sig::SigSpiMiso, 5);
   pin_source!(Pb14, super::usart::Usart3, super::sig::SigUsartRts, 7);
   pin_source!(Pb14, super::usart::Uart4, super::sig::SigUsartRts, 8);
   pin_source!(Pb14, super::tim_gen::Tim12Ch1, super::sig::SigTim, 9);
pin!(PB15, Pb15, pb15, GPIOB, Gpiob, PB15_PIN, GpioPin, GPIOB_PERIPH, PB15_OWNED, 15);
   pin_source!(Pb15, super::usart::Usart1, super::sig::SigUsartRx, 4);
   pin_source!(Pb15, super::spi::Spi2, super::sig::SigSpiMosi, 5);
   pin_source!(Pb15, super::usart::Uart4, super::sig::SigUsartCts, 8);
   pin_source!(Pb15, super::tim_gen::Tim12Ch2, super::sig::SigTim, 9);
pin!(PC0, Pc0, pc0, GPIOC, Gpioc, PC0_PIN, GpioPin, GPIOC_PERIPH, PC0_OWNED, 0);
   pin_source!(Pc0, super::adc::Adc1Ch10, super::sig::SigAdc, 0);
   pin_source!(Pc0, super::adc::Adc2Ch10, super::sig::SigAdc, 0);
   pin_source!(Pc0, super::adc::Adc3Ch10, super::sig::SigAdc, 0);
pin!(PC1, Pc1, pc1, GPIOC, Gpioc, PC1_PIN, GpioPin, GPIOC_PERIPH, PC1_OWNED, 1);
   pin_source!(Pc1, super::adc::Adc1Ch11, super::sig::SigAdc, 0);
   pin_source!(Pc1, super::adc::Adc2Ch11, super::sig::SigAdc, 0);
   pin_source!(Pc1, super::adc::Adc3Ch11, super::sig::SigAdc, 0);
   pin_source!(Pc1, super::spi::Spi2, super::sig::SigSpiMosi, 5);
pin!(PC2, Pc2, pc2, GPIOC, Gpioc, PC2_PIN, GpioPin, GPIOC_PERIPH, PC2_OWNED, 2);
   pin_source!(Pc2, super::adc::Adc1Ch12, super::sig::SigAdc, 0);
   pin_source!(Pc2, super::adc::Adc2Ch12, super::sig::SigAdc, 0);
   pin_source!(Pc2, super::adc::Adc3Ch12, super::sig::SigAdc, 0);
   pin_source!(Pc2, super::spi::Spi2, super::sig::SigSpiMiso, 5);
pin!(PC3, Pc3, pc3, GPIOC, Gpioc, PC3_PIN, GpioPin, GPIOC_PERIPH, PC3_OWNED, 3);
   pin_source!(Pc3, super::adc::Adc1Ch13, super::sig::SigAdc, 0);
   pin_source!(Pc3, super::adc::Adc2Ch13, super::sig::SigAdc, 0);
   pin_source!(Pc3, super::adc::Adc3Ch13, super::sig::SigAdc, 0);
   pin_source!(Pc3, super::spi::Spi2, super::sig::SigSpiMosi, 5);
pin!(PC4, Pc4, pc4, GPIOC, Gpioc, PC4_PIN, GpioPin, GPIOC_PERIPH, PC4_OWNED, 4);
   pin_source!(Pc4, super::adc::Adc1Ch14, super::sig::SigAdc, 0);
   pin_source!(Pc4, super::adc::Adc2Ch14, super::sig::SigAdc, 0);
pin!(PC5, Pc5, pc5, GPIOC, Gpioc, PC5_PIN, GpioPin, GPIOC_PERIPH, PC5_OWNED, 5);
   pin_source!(Pc5, super::adc::Adc1Ch15, super::sig::SigAdc, 0);
   pin_source!(Pc5, super::adc::Adc2Ch15, super::sig::SigAdc, 0);
pin!(PC6, Pc6, pc6, GPIOC, Gpioc, PC6_PIN, GpioPin, GPIOC_PERIPH, PC6_OWNED, 6);
   pin_source!(Pc6, super::tim_gen::Tim3Ch1, super::sig::SigTim, 2);
   pin_source!(Pc6, super::tim_adv::Tim8Ch1, super::sig::SigTim, 3);
   pin_source!(Pc6, super::usart::Usart6, super::sig::SigUsartTx, 8);
   pin_source!(Pc6, super::dcmi::Dcmi, super::sig::SigDcmiD0, 13);
pin!(PC7, Pc7, pc7, GPIOC, Gpioc, PC7_PIN, GpioPin, GPIOC_PERIPH, PC7_OWNED, 7);
   pin_source!(Pc7, super::tim_gen::Tim3Ch2, super::sig::SigTim, 2);
   pin_source!(Pc7, super::tim_adv::Tim8Ch2, super::sig::SigTim, 3);
   pin_source!(Pc7, super::usart::Usart6, super::sig::SigUsartRx, 8);
   pin_source!(Pc7, super::dcmi::Dcmi, super::sig::SigDcmiD1, 13);
pin!(PC8, Pc8, pc8, GPIOC, Gpioc, PC8_PIN, GpioPin, GPIOC_PERIPH, PC8_OWNED, 8);
   pin_source!(Pc8, super::tim_gen::Tim3Ch3, super::sig::SigTim, 2);
   pin_source!(Pc8, super::tim_adv::Tim8Ch3, super::sig::SigTim, 3);
   pin_source!(Pc8, super::usart::Uart5, super::sig::SigUsartRts, 7);
   pin_source!(Pc8, super::usart::Usart6, super::sig::SigUsartCk, 8);
   pin_source!(Pc8, super::dcmi::Dcmi, super::sig::SigDcmiD2, 13);
pin!(PC9, Pc9, pc9, GPIOC, Gpioc, PC9_PIN, GpioPin, GPIOC_PERIPH, PC9_OWNED, 9);
   pin_source!(Pc9, super::tim_gen::Tim3Ch4, super::sig::SigTim, 2);
   pin_source!(Pc9, super::tim_adv::Tim8Ch4, super::sig::SigTim, 3);
   pin_source!(Pc9, super::i2c::I2c3, super::sig::SigI2cSda, 4);
   pin_source!(Pc9, super::usart::Uart5, super::sig::SigUsartCts, 7);
   pin_source!(Pc9, super::dcmi::Dcmi, super::sig::SigDcmiD3, 13);
pin!(PC10, Pc10, pc10, GPIOC, Gpioc, PC10_PIN, GpioPin, GPIOC_PERIPH, PC10_OWNED, 10);
   pin_source!(Pc10, super::spi::Spi3, super::sig::SigSpiSck, 6);
   pin_source!(Pc10, super::usart::Usart3, super::sig::SigUsartTx, 7);
   pin_source!(Pc10, super::usart::Uart4, super::sig::SigUsartTx, 8);
   pin_source!(Pc10, super::dcmi::Dcmi, super::sig::SigDcmiD8, 13);
pin!(PC11, Pc11, pc11, GPIOC, Gpioc, PC11_PIN, GpioPin, GPIOC_PERIPH, PC11_OWNED, 11);
   pin_source!(Pc11, super::spi::Spi3, super::sig::SigSpiMiso, 6);
   pin_source!(Pc11, super::usart::Usart3, super::sig::SigUsartRx, 7);
   pin_source!(Pc11, super::usart::Uart4, super::sig::SigUsartRx, 8);
   pin_source!(Pc11, super::dcmi::Dcmi, super::sig::SigDcmiD4, 13);
pin!(PC12, Pc12, pc12, GPIOC, Gpioc, PC12_PIN, GpioPin, GPIOC_PERIPH, PC12_OWNED, 12);
   pin_source!(Pc12, super::spi::Spi3, super::sig::SigSpiMosi, 6);
   pin_source!(Pc12, super::usart::Usart3, super::sig::SigUsartCk, 7);
   pin_source!(Pc12, super::usart::Uart5, super::sig::SigUsartTx, 8);
   pin_source!(Pc12, super::dcmi::Dcmi, super::sig::SigDcmiD9, 13);
pin!(PC13, Pc13, pc13, GPIOC, Gpioc, PC13_PIN, GpioPin, GPIOC_PERIPH, PC13_OWNED, 13);
pin!(PC14, Pc14, pc14, GPIOC, Gpioc, PC14_PIN, GpioPin, GPIOC_PERIPH, PC14_OWNED, 14);
pin!(PC15, Pc15, pc15, GPIOC, Gpioc, PC15_PIN, GpioPin, GPIOC_PERIPH, PC15_OWNED, 15);
pin!(PD0, Pd0, pd0, GPIOD, Gpiod, PD0_PIN, GpioPin, GPIOD_PERIPH, PD0_OWNED, 0);
   pin_source!(Pd0, super::usart::Uart4, super::sig::SigUsartRx, 8);
   pin_source!(Pd0, super::can::Can1, super::sig::SigCanRx, 9);
pin!(PD1, Pd1, pd1, GPIOD, Gpiod, PD1_PIN, GpioPin, GPIOD_PERIPH, PD1_OWNED, 1);
   pin_source!(Pd1, super::usart::Uart4, super::sig::SigUsartTx, 8);
   pin_source!(Pd1, super::can::Can1, super::sig::SigCanTx, 9);
pin!(PD2, Pd2, pd2, GPIOD, Gpiod, PD2_PIN, GpioPin, GPIOD_PERIPH, PD2_OWNED, 2);
   pin_source!(Pd2, super::usart::Uart5, super::sig::SigUsartRx, 8);
   pin_source!(Pd2, super::dcmi::Dcmi, super::sig::SigDcmiD11, 13);
pin!(PD3, Pd3, pd3, GPIOD, Gpiod, PD3_PIN, GpioPin, GPIOD_PERIPH, PD3_OWNED, 3);
   pin_source!(Pd3, super::spi::Spi2, super::sig::SigSpiSck, 5);
   pin_source!(Pd3, super::usart::Usart2, super::sig::SigUsartCts, 7);
   pin_source!(Pd3, super::dcmi::Dcmi, super::sig::SigDcmiD5, 13);
pin!(PD4, Pd4, pd4, GPIOD, Gpiod, PD4_PIN, GpioPin, GPIOD_PERIPH, PD4_OWNED, 4);
   pin_source!(Pd4, super::usart::Usart2, super::sig::SigUsartRts, 7);
pin!(PD5, Pd5, pd5, GPIOD, Gpiod, PD5_PIN, GpioPin, GPIOD_PERIPH, PD5_OWNED, 5);
   pin_source!(Pd5, super::usart::Usart2, super::sig::SigUsartTx, 7);
pin!(PD6, Pd6, pd6, GPIOD, Gpiod, PD6_PIN, GpioPin, GPIOD_PERIPH, PD6_OWNED, 6);
   pin_source!(Pd6, super::spi::Spi3, super::sig::SigSpiMosi, 5);
   pin_source!(Pd6, super::usart::Usart2, super::sig::SigUsartRx, 7);
   pin_source!(Pd6, super::dcmi::Dcmi, super::sig::SigDcmiD10, 13);
pin!(PD7, Pd7, pd7, GPIOD, Gpiod, PD7_PIN, GpioPin, GPIOD_PERIPH, PD7_OWNED, 7);
   pin_source!(Pd7, super::spi::Spi1, super::sig::SigSpiMosi, 5);
   pin_source!(Pd7, super::usart::Usart2, super::sig::SigUsartCk, 7);
pin!(PD8, Pd8, pd8, GPIOD, Gpiod, PD8_PIN, GpioPin, GPIOD_PERIPH, PD8_OWNED, 8);
   pin_source!(Pd8, super::usart::Usart3, super::sig::SigUsartTx, 7);
pin!(PD9, Pd9, pd9, GPIOD, Gpiod, PD9_PIN, GpioPin, GPIOD_PERIPH, PD9_OWNED, 9);
   pin_source!(Pd9, super::usart::Usart3, super::sig::SigUsartRx, 7);
pin!(PD10, Pd10, pd10, GPIOD, Gpiod, PD10_PIN, GpioPin, GPIOD_PERIPH, PD10_OWNED, 10);
   pin_source!(Pd10, super::usart::Usart3, super::sig::SigUsartCk, 7);
pin!(PD11, Pd11, pd11, GPIOD, Gpiod, PD11_PIN, GpioPin, GPIOD_PERIPH, PD11_OWNED, 11);
   pin_source!(Pd11, super::usart::Usart3, super::sig::SigUsartCts, 7);
pin!(PD12, Pd12, pd12, GPIOD, Gpiod, PD12_PIN, GpioPin, GPIOD_PERIPH, PD12_OWNED, 12);
   pin_source!(Pd12, super::tim_gen::Tim4Ch1, super::sig::SigTim, 2);
   pin_source!(Pd12, super::i2c::I2c4, super::sig::SigI2cScl, 4);
   pin_source!(Pd12, super::usart::Usart3, super::sig::SigUsartRts, 7);
pin!(PD13, Pd13, pd13, GPIOD, Gpiod, PD13_PIN, GpioPin, GPIOD_PERIPH, PD13_OWNED, 13);
   pin_source!(Pd13, super::tim_gen::Tim4Ch2, super::sig::SigTim, 2);
   pin_source!(Pd13, super::i2c::I2c4, super::sig::SigI2cSda, 4);
pin!(PD14, Pd14, pd14, GPIOD, Gpiod, PD14_PIN, GpioPin, GPIOD_PERIPH, PD14_OWNED, 14);
   pin_source!(Pd14, super::tim_gen::Tim4Ch3, super::sig::SigTim, 2);
   pin_source!(Pd14, super::usart::Uart8, super::sig::SigUsartCts, 8);
pin!(PD15, Pd15, pd15, GPIOD, Gpiod, PD15_PIN, GpioPin, GPIOD_PERIPH, PD15_OWNED, 15);
   pin_source!(Pd15, super::tim_gen::Tim4Ch4, super::sig::SigTim, 2);
   pin_source!(Pd15, super::usart::Uart8, super::sig::SigUsartRts, 8);
pin!(PE0, Pe0, pe0, GPIOE, Gpioe, PE0_PIN, GpioPin, GPIOE_PERIPH, PE0_OWNED, 0);
   pin_source!(Pe0, super::usart::Uart8, super::sig::SigUsartRx, 8);
   pin_source!(Pe0, super::dcmi::Dcmi, super::sig::SigDcmiD2, 13);
pin!(PE1, Pe1, pe1, GPIOE, Gpioe, PE1_PIN, GpioPin, GPIOE_PERIPH, PE1_OWNED, 1);
   pin_source!(Pe1, super::usart::Uart8, super::sig::SigUsartTx, 8);
   pin_source!(Pe1, super::dcmi::Dcmi, super::sig::SigDcmiD3, 13);
pin!(PE2, Pe2, pe2, GPIOE, Gpioe, PE2_PIN, GpioPin, GPIOE_PERIPH, PE2_OWNED, 2);
   pin_source!(Pe2, super::spi::Spi4, super::sig::SigSpiSck, 5);
pin!(PE3, Pe3, pe3, GPIOE, Gpioe, PE3_PIN, GpioPin, GPIOE_PERIPH, PE3_OWNED, 3);
pin!(PE4, Pe4, pe4, GPIOE, Gpioe, PE4_PIN, GpioPin, GPIOE_PERIPH, PE4_OWNED, 4);
   pin_source!(Pe4, super::spi::Spi4, super::sig::SigSpiNss, 5);
   pin_source!(Pe4, super::dcmi::Dcmi, super::sig::SigDcmiD4, 13);
pin!(PE5, Pe5, pe5, GPIOE, Gpioe, PE5_PIN, GpioPin, GPIOE_PERIPH, PE5_OWNED, 5);
   pin_source!(Pe5, super::tim_gen::Tim9Ch1, super::sig::SigTim, 3);
   pin_source!(Pe5, super::spi::Spi4, super::sig::SigSpiMiso, 5);
   pin_source!(Pe5, super::dcmi::Dcmi, super::sig::SigDcmiD6, 13);
pin!(PE6, Pe6, pe6, GPIOE, Gpioe, PE6_PIN, GpioPin, GPIOE_PERIPH, PE6_OWNED, 6);
   pin_source!(Pe6, super::tim_gen::Tim9Ch2, super::sig::SigTim, 3);
   pin_source!(Pe6, super::spi::Spi4, super::sig::SigSpiMosi, 5);
   pin_source!(Pe6, super::dcmi::Dcmi, super::sig::SigDcmiD7, 13);
pin!(PE7, Pe7, pe7, GPIOE, Gpioe, PE7_PIN, GpioPin, GPIOE_PERIPH, PE7_OWNED, 7);
   pin_source!(Pe7, super::usart::Uart7, super::sig::SigUsartRx, 8);
pin!(PE8, Pe8, pe8, GPIOE, Gpioe, PE8_PIN, GpioPin, GPIOE_PERIPH, PE8_OWNED, 8);
   pin_source!(Pe8, super::usart::Uart7, super::sig::SigUsartTx, 8);
pin!(PE9, Pe9, pe9, GPIOE, Gpioe, PE9_PIN, GpioPin, GPIOE_PERIPH, PE9_OWNED, 9);
   pin_source!(Pe9, super::tim_adv::Tim1Ch1, super::sig::SigTim, 1);
   pin_source!(Pe9, super::usart::Uart7, super::sig::SigUsartRts, 8);
pin!(PE10, Pe10, pe10, GPIOE, Gpioe, PE10_PIN, GpioPin, GPIOE_PERIPH, PE10_OWNED, 10);
   pin_source!(Pe10, super::usart::Uart7, super::sig::SigUsartCts, 8);
pin!(PE11, Pe11, pe11, GPIOE, Gpioe, PE11_PIN, GpioPin, GPIOE_PERIPH, PE11_OWNED, 11);
   pin_source!(Pe11, super::tim_adv::Tim1Ch2, super::sig::SigTim, 1);
   pin_source!(Pe11, super::spi::Spi4, super::sig::SigSpiNss, 5);
pin!(PE12, Pe12, pe12, GPIOE, Gpioe, PE12_PIN, GpioPin, GPIOE_PERIPH, PE12_OWNED, 12);
   pin_source!(Pe12, super::spi::Spi4, super::sig::SigSpiSck, 5);
pin!(PE13, Pe13, pe13, GPIOE, Gpioe, PE13_PIN, GpioPin, GPIOE_PERIPH, PE13_OWNED, 13);
   pin_source!(Pe13, super::tim_adv::Tim1Ch3, super::sig::SigTim, 1);
   pin_source!(Pe13, super::spi::Spi4, super::sig::SigSpiMiso, 5);
pin!(PE14, Pe14, pe14, GPIOE, Gpioe, PE14_PIN, GpioPin, GPIOE_PERIPH, PE14_OWNED, 14);
   pin_source!(Pe14, super::tim_adv::Tim1Ch4, super::sig::SigTim, 1);
   pin_source!(Pe14, super::spi::Spi4, super::sig::SigSpiMosi, 5);
pin!(PE15, Pe15, pe15, GPIOE, Gpioe, PE15_PIN, GpioPin, GPIOE_PERIPH, PE15_OWNED, 15);
pin!(PF0, Pf0, pf0, GPIOF, Gpiof, PF0_PIN, GpioPin, GPIOF_PERIPH, PF0_OWNED, 0);
   pin_source!(Pf0, super::i2c::I2c2, super::sig::SigI2cSda, 4);
pin!(PF1, Pf1, pf1, GPIOF, Gpiof, PF1_PIN, GpioPin, GPIOF_PERIPH, PF1_OWNED, 1);
   pin_source!(Pf1, super::i2c::I2c2, super::sig::SigI2cScl, 4);
pin!(PF2, Pf2, pf2, GPIOF, Gpiof, PF2_PIN, GpioPin, GPIOF_PERIPH, PF2_OWNED, 2);
pin!(PF3, Pf3, pf3, GPIOF, Gpiof, PF3_PIN, GpioPin, GPIOF_PERIPH, PF3_OWNED, 3);
   pin_source!(Pf3, super::adc::Adc3Ch9, super::sig::SigAdc, 0);
pin!(PF4, Pf4, pf4, GPIOF, Gpiof, PF4_PIN, GpioPin, GPIOF_PERIPH, PF4_OWNED, 4);
   pin_source!(Pf4, super::adc::Adc3Ch14, super::sig::SigAdc, 0);
pin!(PF5, Pf5, pf5, GPIOF, Gpiof, PF5_PIN, GpioPin, GPIOF_PERIPH, PF5_OWNED, 5);
   pin_source!(Pf5, super::adc::Adc3Ch15, super::sig::SigAdc, 0);
pin!(PF6, Pf6, pf6, GPIOF, Gpiof, PF6_PIN, GpioPin, GPIOF_PERIPH, PF6_OWNED, 6);
   pin_source!(Pf6, super::adc::Adc3Ch4, super::sig::SigAdc, 0);
   pin_source!(Pf6, super::tim_gen::Tim10Ch1, super::sig::SigTim, 3);
   pin_source!(Pf6, super::spi::Spi5, super::sig::SigSpiNss, 5);
   pin_source!(Pf6, super::usart::Uart7, super::sig::SigUsartRx, 8);
pin!(PF7, Pf7, pf7, GPIOF, Gpiof, PF7_PIN, GpioPin, GPIOF_PERIPH, PF7_OWNED, 7);
   pin_source!(Pf7, super::adc::Adc3Ch5, super::sig::SigAdc, 0);
   pin_source!(Pf7, super::tim_gen::Tim11Ch1, super::sig::SigTim, 3);
   pin_source!(Pf7, super::spi::Spi5, super::sig::SigSpiSck, 5);
   pin_source!(Pf7, super::usart::Uart7, super::sig::SigUsartTx, 8);
pin!(PF8, Pf8, pf8, GPIOF, Gpiof, PF8_PIN, GpioPin, GPIOF_PERIPH, PF8_OWNED, 8);
   pin_source!(Pf8, super::adc::Adc3Ch6, super::sig::SigAdc, 0);
   pin_source!(Pf8, super::spi::Spi5, super::sig::SigSpiMiso, 5);
   pin_source!(Pf8, super::usart::Uart7, super::sig::SigUsartRts, 8);
   pin_source!(Pf8, super::tim_gen::Tim13Ch1, super::sig::SigTim, 9);
pin!(PF9, Pf9, pf9, GPIOF, Gpiof, PF9_PIN, GpioPin, GPIOF_PERIPH, PF9_OWNED, 9);
   pin_source!(Pf9, super::adc::Adc3Ch7, super::sig::SigAdc, 0);
   pin_source!(Pf9, super::spi::Spi5, super::sig::SigSpiMosi, 5);
   pin_source!(Pf9, super::usart::Uart7, super::sig::SigUsartCts, 8);
   pin_source!(Pf9, super::tim_gen::Tim14Ch1, super::sig::SigTim, 9);
pin!(PF10, Pf10, pf10, GPIOF, Gpiof, PF10_PIN, GpioPin, GPIOF_PERIPH, PF10_OWNED, 10);
   pin_source!(Pf10, super::adc::Adc3Ch8, super::sig::SigAdc, 0);
   pin_source!(Pf10, super::dcmi::Dcmi, super::sig::SigDcmiD11, 13);
pin!(PF11, Pf11, pf11, GPIOF, Gpiof, PF11_PIN, GpioPin, GPIOF_PERIPH, PF11_OWNED, 11);
   pin_source!(Pf11, super::spi::Spi5, super::sig::SigSpiMosi, 5);
   pin_source!(Pf11, super::dcmi::Dcmi, super::sig::SigDcmiD12, 13);
pin!(PF12, Pf12, pf12, GPIOF, Gpiof, PF12_PIN, GpioPin, GPIOF_PERIPH, PF12_OWNED, 12);
pin!(PF13, Pf13, pf13, GPIOF, Gpiof, PF13_PIN, GpioPin, GPIOF_PERIPH, PF13_OWNED, 13);
pin!(PF14, Pf14, pf14, GPIOF, Gpiof, PF14_PIN, GpioPin, GPIOF_PERIPH, PF14_OWNED, 14);
   pin_source!(Pf14, super::i2c::I2c4, super::sig::SigI2cScl, 4);
pin!(PF15, Pf15, pf15, GPIOF, Gpiof, PF15_PIN, GpioPin, GPIOF_PERIPH, PF15_OWNED, 15);
   pin_source!(Pf15, super::i2c::I2c4, super::sig::SigI2cSda, 4);
pin!(PG0, Pg0, pg0, GPIOG, Gpiog, PG0_PIN, GpioPin, GPIOG_PERIPH, PG0_OWNED, 0);
pin!(PG1, Pg1, pg1, GPIOG, Gpiog, PG1_PIN, GpioPin, GPIOG_PERIPH, PG1_OWNED, 1);
pin!(PG2, Pg2, pg2, GPIOG, Gpiog, PG2_PIN, GpioPin, GPIOG_PERIPH, PG2_OWNED, 2);
pin!(PG3, Pg3, pg3, GPIOG, Gpiog, PG3_PIN, GpioPin, GPIOG_PERIPH, PG3_OWNED, 3);
pin!(PG4, Pg4, pg4, GPIOG, Gpiog, PG4_PIN, GpioPin, GPIOG_PERIPH, PG4_OWNED, 4);
pin!(PG5, Pg5, pg5, GPIOG, Gpiog, PG5_PIN, GpioPin, GPIOG_PERIPH, PG5_OWNED, 5);
pin!(PG6, Pg6, pg6, GPIOG, Gpiog, PG6_PIN, GpioPin, GPIOG_PERIPH, PG6_OWNED, 6);
   pin_source!(Pg6, super::dcmi::Dcmi, super::sig::SigDcmiD12, 13);
pin!(PG7, Pg7, pg7, GPIOG, Gpiog, PG7_PIN, GpioPin, GPIOG_PERIPH, PG7_OWNED, 7);
   pin_source!(Pg7, super::usart::Usart6, super::sig::SigUsartCk, 8);
   pin_source!(Pg7, super::dcmi::Dcmi, super::sig::SigDcmiD13, 13);
pin!(PG8, Pg8, pg8, GPIOG, Gpiog, PG8_PIN, GpioPin, GPIOG_PERIPH, PG8_OWNED, 8);
   pin_source!(Pg8, super::spi::Spi6, super::sig::SigSpiNss, 5);
   pin_source!(Pg8, super::usart::Usart6, super::sig::SigUsartRts, 8);
pin!(PG9, Pg9, pg9, GPIOG, Gpiog, PG9_PIN, GpioPin, GPIOG_PERIPH, PG9_OWNED, 9);
   pin_source!(Pg9, super::spi::Spi1, super::sig::SigSpiMiso, 5);
   pin_source!(Pg9, super::usart::Usart6, super::sig::SigUsartRx, 8);
   pin_source!(Pg9, super::dcmi::Dcmi, super::sig::SigDcmiVsync, 13);
pin!(PG10, Pg10, pg10, GPIOG, Gpiog, PG10_PIN, GpioPin, GPIOG_PERIPH, PG10_OWNED, 10);
   pin_source!(Pg10, super::spi::Spi1, super::sig::SigSpiNss, 5);
   pin_source!(Pg10, super::dcmi::Dcmi, super::sig::SigDcmiD2, 13);
pin!(PG11, Pg11, pg11, GPIOG, Gpiog, PG11_PIN, GpioPin, GPIOG_PERIPH, PG11_OWNED, 11);
   pin_source!(Pg11, super::spi::Spi1, super::sig::SigSpiSck, 5);
   pin_source!(Pg11, super::dcmi::Dcmi, super::sig::SigDcmiD3, 13);
pin!(PG12, Pg12, pg12, GPIOG, Gpiog, PG12_PIN, GpioPin, GPIOG_PERIPH, PG12_OWNED, 12);
   pin_source!(Pg12, super::spi::Spi6, super::sig::SigSpiMiso, 5);
   pin_source!(Pg12, super::usart::Usart6, super::sig::SigUsartRts, 8);
pin!(PG13, Pg13, pg13, GPIOG, Gpiog, PG13_PIN, GpioPin, GPIOG_PERIPH, PG13_OWNED, 13);
   pin_source!(Pg13, super::spi::Spi6, super::sig::SigSpiSck, 5);
   pin_source!(Pg13, super::usart::Usart6, super::sig::SigUsartCts, 8);
pin!(PG14, Pg14, pg14, GPIOG, Gpiog, PG14_PIN, GpioPin, GPIOG_PERIPH, PG14_OWNED, 14);
   pin_source!(Pg14, super::spi::Spi6, super::sig::SigSpiMosi, 5);
   pin_source!(Pg14, super::usart::Usart6, super::sig::SigUsartTx, 8);
pin!(PG15, Pg15, pg15, GPIOG, Gpiog, PG15_PIN, GpioPin, GPIOG_PERIPH, PG15_OWNED, 15);
   pin_source!(Pg15, super::usart::Usart6, super::sig::SigUsartCts, 8);
   pin_source!(Pg15, super::dcmi::Dcmi, super::sig::SigDcmiD13, 13);
pin!(PH0, Ph0, ph0, GPIOH, Gpioh, PH0_PIN, GpioPin, GPIOH_PERIPH, PH0_OWNED, 0);
pin!(PH1, Ph1, ph1, GPIOH, Gpioh, PH1_PIN, GpioPin, GPIOH_PERIPH, PH1_OWNED, 1);
pin!(PH2, Ph2, ph2, GPIOH, Gpioh, PH2_PIN, GpioPin, GPIOH_PERIPH, PH2_OWNED, 2);
pin!(PH3, Ph3, ph3, GPIOH, Gpioh, PH3_PIN, GpioPin, GPIOH_PERIPH, PH3_OWNED, 3);
pin!(PH4, Ph4, ph4, GPIOH, Gpioh, PH4_PIN, GpioPin, GPIOH_PERIPH, PH4_OWNED, 4);
   pin_source!(Ph4, super::i2c::I2c2, super::sig::SigI2cScl, 4);
pin!(PH5, Ph5, ph5, GPIOH, Gpioh, PH5_PIN, GpioPin, GPIOH_PERIPH, PH5_OWNED, 5);
   pin_source!(Ph5, super::i2c::I2c2, super::sig::SigI2cSda, 4);
   pin_source!(Ph5, super::spi::Spi5, super::sig::SigSpiNss, 5);
pin!(PH6, Ph6, ph6, GPIOH, Gpioh, PH6_PIN, GpioPin, GPIOH_PERIPH, PH6_OWNED, 6);
   pin_source!(Ph6, super::spi::Spi5, super::sig::SigSpiSck, 5);
   pin_source!(Ph6, super::tim_gen::Tim12Ch1, super::sig::SigTim, 9);
   pin_source!(Ph6, super::dcmi::Dcmi, super::sig::SigDcmiD8, 13);
pin!(PH7, Ph7, ph7, GPIOH, Gpioh, PH7_PIN, GpioPin, GPIOH_PERIPH, PH7_OWNED, 7);
   pin_source!(Ph7, super::i2c::I2c3, super::sig::SigI2cScl, 4);
   pin_source!(Ph7, super::spi::Spi5, super::sig::SigSpiMiso, 5);
   pin_source!(Ph7, super::dcmi::Dcmi, super::sig::SigDcmiD9, 13);
pin!(PH8, Ph8, ph8, GPIOH, Gpioh, PH8_PIN, GpioPin, GPIOH_PERIPH, PH8_OWNED, 8);
   pin_source!(Ph8, super::i2c::I2c3, super::sig::SigI2cSda, 4);
   pin_source!(Ph8, super::dcmi::Dcmi, super::sig::SigDcmiHsync, 13);
pin!(PH9, Ph9, ph9, GPIOH, Gpioh, PH9_PIN, GpioPin, GPIOH_PERIPH, PH9_OWNED, 9);
   pin_source!(Ph9, super::tim_gen::Tim12Ch2, super::sig::SigTim, 9);
   pin_source!(Ph9, super::dcmi::Dcmi, super::sig::SigDcmiD0, 13);
pin!(PH10, Ph10, ph10, GPIOH, Gpioh, PH10_PIN, GpioPin, GPIOH_PERIPH, PH10_OWNED, 10);
   pin_source!(Ph10, super::tim_gen::Tim5Ch1, super::sig::SigTim, 2);
   pin_source!(Ph10, super::dcmi::Dcmi, super::sig::SigDcmiD1, 13);
pin!(PH11, Ph11, ph11, GPIOH, Gpioh, PH11_PIN, GpioPin, GPIOH_PERIPH, PH11_OWNED, 11);
   pin_source!(Ph11, super::tim_gen::Tim5Ch2, super::sig::SigTim, 2);
   pin_source!(Ph11, super::i2c::I2c4, super::sig::SigI2cScl, 4);
   pin_source!(Ph11, super::dcmi::Dcmi, super::sig::SigDcmiD2, 13);
pin!(PH12, Ph12, ph12, GPIOH, Gpioh, PH12_PIN, GpioPin, GPIOH_PERIPH, PH12_OWNED, 12);
   pin_source!(Ph12, super::tim_gen::Tim5Ch3, super::sig::SigTim, 2);
   pin_source!(Ph12, super::i2c::I2c4, super::sig::SigI2cSda, 4);
   pin_source!(Ph12, super::dcmi::Dcmi, super::sig::SigDcmiD3, 13);
pin!(PH13, Ph13, ph13, GPIOH, Gpioh, PH13_PIN, GpioPin, GPIOH_PERIPH, PH13_OWNED, 13);
   pin_source!(Ph13, super::usart::Uart4, super::sig::SigUsartTx, 8);
   pin_source!(Ph13, super::can::Can1, super::sig::SigCanTx, 9);
pin!(PH14, Ph14, ph14, GPIOH, Gpioh, PH14_PIN, GpioPin, GPIOH_PERIPH, PH14_OWNED, 14);
   pin_source!(Ph14, super::usart::Uart4, super::sig::SigUsartRx, 8);
   pin_source!(Ph14, super::can::Can1, super::sig::SigCanRx, 9);
   pin_source!(Ph14, super::dcmi::Dcmi, super::sig::SigDcmiD4, 13);
pin!(PH15, Ph15, ph15, GPIOH, Gpioh, PH15_PIN, GpioPin, GPIOH_PERIPH, PH15_OWNED, 15);
   pin_source!(Ph15, super::dcmi::Dcmi, super::sig::SigDcmiD11, 13);
pin!(PI0, Pi0, pi0, GPIOI, Gpioi, PI0_PIN, GpioPin, GPIOI_PERIPH, PI0_OWNED, 0);
   pin_source!(Pi0, super::tim_gen::Tim5Ch4, super::sig::SigTim, 2);
   pin_source!(Pi0, super::spi::Spi2, super::sig::SigSpiNss, 5);
   pin_source!(Pi0, super::dcmi::Dcmi, super::sig::SigDcmiD13, 13);
pin!(PI1, Pi1, pi1, GPIOI, Gpioi, PI1_PIN, GpioPin, GPIOI_PERIPH, PI1_OWNED, 1);
   pin_source!(Pi1, super::spi::Spi2, super::sig::SigSpiSck, 5);
   pin_source!(Pi1, super::dcmi::Dcmi, super::sig::SigDcmiD8, 13);
pin!(PI2, Pi2, pi2, GPIOI, Gpioi, PI2_PIN, GpioPin, GPIOI_PERIPH, PI2_OWNED, 2);
   pin_source!(Pi2, super::tim_adv::Tim8Ch4, super::sig::SigTim, 3);
   pin_source!(Pi2, super::spi::Spi2, super::sig::SigSpiMiso, 5);
   pin_source!(Pi2, super::dcmi::Dcmi, super::sig::SigDcmiD9, 13);
pin!(PI3, Pi3, pi3, GPIOI, Gpioi, PI3_PIN, GpioPin, GPIOI_PERIPH, PI3_OWNED, 3);
   pin_source!(Pi3, super::spi::Spi2, super::sig::SigSpiMosi, 5);
   pin_source!(Pi3, super::dcmi::Dcmi, super::sig::SigDcmiD10, 13);
pin!(PI4, Pi4, pi4, GPIOI, Gpioi, PI4_PIN, GpioPin, GPIOI_PERIPH, PI4_OWNED, 4);
   pin_source!(Pi4, super::dcmi::Dcmi, super::sig::SigDcmiD5, 13);
pin!(PI5, Pi5, pi5, GPIOI, Gpioi, PI5_PIN, GpioPin, GPIOI_PERIPH, PI5_OWNED, 5);
   pin_source!(Pi5, super::tim_adv::Tim8Ch1, super::sig::SigTim, 3);
   pin_source!(Pi5, super::dcmi::Dcmi, super::sig::SigDcmiVsync, 13);
pin!(PI6, Pi6, pi6, GPIOI, Gpioi, PI6_PIN, GpioPin, GPIOI_PERIPH, PI6_OWNED, 6);
   pin_source!(Pi6, super::tim_adv::Tim8Ch2, super::sig::SigTim, 3);
   pin_source!(Pi6, super::dcmi::Dcmi, super::sig::SigDcmiD6, 13);
pin!(PI7, Pi7, pi7, GPIOI, Gpioi, PI7_PIN, GpioPin, GPIOI_PERIPH, PI7_OWNED, 7);
   pin_source!(Pi7, super::tim_adv::Tim8Ch3, super::sig::SigTim, 3);
   pin_source!(Pi7, super::dcmi::Dcmi, super::sig::SigDcmiD7, 13);
pin!(PI8, Pi8, pi8, GPIOI, Gpioi, PI8_PIN, GpioPin, GPIOI_PERIPH, PI8_OWNED, 8);
pin!(PI9, Pi9, pi9, GPIOI, Gpioi, PI9_PIN, GpioPin, GPIOI_PERIPH, PI9_OWNED, 9);
   pin_source!(Pi9, super::usart::Uart4, super::sig::SigUsartRx, 8);
   pin_source!(Pi9, super::can::Can1, super::sig::SigCanRx, 9);
pin!(PI10, Pi10, pi10, GPIOI, Gpioi, PI10_PIN, GpioPin, GPIOI_PERIPH, PI10_OWNED, 10);
pin!(PI11, Pi11, pi11, GPIOI, Gpioi, PI11_PIN, GpioPin, GPIOI_PERIPH, PI11_OWNED, 11);
pin!(PI12, Pi12, pi12, GPIOI, Gpioi, PI12_PIN, GpioPin, GPIOI_PERIPH, PI12_OWNED, 12);
pin!(PI13, Pi13, pi13, GPIOI, Gpioi, PI13_PIN, GpioPin, GPIOI_PERIPH, PI13_OWNED, 13);
pin!(PI14, Pi14, pi14, GPIOI, Gpioi, PI14_PIN, GpioPin, GPIOI_PERIPH, PI14_OWNED, 14);
pin!(PI15, Pi15, pi15, GPIOI, Gpioi, PI15_PIN, GpioPin, GPIOI_PERIPH, PI15_OWNED, 15);
pin!(PJ0, Pj0, pj0, GPIOJ, Gpioj, PJ0_PIN, GpioPin, GPIOJ_PERIPH, PJ0_OWNED, 0);
pin!(PJ1, Pj1, pj1, GPIOJ, Gpioj, PJ1_PIN, GpioPin, GPIOJ_PERIPH, PJ1_OWNED, 1);
pin!(PJ2, Pj2, pj2, GPIOJ, Gpioj, PJ2_PIN, GpioPin, GPIOJ_PERIPH, PJ2_OWNED, 2);
pin!(PJ3, Pj3, pj3, GPIOJ, Gpioj, PJ3_PIN, GpioPin, GPIOJ_PERIPH, PJ3_OWNED, 3);
pin!(PJ4, Pj4, pj4, GPIOJ, Gpioj, PJ4_PIN, GpioPin, GPIOJ_PERIPH, PJ4_OWNED, 4);
pin!(PJ5, Pj5, pj5, GPIOJ, Gpioj, PJ5_PIN, GpioPin, GPIOJ_PERIPH, PJ5_OWNED, 5);
pin!(PJ6, Pj6, pj6, GPIOJ, Gpioj, PJ6_PIN, GpioPin, GPIOJ_PERIPH, PJ6_OWNED, 6);
pin!(PJ7, Pj7, pj7, GPIOJ, Gpioj, PJ7_PIN, GpioPin, GPIOJ_PERIPH, PJ7_OWNED, 7);
pin!(PJ8, Pj8, pj8, GPIOJ, Gpioj, PJ8_PIN, GpioPin, GPIOJ_PERIPH, PJ8_OWNED, 8);
pin!(PJ9, Pj9, pj9, GPIOJ, Gpioj, PJ9_PIN, GpioPin, GPIOJ_PERIPH, PJ9_OWNED, 9);
pin!(PJ10, Pj10, pj10, GPIOJ, Gpioj, PJ10_PIN, GpioPin, GPIOJ_PERIPH, PJ10_OWNED, 10);
pin!(PJ11, Pj11, pj11, GPIOJ, Gpioj, PJ11_PIN, GpioPin, GPIOJ_PERIPH, PJ11_OWNED, 11);
pin!(PJ12, Pj12, pj12, GPIOJ, Gpioj, PJ12_PIN, GpioPin, GPIOJ_PERIPH, PJ12_OWNED, 12);
pin!(PJ13, Pj13, pj13, GPIOJ, Gpioj, PJ13_PIN, GpioPin, GPIOJ_PERIPH, PJ13_OWNED, 13);
pin!(PJ14, Pj14, pj14, GPIOJ, Gpioj, PJ14_PIN, GpioPin, GPIOJ_PERIPH, PJ14_OWNED, 14);
pin!(PJ15, Pj15, pj15, GPIOJ, Gpioj, PJ15_PIN, GpioPin, GPIOJ_PERIPH, PJ15_OWNED, 15);
pin!(PK0, Pk0, pk0, GPIOK, Gpiok, PK0_PIN, GpioPin, GPIOK_PERIPH, PK0_OWNED, 0);
pin!(PK1, Pk1, pk1, GPIOK, Gpiok, PK1_PIN, GpioPin, GPIOK_PERIPH, PK1_OWNED, 1);
pin!(PK2, Pk2, pk2, GPIOK, Gpiok, PK2_PIN, GpioPin, GPIOK_PERIPH, PK2_OWNED, 2);
pin!(PK3, Pk3, pk3, GPIOK, Gpiok, PK3_PIN, GpioPin, GPIOK_PERIPH, PK3_OWNED, 3);
pin!(PK4, Pk4, pk4, GPIOK, Gpiok, PK4_PIN, GpioPin, GPIOK_PERIPH, PK4_OWNED, 4);
pin!(PK5, Pk5, pk5, GPIOK, Gpiok, PK5_PIN, GpioPin, GPIOK_PERIPH, PK5_OWNED, 5);
pin!(PK6, Pk6, pk6, GPIOK, Gpiok, PK6_PIN, GpioPin, GPIOK_PERIPH, PK6_OWNED, 6);
pin!(PK7, Pk7, pk7, GPIOK, Gpiok, PK7_PIN, GpioPin, GPIOK_PERIPH, PK7_OWNED, 7);
