pub use kinetis_common::spi::*;

::bobbin_mcu::periph!( SPI0, Spi0, SPI0_PERIPH, SpiPeriph, SPI0_OWNED, SPI0_REF_COUNT, 0x4002c000, 0x00, 0x0f);
::bobbin_mcu::periph!( SPI1, Spi1, SPI1_PERIPH, SpiPeriph, SPI1_OWNED, SPI1_REF_COUNT, 0x4002d000, 0x01, 0x10);
::bobbin_mcu::periph!( SPI2, Spi2, SPI2_PERIPH, SpiPeriph, SPI2_OWNED, SPI2_REF_COUNT, 0x400ac000, 0x02, 0x11);

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("SPI0"), description: None }
impl ::bobbin_mcu::gate::GateEn for Spi0 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc6().spi0() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_spi0(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC6"), field: Some("SPI1"), description: None }
impl ::bobbin_mcu::gate::GateEn for Spi1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc6().spi1() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc6(|r| r.set_spi1(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("SIM"), register: Some("SCGC3"), field: Some("SPI2"), description: None }
impl ::bobbin_mcu::gate::GateEn for Spi2 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::sim::SIM.scgc3().spi2() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::sim::SIM.with_scgc3(|r| r.set_spi2(value));
        self
    }
}

