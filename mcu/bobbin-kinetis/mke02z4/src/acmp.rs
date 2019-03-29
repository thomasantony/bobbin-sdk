::bobbin_mcu::periph!( ACMP0, Acmp0, ACMP0_PERIPH, AcmpPeriph, ACMP0_OWNED, ACMP0_REF_COUNT, 0x40073000, 0x00, 0x12);
::bobbin_mcu::periph!( ACMP1, Acmp1, ACMP1_PERIPH, AcmpPeriph, ACMP1_OWNED, ACMP1_REF_COUNT, 0x40074000, 0x01, 0x13);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ACMP Peripheral"]
pub struct AcmpPeriph(pub usize); 

impl AcmpPeriph {
    #[doc="Get the CS Register."]
    #[inline] pub fn cs_reg(&self) -> ::bobbin_mcu::register::Register<Cs> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cs, 0x0)
    }

    #[doc="Get the *mut pointer for the CS register."]
    #[inline] pub fn cs_mut(&self) -> *mut Cs { 
        self.cs_reg().ptr()
    }

    #[doc="Get the *const pointer for the CS register."]
    #[inline] pub fn cs_ptr(&self) -> *const Cs { 
        self.cs_reg().ptr()
    }

    #[doc="Read the CS register."]
    #[inline] pub fn cs(&self) -> Cs { 
        self.cs_reg().read()
    }

    #[doc="Write the CS register."]
    #[inline] pub fn write_cs(&self, value: Cs) -> &Self { 
        self.cs_reg().write(value);
        self
    }

    #[doc="Set the CS register."]
    #[inline] pub fn set_cs<F: FnOnce(Cs) -> Cs>(&self, f: F) -> &Self {
        self.cs_reg().set(f);
        self
    }

    #[doc="Modify the CS register."]
    #[inline] pub fn with_cs<F: FnOnce(Cs) -> Cs>(&self, f: F) -> &Self {
        self.cs_reg().with(f);
        self
    }

    #[doc="Get the C0 Register."]
    #[inline] pub fn c0_reg(&self) -> ::bobbin_mcu::register::Register<C0> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C0, 0x1)
    }

    #[doc="Get the *mut pointer for the C0 register."]
    #[inline] pub fn c0_mut(&self) -> *mut C0 { 
        self.c0_reg().ptr()
    }

    #[doc="Get the *const pointer for the C0 register."]
    #[inline] pub fn c0_ptr(&self) -> *const C0 { 
        self.c0_reg().ptr()
    }

    #[doc="Read the C0 register."]
    #[inline] pub fn c0(&self) -> C0 { 
        self.c0_reg().read()
    }

    #[doc="Write the C0 register."]
    #[inline] pub fn write_c0(&self, value: C0) -> &Self { 
        self.c0_reg().write(value);
        self
    }

    #[doc="Set the C0 register."]
    #[inline] pub fn set_c0<F: FnOnce(C0) -> C0>(&self, f: F) -> &Self {
        self.c0_reg().set(f);
        self
    }

    #[doc="Modify the C0 register."]
    #[inline] pub fn with_c0<F: FnOnce(C0) -> C0>(&self, f: F) -> &Self {
        self.c0_reg().with(f);
        self
    }

    #[doc="Get the C1 Register."]
    #[inline] pub fn c1_reg(&self) -> ::bobbin_mcu::register::Register<C1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1, 0x2)
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
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2, 0x3)
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

}

#[doc="ACMP Control and Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cs(pub u8);
impl Cs {
    #[doc="ACMP MOD"]
    #[inline] pub fn acmod(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if ACMOD != 0"]
    #[inline] pub fn test_acmod(&self) -> bool {
        self.acmod() != 0
    }

    #[doc="Sets the ACMOD field."]
    #[inline] pub fn set_acmod<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ACMP Output Pin Enable"]
    #[inline] pub fn acope(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ACOPE != 0"]
    #[inline] pub fn test_acope(&self) -> bool {
        self.acope() != 0
    }

    #[doc="Sets the ACOPE field."]
    #[inline] pub fn set_acope<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="ACMP Output"]
    #[inline] pub fn aco(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ACO != 0"]
    #[inline] pub fn test_aco(&self) -> bool {
        self.aco() != 0
    }

    #[doc="Sets the ACO field."]
    #[inline] pub fn set_aco<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ACMP Interrupt Enable"]
    #[inline] pub fn acie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ACIE != 0"]
    #[inline] pub fn test_acie(&self) -> bool {
        self.acie() != 0
    }

    #[doc="Sets the ACIE field."]
    #[inline] pub fn set_acie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="ACMP Interrupt Flag Bit"]
    #[inline] pub fn acf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACF != 0"]
    #[inline] pub fn test_acf(&self) -> bool {
        self.acf() != 0
    }

    #[doc="Sets the ACF field."]
    #[inline] pub fn set_acf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Analog Comparator Hysterisis Selection"]
    #[inline] pub fn hyst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if HYST != 0"]
    #[inline] pub fn test_hyst(&self) -> bool {
        self.hyst() != 0
    }

    #[doc="Sets the HYST field."]
    #[inline] pub fn set_hyst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Analog Comparator Enable"]
    #[inline] pub fn ace(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ACE != 0"]
    #[inline] pub fn test_ace(&self) -> bool {
        self.ace() != 0
    }

    #[doc="Sets the ACE field."]
    #[inline] pub fn set_ace<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Cs {
    #[inline]
    fn from(other: u8) -> Self {
         Cs(other)
    }
}

impl ::core::fmt::Display for Cs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.acmod() != 0 { try!(write!(f, " acmod=0x{:x}", self.acmod()))}
        if self.acope() != 0 { try!(write!(f, " acope"))}
        if self.aco() != 0 { try!(write!(f, " aco"))}
        if self.acie() != 0 { try!(write!(f, " acie"))}
        if self.acf() != 0 { try!(write!(f, " acf"))}
        if self.hyst() != 0 { try!(write!(f, " hyst"))}
        if self.ace() != 0 { try!(write!(f, " ace"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ACMP Control Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C0(pub u8);
impl C0 {
    #[doc="ACMP Negative Input Select"]
    #[inline] pub fn acnsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if ACNSEL != 0"]
    #[inline] pub fn test_acnsel(&self) -> bool {
        self.acnsel() != 0
    }

    #[doc="Sets the ACNSEL field."]
    #[inline] pub fn set_acnsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="ACMP Positive Input Select"]
    #[inline] pub fn acpsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if ACPSEL != 0"]
    #[inline] pub fn test_acpsel(&self) -> bool {
        self.acpsel() != 0
    }

    #[doc="Sets the ACPSEL field."]
    #[inline] pub fn set_acpsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u8> for C0 {
    #[inline]
    fn from(other: u8) -> Self {
         C0(other)
    }
}

impl ::core::fmt::Display for C0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.acnsel() != 0 { try!(write!(f, " acnsel=0x{:x}", self.acnsel()))}
        if self.acpsel() != 0 { try!(write!(f, " acpsel=0x{:x}", self.acpsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ACMP Control Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1(pub u8);
impl C1 {
    #[doc="DAC Output Level Selection"]
    #[inline] pub fn dacval(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if DACVAL != 0"]
    #[inline] pub fn test_dacval(&self) -> bool {
        self.dacval() != 0
    }

    #[doc="Sets the DACVAL field."]
    #[inline] pub fn set_dacval<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DAC Reference Select"]
    #[inline] pub fn dacref(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if DACREF != 0"]
    #[inline] pub fn test_dacref(&self) -> bool {
        self.dacref() != 0
    }

    #[doc="Sets the DACREF field."]
    #[inline] pub fn set_dacref<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DAC Enable"]
    #[inline] pub fn dacen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DACEN != 0"]
    #[inline] pub fn test_dacen(&self) -> bool {
        self.dacen() != 0
    }

    #[doc="Sets the DACEN field."]
    #[inline] pub fn set_dacen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
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
        if self.dacval() != 0 { try!(write!(f, " dacval=0x{:x}", self.dacval()))}
        if self.dacref() != 0 { try!(write!(f, " dacref"))}
        if self.dacen() != 0 { try!(write!(f, " dacen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="ACMP Control Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2(pub u8);
impl C2 {
    #[doc="ACMP Input Pin Enable"]
    #[inline] pub fn acipe(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if ACIPE != 0"]
    #[inline] pub fn test_acipe(&self) -> bool {
        self.acipe() != 0
    }

    #[doc="Sets the ACIPE field."]
    #[inline] pub fn set_acipe<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
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
        if self.acipe() != 0 { try!(write!(f, " acipe=0x{:x}", self.acipe()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

