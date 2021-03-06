pub use ::stm32_common::spi_v2::*;

::bobbin_mcu::periph!( SPI1, Spi1, SPI1_PERIPH, SpiPeriph, SPI1_OWNED, SPI1_REF_COUNT, 0x40013000, 0x00, 0x2b);
::bobbin_mcu::periph!( SPI2, Spi2, SPI2_PERIPH, SpiPeriph, SPI2_OWNED, SPI2_REF_COUNT, 0x40003800, 0x01, 0x2c);
::bobbin_mcu::periph!( SPI3, Spi3, SPI3_PERIPH, SpiPeriph, SPI3_OWNED, SPI3_REF_COUNT, 0x40003c00, 0x02, 0x2d);

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB2RSTR"), field: Some("SPI1RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Spi1 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2rstr().spi1rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2rstr(|r| r.set_spi1rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB2ENR"), field: Some("SPI1EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Spi1 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2enr().spi1en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2enr(|r| r.set_spi1en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB2SMENR"), field: Some("SPI1SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Spi1 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb2smenr().spi1smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb2smenr(|r| r.set_spi1smen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR1"), field: Some("SPI2RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Spi2 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr1().spi2rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr1(|r| r.set_spi2rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR1"), field: Some("SPI2EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Spi2 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr1().spi2en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr1(|r| r.set_spi2en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR1"), field: Some("SPI2SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Spi2 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1smenr1().spi2smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr1(|r| r.set_spi2smen(value));
        self
    }
}

// Gate { name: None, gate_type: Some("RST"), periph: Some("RCC"), register: Some("APB1RSTR1"), field: Some("SPI3RST"), description: None }
impl ::bobbin_mcu::gate::GateRst for Spi3 {
    #[inline]
    fn gate_rst(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1rstr1().spi3rst() }
    #[inline]
    fn set_gate_rst<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1rstr1(|r| r.set_spi3rst(value));
        self
    }
}

// Gate { name: None, gate_type: Some("EN"), periph: Some("RCC"), register: Some("APB1ENR1"), field: Some("SPI3EN"), description: None }
impl ::bobbin_mcu::gate::GateEn for Spi3 {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1enr1().spi3en() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1enr1(|r| r.set_spi3en(value));
        self
    }
}

// Gate { name: None, gate_type: Some("SLEEP_EN"), periph: Some("RCC"), register: Some("APB1SMENR1"), field: Some("SPI3SMEN"), description: None }
impl ::bobbin_mcu::gate::GateSleepEn for Spi3 {
    #[inline]
    fn gate_sleep_en(&self) -> ::bobbin_bits::U1 { ::rcc::RCC.apb1smenr1().spi3smen() }
    #[inline]
    fn set_gate_sleep_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::rcc::RCC.with_apb1smenr1(|r| r.set_spi3smen(value));
        self
    }
}

