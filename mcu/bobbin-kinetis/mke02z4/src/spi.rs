::bobbin_mcu::periph!( SPI0, Spi0, SPI0_PERIPH, SpiPeriph, SPI0_OWNED, SPI0_REF_COUNT, 0x40076000, 0x00, 0x14);
::bobbin_mcu::periph!( SPI1, Spi1, SPI1_PERIPH, SpiPeriph, SPI1_OWNED, SPI1_REF_COUNT, 0x40077000, 0x01, 0x15);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SPI Peripheral"]
pub struct SpiPeriph(pub usize); 

impl SpiPeriph {
    #[doc="Get the C1 Register."]
    #[inline] pub fn c1_reg(&self) -> ::bobbin_mcu::register::Register<C1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1, 0x0)
    }

    #[doc="Get the *mut pointer for the C1 register."]
    #[inline] pub fn c1_mut(&self) -> *mut C1 { 
        self.c1_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1 register."]
    #[inline] pub fn c1_ptr(&self) -> *const C1 { 
        self.c1_reg().ptr()
    }

    #[doc="Read the C1 register."]
    #[inline] pub fn c1(&self) -> C1 { 
        self.c1_reg().read()
    }

    #[doc="Write the C1 register."]
    #[inline] pub fn write_c1(&self, value: C1) -> &Self { 
        self.c1_reg().write(value);
        self
    }

    #[doc="Set the C1 register."]
    #[inline] pub fn set_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
        self.c1_reg().set(f);
        self
    }

    #[doc="Modify the C1 register."]
    #[inline] pub fn with_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
        self.c1_reg().with(f);
        self
    }

    #[doc="Get the C2 Register."]
    #[inline] pub fn c2_reg(&self) -> ::bobbin_mcu::register::Register<C2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2, 0x1)
    }

    #[doc="Get the *mut pointer for the C2 register."]
    #[inline] pub fn c2_mut(&self) -> *mut C2 { 
        self.c2_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2 register."]
    #[inline] pub fn c2_ptr(&self) -> *const C2 { 
        self.c2_reg().ptr()
    }

    #[doc="Read the C2 register."]
    #[inline] pub fn c2(&self) -> C2 { 
        self.c2_reg().read()
    }

    #[doc="Write the C2 register."]
    #[inline] pub fn write_c2(&self, value: C2) -> &Self { 
        self.c2_reg().write(value);
        self
    }

    #[doc="Set the C2 register."]
    #[inline] pub fn set_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
        self.c2_reg().set(f);
        self
    }

    #[doc="Modify the C2 register."]
    #[inline] pub fn with_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
        self.c2_reg().with(f);
        self
    }

    #[doc="Get the BR Register."]
    #[inline] pub fn br_reg(&self) -> ::bobbin_mcu::register::Register<Br> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Br, 0x2)
    }

    #[doc="Get the *mut pointer for the BR register."]
    #[inline] pub fn br_mut(&self) -> *mut Br { 
        self.br_reg().ptr()
    }

    #[doc="Get the *const pointer for the BR register."]
    #[inline] pub fn br_ptr(&self) -> *const Br { 
        self.br_reg().ptr()
    }

    #[doc="Read the BR register."]
    #[inline] pub fn br(&self) -> Br { 
        self.br_reg().read()
    }

    #[doc="Write the BR register."]
    #[inline] pub fn write_br(&self, value: Br) -> &Self { 
        self.br_reg().write(value);
        self
    }

    #[doc="Set the BR register."]
    #[inline] pub fn set_br<F: FnOnce(Br) -> Br>(&self, f: F) -> &Self {
        self.br_reg().set(f);
        self
    }

    #[doc="Modify the BR register."]
    #[inline] pub fn with_br<F: FnOnce(Br) -> Br>(&self, f: F) -> &Self {
        self.br_reg().with(f);
        self
    }

    #[doc="Get the S Register."]
    #[inline] pub fn s_reg(&self) -> ::bobbin_mcu::register::Register<S> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut S, 0x3)
    }

    #[doc="Get the *mut pointer for the S register."]
    #[inline] pub fn s_mut(&self) -> *mut S { 
        self.s_reg().ptr()
    }

    #[doc="Get the *const pointer for the S register."]
    #[inline] pub fn s_ptr(&self) -> *const S { 
        self.s_reg().ptr()
    }

    #[doc="Read the S register."]
    #[inline] pub fn s(&self) -> S { 
        self.s_reg().read()
    }

    #[doc="Write the S register."]
    #[inline] pub fn write_s(&self, value: S) -> &Self { 
        self.s_reg().write(value);
        self
    }

    #[doc="Set the S register."]
    #[inline] pub fn set_s<F: FnOnce(S) -> S>(&self, f: F) -> &Self {
        self.s_reg().set(f);
        self
    }

    #[doc="Modify the S register."]
    #[inline] pub fn with_s<F: FnOnce(S) -> S>(&self, f: F) -> &Self {
        self.s_reg().with(f);
        self
    }

    #[doc="Get the D Register."]
    #[inline] pub fn d_reg(&self) -> ::bobbin_mcu::register::Register<D> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut D, 0x5)
    }

    #[doc="Get the *mut pointer for the D register."]
    #[inline] pub fn d_mut(&self) -> *mut D { 
        self.d_reg().ptr()
    }

    #[doc="Get the *const pointer for the D register."]
    #[inline] pub fn d_ptr(&self) -> *const D { 
        self.d_reg().ptr()
    }

    #[doc="Read the D register."]
    #[inline] pub fn d(&self) -> D { 
        self.d_reg().read()
    }

    #[doc="Write the D register."]
    #[inline] pub fn write_d(&self, value: D) -> &Self { 
        self.d_reg().write(value);
        self
    }

    #[doc="Set the D register."]
    #[inline] pub fn set_d<F: FnOnce(D) -> D>(&self, f: F) -> &Self {
        self.d_reg().set(f);
        self
    }

    #[doc="Modify the D register."]
    #[inline] pub fn with_d<F: FnOnce(D) -> D>(&self, f: F) -> &Self {
        self.d_reg().with(f);
        self
    }

    #[doc="Get the M Register."]
    #[inline] pub fn m_reg(&self) -> ::bobbin_mcu::register::Register<M> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut M, 0x7)
    }

    #[doc="Get the *mut pointer for the M register."]
    #[inline] pub fn m_mut(&self) -> *mut M { 
        self.m_reg().ptr()
    }

    #[doc="Get the *const pointer for the M register."]
    #[inline] pub fn m_ptr(&self) -> *const M { 
        self.m_reg().ptr()
    }

    #[doc="Read the M register."]
    #[inline] pub fn m(&self) -> M { 
        self.m_reg().read()
    }

    #[doc="Write the M register."]
    #[inline] pub fn write_m(&self, value: M) -> &Self { 
        self.m_reg().write(value);
        self
    }

    #[doc="Set the M register."]
    #[inline] pub fn set_m<F: FnOnce(M) -> M>(&self, f: F) -> &Self {
        self.m_reg().set(f);
        self
    }

    #[doc="Modify the M register."]
    #[inline] pub fn with_m<F: FnOnce(M) -> M>(&self, f: F) -> &Self {
        self.m_reg().with(f);
        self
    }

}

#[doc="SPI Control Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1(pub u8);
impl C1 {
    #[doc="LSB First (shifter direction)"]
    #[inline] pub fn lsbfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LSBFE != 0"]
    #[inline] pub fn test_lsbfe(&self) -> bool {
        self.lsbfe() != 0
    }

    #[doc="Sets the LSBFE field."]
    #[inline] pub fn set_lsbfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Slave Select Output Enable"]
    #[inline] pub fn ssoe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SSOE != 0"]
    #[inline] pub fn test_ssoe(&self) -> bool {
        self.ssoe() != 0
    }

    #[doc="Sets the SSOE field."]
    #[inline] pub fn set_ssoe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clock Phase"]
    #[inline] pub fn cpha(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CPHA != 0"]
    #[inline] pub fn test_cpha(&self) -> bool {
        self.cpha() != 0
    }

    #[doc="Sets the CPHA field."]
    #[inline] pub fn set_cpha<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Clock Polarity"]
    #[inline] pub fn cpol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CPOL != 0"]
    #[inline] pub fn test_cpol(&self) -> bool {
        self.cpol() != 0
    }

    #[doc="Sets the CPOL field."]
    #[inline] pub fn set_cpol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Master/Slave Mode Select"]
    #[inline] pub fn mstr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MSTR != 0"]
    #[inline] pub fn test_mstr(&self) -> bool {
        self.mstr() != 0
    }

    #[doc="Sets the MSTR field."]
    #[inline] pub fn set_mstr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SPI Transmit Interrupt Enable"]
    #[inline] pub fn sptie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SPTIE != 0"]
    #[inline] pub fn test_sptie(&self) -> bool {
        self.sptie() != 0
    }

    #[doc="Sets the SPTIE field."]
    #[inline] pub fn set_sptie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SPI System Enable"]
    #[inline] pub fn spe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SPE != 0"]
    #[inline] pub fn test_spe(&self) -> bool {
        self.spe() != 0
    }

    #[doc="Sets the SPE field."]
    #[inline] pub fn set_spe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SPI Interrupt Enable: for SPRF and MODF"]
    #[inline] pub fn spie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SPIE != 0"]
    #[inline] pub fn test_spie(&self) -> bool {
        self.spie() != 0
    }

    #[doc="Sets the SPIE field."]
    #[inline] pub fn set_spie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for C1 {
    #[inline]
    fn from(other: u8) -> Self {
         C1(other)
    }
}

impl ::core::fmt::Display for C1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lsbfe() != 0 { try!(write!(f, " lsbfe"))}
        if self.ssoe() != 0 { try!(write!(f, " ssoe"))}
        if self.cpha() != 0 { try!(write!(f, " cpha"))}
        if self.cpol() != 0 { try!(write!(f, " cpol"))}
        if self.mstr() != 0 { try!(write!(f, " mstr"))}
        if self.sptie() != 0 { try!(write!(f, " sptie"))}
        if self.spe() != 0 { try!(write!(f, " spe"))}
        if self.spie() != 0 { try!(write!(f, " spie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Control Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2(pub u8);
impl C2 {
    #[doc="SPI Pin Control 0"]
    #[inline] pub fn spc0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SPC0 != 0"]
    #[inline] pub fn test_spc0(&self) -> bool {
        self.spc0() != 0
    }

    #[doc="Sets the SPC0 field."]
    #[inline] pub fn set_spc0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SPI Stop in Wait Mode"]
    #[inline] pub fn spiswai(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SPISWAI != 0"]
    #[inline] pub fn test_spiswai(&self) -> bool {
        self.spiswai() != 0
    }

    #[doc="Sets the SPISWAI field."]
    #[inline] pub fn set_spiswai<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Bidirectional Mode Output Enable"]
    #[inline] pub fn bidiroe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BIDIROE != 0"]
    #[inline] pub fn test_bidiroe(&self) -> bool {
        self.bidiroe() != 0
    }

    #[doc="Sets the BIDIROE field."]
    #[inline] pub fn set_bidiroe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Master Mode-Fault Function Enable"]
    #[inline] pub fn modfen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MODFEN != 0"]
    #[inline] pub fn test_modfen(&self) -> bool {
        self.modfen() != 0
    }

    #[doc="Sets the MODFEN field."]
    #[inline] pub fn set_modfen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SPI Match Interrupt Enable"]
    #[inline] pub fn spmie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SPMIE != 0"]
    #[inline] pub fn test_spmie(&self) -> bool {
        self.spmie() != 0
    }

    #[doc="Sets the SPMIE field."]
    #[inline] pub fn set_spmie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for C2 {
    #[inline]
    fn from(other: u8) -> Self {
         C2(other)
    }
}

impl ::core::fmt::Display for C2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.spc0() != 0 { try!(write!(f, " spc0"))}
        if self.spiswai() != 0 { try!(write!(f, " spiswai"))}
        if self.bidiroe() != 0 { try!(write!(f, " bidiroe"))}
        if self.modfen() != 0 { try!(write!(f, " modfen"))}
        if self.spmie() != 0 { try!(write!(f, " spmie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Baud Rate Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Br(pub u8);
impl Br {
    #[doc="SPI Baud Rate Divisor"]
    #[inline] pub fn spr(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if SPR != 0"]
    #[inline] pub fn test_spr(&self) -> bool {
        self.spr() != 0
    }

    #[doc="Sets the SPR field."]
    #[inline] pub fn set_spr<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SPI Baud Rate Prescale Divisor"]
    #[inline] pub fn sppr(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if SPPR != 0"]
    #[inline] pub fn test_sppr(&self) -> bool {
        self.sppr() != 0
    }

    #[doc="Sets the SPPR field."]
    #[inline] pub fn set_sppr<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for Br {
    #[inline]
    fn from(other: u8) -> Self {
         Br(other)
    }
}

impl ::core::fmt::Display for Br {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Br {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.spr() != 0 { try!(write!(f, " spr=0x{:x}", self.spr()))}
        if self.sppr() != 0 { try!(write!(f, " sppr=0x{:x}", self.sppr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct S(pub u8);
impl S {
    #[doc="Master Mode Fault Flag"]
    #[inline] pub fn modf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MODF != 0"]
    #[inline] pub fn test_modf(&self) -> bool {
        self.modf() != 0
    }

    #[doc="Sets the MODF field."]
    #[inline] pub fn set_modf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SPI Transmit Buffer Empty Flag"]
    #[inline] pub fn sptef(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SPTEF != 0"]
    #[inline] pub fn test_sptef(&self) -> bool {
        self.sptef() != 0
    }

    #[doc="Sets the SPTEF field."]
    #[inline] pub fn set_sptef<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SPI Match Flag"]
    #[inline] pub fn spmf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SPMF != 0"]
    #[inline] pub fn test_spmf(&self) -> bool {
        self.spmf() != 0
    }

    #[doc="Sets the SPMF field."]
    #[inline] pub fn set_spmf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SPI Read Buffer Full Flag"]
    #[inline] pub fn sprf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SPRF != 0"]
    #[inline] pub fn test_sprf(&self) -> bool {
        self.sprf() != 0
    }

    #[doc="Sets the SPRF field."]
    #[inline] pub fn set_sprf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for S {
    #[inline]
    fn from(other: u8) -> Self {
         S(other)
    }
}

impl ::core::fmt::Display for S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for S {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.modf() != 0 { try!(write!(f, " modf"))}
        if self.sptef() != 0 { try!(write!(f, " sptef"))}
        if self.spmf() != 0 { try!(write!(f, " spmf"))}
        if self.sprf() != 0 { try!(write!(f, " sprf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct D(pub u8);
impl D {
    #[doc="Data (low byte)"]
    #[inline] pub fn bits(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BITS != 0"]
    #[inline] pub fn test_bits(&self) -> bool {
        self.bits() != 0
    }

    #[doc="Sets the BITS field."]
    #[inline] pub fn set_bits<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for D {
    #[inline]
    fn from(other: u8) -> Self {
         D(other)
    }
}

impl ::core::fmt::Display for D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bits() != 0 { try!(write!(f, " bits=0x{:x}", self.bits()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SPI Match Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct M(pub u8);
impl M {
    #[doc="Hardware compare value (low byte)"]
    #[inline] pub fn bits(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if BITS != 0"]
    #[inline] pub fn test_bits(&self) -> bool {
        self.bits() != 0
    }

    #[doc="Sets the BITS field."]
    #[inline] pub fn set_bits<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for M {
    #[inline]
    fn from(other: u8) -> Self {
         M(other)
    }
}

impl ::core::fmt::Display for M {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for M {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bits() != 0 { try!(write!(f, " bits=0x{:x}", self.bits()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

