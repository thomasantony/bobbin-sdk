#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::lpspi::*;

periph!( LPSPI0, Lpspi0, _LPSPI0, LpspiPeriph, 0x4002c000);
periph!( LPSPI1, Lpspi1, _LPSPI1, LpspiPeriph, 0x4002d000);
periph!( LPSPI2, Lpspi2, _LPSPI2, LpspiPeriph, 0x4002e000);

impl super::sig::Signal<super::sig::Lpspi0Sck> for Lpspi0 {}
impl super::sig::SignalSpiSck<super::sig::Lpspi0Sck> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Sout> for Lpspi0 {}
impl super::sig::SignalSpiSout<super::sig::Lpspi0Sout> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Sin> for Lpspi0 {}
impl super::sig::SignalSpiSin<super::sig::Lpspi0Sin> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs0> for Lpspi0 {}
impl super::sig::SignalSpiPcs0<super::sig::Lpspi0Pcs0> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs1> for Lpspi0 {}
impl super::sig::SignalSpiPcs1<super::sig::Lpspi0Pcs1> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs2> for Lpspi0 {}
impl super::sig::SignalSpiPcs2<super::sig::Lpspi0Pcs2> for Lpspi0 {}
impl super::sig::Signal<super::sig::Lpspi0Pcs3> for Lpspi0 {}
impl super::sig::SignalSpiPcs3<super::sig::Lpspi0Pcs3> for Lpspi0 {}

impl super::sig::Signal<super::sig::Lpspi1Sck> for Lpspi1 {}
impl super::sig::SignalSpiSck<super::sig::Lpspi1Sck> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Sout> for Lpspi1 {}
impl super::sig::SignalSpiSout<super::sig::Lpspi1Sout> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Sin> for Lpspi1 {}
impl super::sig::SignalSpiSin<super::sig::Lpspi1Sin> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs0> for Lpspi1 {}
impl super::sig::SignalSpiPcs0<super::sig::Lpspi1Pcs0> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs1> for Lpspi1 {}
impl super::sig::SignalSpiPcs1<super::sig::Lpspi1Pcs1> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs2> for Lpspi1 {}
impl super::sig::SignalSpiPcs2<super::sig::Lpspi1Pcs2> for Lpspi1 {}
impl super::sig::Signal<super::sig::Lpspi1Pcs3> for Lpspi1 {}
impl super::sig::SignalSpiPcs3<super::sig::Lpspi1Pcs3> for Lpspi1 {}

impl super::sig::Signal<super::sig::Lpspi2Sck> for Lpspi2 {}
impl super::sig::SignalSpiSck<super::sig::Lpspi2Sck> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Sout> for Lpspi2 {}
impl super::sig::SignalSpiSout<super::sig::Lpspi2Sout> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Sin> for Lpspi2 {}
impl super::sig::SignalSpiSin<super::sig::Lpspi2Sin> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs0> for Lpspi2 {}
impl super::sig::SignalSpiPcs0<super::sig::Lpspi2Pcs0> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs1> for Lpspi2 {}
impl super::sig::SignalSpiPcs1<super::sig::Lpspi2Pcs1> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs2> for Lpspi2 {}
impl super::sig::SignalSpiPcs2<super::sig::Lpspi2Pcs2> for Lpspi2 {}
impl super::sig::Signal<super::sig::Lpspi2Pcs3> for Lpspi2 {}
impl super::sig::SignalSpiPcs3<super::sig::Lpspi2Pcs3> for Lpspi2 {}



pub trait IrqLpspi<T> {
    fn irq_lpspi(&self) -> T;
}

impl IrqLpspi<super::irq::IrqLpspi0> for Lpspi0 {
    fn irq_lpspi(&self) -> super::irq::IrqLpspi0 { super::irq::IRQ_LPSPI0 }
}

impl IrqLpspi<super::irq::IrqLpspi1> for Lpspi1 {
    fn irq_lpspi(&self) -> super::irq::IrqLpspi1 { super::irq::IRQ_LPSPI1 }
}

impl IrqLpspi<super::irq::IrqLpspi2> for Lpspi2 {
    fn irq_lpspi(&self) -> super::irq::IrqLpspi2 { super::irq::IRQ_LPSPI2 }
}
