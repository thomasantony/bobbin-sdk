::bobbin_mcu::periph!( ICS, Ics, ICS_PERIPH, IcsPeriph, ICS_OWNED, ICS_REF_COUNT, 0x40064000, 0x00, 0x0c);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ICS Peripheral"]
pub struct IcsPeriph(pub usize); 

impl IcsPeriph {
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

    #[doc="Get the C3 Register."]
    #[inline] pub fn c3_reg(&self) -> ::bobbin_mcu::register::Register<C3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C3, 0x2)
    }

    #[doc="Get the *mut pointer for the C3 register."]
    #[inline] pub fn c3_mut(&self) -> *mut C3 { 
        self.c3_reg().ptr()
    }

    #[doc="Get the *const pointer for the C3 register."]
    #[inline] pub fn c3_ptr(&self) -> *const C3 { 
        self.c3_reg().ptr()
    }

    #[doc="Read the C3 register."]
    #[inline] pub fn c3(&self) -> C3 { 
        self.c3_reg().read()
    }

    #[doc="Write the C3 register."]
    #[inline] pub fn write_c3(&self, value: C3) -> &Self { 
        self.c3_reg().write(value);
        self
    }

    #[doc="Set the C3 register."]
    #[inline] pub fn set_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
        self.c3_reg().set(f);
        self
    }

    #[doc="Modify the C3 register."]
    #[inline] pub fn with_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
        self.c3_reg().with(f);
        self
    }

    #[doc="Get the C4 Register."]
    #[inline] pub fn c4_reg(&self) -> ::bobbin_mcu::register::Register<C4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C4, 0x3)
    }

    #[doc="Get the *mut pointer for the C4 register."]
    #[inline] pub fn c4_mut(&self) -> *mut C4 { 
        self.c4_reg().ptr()
    }

    #[doc="Get the *const pointer for the C4 register."]
    #[inline] pub fn c4_ptr(&self) -> *const C4 { 
        self.c4_reg().ptr()
    }

    #[doc="Read the C4 register."]
    #[inline] pub fn c4(&self) -> C4 { 
        self.c4_reg().read()
    }

    #[doc="Write the C4 register."]
    #[inline] pub fn write_c4(&self, value: C4) -> &Self { 
        self.c4_reg().write(value);
        self
    }

    #[doc="Set the C4 register."]
    #[inline] pub fn set_c4<F: FnOnce(C4) -> C4>(&self, f: F) -> &Self {
        self.c4_reg().set(f);
        self
    }

    #[doc="Modify the C4 register."]
    #[inline] pub fn with_c4<F: FnOnce(C4) -> C4>(&self, f: F) -> &Self {
        self.c4_reg().with(f);
        self
    }

    #[doc="Get the S Register."]
    #[inline] pub fn s_reg(&self) -> ::bobbin_mcu::register::Register<S> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut S, 0x4)
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

}

#[doc="ICS Control Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1(pub u8);
impl C1 {
    #[doc="Internal Reference Stop Enable"]
    #[inline] pub fn irefsten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IREFSTEN != 0"]
    #[inline] pub fn test_irefsten(&self) -> bool {
        self.irefsten() != 0
    }

    #[doc="Sets the IREFSTEN field."]
    #[inline] pub fn set_irefsten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Internal Reference Clock Enable"]
    #[inline] pub fn irclken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IRCLKEN != 0"]
    #[inline] pub fn test_irclken(&self) -> bool {
        self.irclken() != 0
    }

    #[doc="Sets the IRCLKEN field."]
    #[inline] pub fn set_irclken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Internal Reference Select"]
    #[inline] pub fn irefs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IREFS != 0"]
    #[inline] pub fn test_irefs(&self) -> bool {
        self.irefs() != 0
    }

    #[doc="Sets the IREFS field."]
    #[inline] pub fn set_irefs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Reference Divider"]
    #[inline] pub fn rdiv(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if RDIV != 0"]
    #[inline] pub fn test_rdiv(&self) -> bool {
        self.rdiv() != 0
    }

    #[doc="Sets the RDIV field."]
    #[inline] pub fn set_rdiv<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clock Source Select"]
    #[inline] pub fn clks(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if CLKS != 0"]
    #[inline] pub fn test_clks(&self) -> bool {
        self.clks() != 0
    }

    #[doc="Sets the CLKS field."]
    #[inline] pub fn set_clks<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
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
        if self.irefsten() != 0 { try!(write!(f, " irefsten"))}
        if self.irclken() != 0 { try!(write!(f, " irclken"))}
        if self.irefs() != 0 { try!(write!(f, " irefs"))}
        if self.rdiv() != 0 { try!(write!(f, " rdiv=0x{:x}", self.rdiv()))}
        if self.clks() != 0 { try!(write!(f, " clks=0x{:x}", self.clks()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ICS Control Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2(pub u8);
impl C2 {
    #[doc="Low Power Select"]
    #[inline] pub fn lp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LP != 0"]
    #[inline] pub fn test_lp(&self) -> bool {
        self.lp() != 0
    }

    #[doc="Sets the LP field."]
    #[inline] pub fn set_lp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Bus Frequency Divider"]
    #[inline] pub fn bdiv(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x7) as u8) } // [7:5]
    }

    #[doc="Returns true if BDIV != 0"]
    #[inline] pub fn test_bdiv(&self) -> bool {
        self.bdiv() != 0
    }

    #[doc="Sets the BDIV field."]
    #[inline] pub fn set_bdiv<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 5);
        self.0 |= value << 5;
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
        if self.lp() != 0 { try!(write!(f, " lp"))}
        if self.bdiv() != 0 { try!(write!(f, " bdiv=0x{:x}", self.bdiv()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ICS Control Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C3(pub u8);
impl C3 {
    #[doc="Slow Internal Reference Clock Trim Setting"]
    #[inline] pub fn sctrim(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCTRIM != 0"]
    #[inline] pub fn test_sctrim(&self) -> bool {
        self.sctrim() != 0
    }

    #[doc="Sets the SCTRIM field."]
    #[inline] pub fn set_sctrim<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for C3 {
    #[inline]
    fn from(other: u8) -> Self {
         C3(other)
    }
}

impl ::core::fmt::Display for C3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sctrim() != 0 { try!(write!(f, " sctrim=0x{:x}", self.sctrim()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ICS Control Register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C4(pub u8);
impl C4 {
    #[doc="Slow Internal Reference Clock Fine Trim"]
    #[inline] pub fn scftrim(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SCFTRIM != 0"]
    #[inline] pub fn test_scftrim(&self) -> bool {
        self.scftrim() != 0
    }

    #[doc="Sets the SCFTRIM field."]
    #[inline] pub fn set_scftrim<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Monitor Enable"]
    #[inline] pub fn cme(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CME != 0"]
    #[inline] pub fn test_cme(&self) -> bool {
        self.cme() != 0
    }

    #[doc="Sets the CME field."]
    #[inline] pub fn set_cme<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Loss of Lock Interrupt"]
    #[inline] pub fn lolie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LOLIE != 0"]
    #[inline] pub fn test_lolie(&self) -> bool {
        self.lolie() != 0
    }

    #[doc="Sets the LOLIE field."]
    #[inline] pub fn set_lolie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for C4 {
    #[inline]
    fn from(other: u8) -> Self {
         C4(other)
    }
}

impl ::core::fmt::Display for C4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.scftrim() != 0 { try!(write!(f, " scftrim"))}
        if self.cme() != 0 { try!(write!(f, " cme"))}
        if self.lolie() != 0 { try!(write!(f, " lolie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ICS Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct S(pub u8);
impl S {
    #[doc="Clock Mode Status"]
    #[inline] pub fn clkst(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if CLKST != 0"]
    #[inline] pub fn test_clkst(&self) -> bool {
        self.clkst() != 0
    }

    #[doc="Sets the CLKST field."]
    #[inline] pub fn set_clkst<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Internal Reference Status"]
    #[inline] pub fn irefst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IREFST != 0"]
    #[inline] pub fn test_irefst(&self) -> bool {
        self.irefst() != 0
    }

    #[doc="Sets the IREFST field."]
    #[inline] pub fn set_irefst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Lock Status"]
    #[inline] pub fn lock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LOCK != 0"]
    #[inline] pub fn test_lock(&self) -> bool {
        self.lock() != 0
    }

    #[doc="Sets the LOCK field."]
    #[inline] pub fn set_lock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Loss of Lock Status"]
    #[inline] pub fn lols(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LOLS != 0"]
    #[inline] pub fn test_lols(&self) -> bool {
        self.lols() != 0
    }

    #[doc="Sets the LOLS field."]
    #[inline] pub fn set_lols<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.clkst() != 0 { try!(write!(f, " clkst=0x{:x}", self.clkst()))}
        if self.irefst() != 0 { try!(write!(f, " irefst"))}
        if self.lock() != 0 { try!(write!(f, " lock"))}
        if self.lols() != 0 { try!(write!(f, " lols"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

