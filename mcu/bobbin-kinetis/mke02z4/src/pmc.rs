::bobbin_mcu::periph!( PMC, Pmc, PMC_PERIPH, PmcPeriph, PMC_OWNED, PMC_REF_COUNT, 0x4007d000, 0x00, 0x18);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PMC Peripheral"]
pub struct PmcPeriph(pub usize); 

impl PmcPeriph {
    #[doc="Get the SPMSC1 Register."]
    #[inline] pub fn spmsc1_reg(&self) -> ::bobbin_mcu::register::Register<Spmsc1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Spmsc1, 0x0)
    }

    #[doc="Get the *mut pointer for the SPMSC1 register."]
    #[inline] pub fn spmsc1_mut(&self) -> *mut Spmsc1 { 
        self.spmsc1_reg().ptr()
    }

    #[doc="Get the *const pointer for the SPMSC1 register."]
    #[inline] pub fn spmsc1_ptr(&self) -> *const Spmsc1 { 
        self.spmsc1_reg().ptr()
    }

    #[doc="Read the SPMSC1 register."]
    #[inline] pub fn spmsc1(&self) -> Spmsc1 { 
        self.spmsc1_reg().read()
    }

    #[doc="Write the SPMSC1 register."]
    #[inline] pub fn write_spmsc1(&self, value: Spmsc1) -> &Self { 
        self.spmsc1_reg().write(value);
        self
    }

    #[doc="Set the SPMSC1 register."]
    #[inline] pub fn set_spmsc1<F: FnOnce(Spmsc1) -> Spmsc1>(&self, f: F) -> &Self {
        self.spmsc1_reg().set(f);
        self
    }

    #[doc="Modify the SPMSC1 register."]
    #[inline] pub fn with_spmsc1<F: FnOnce(Spmsc1) -> Spmsc1>(&self, f: F) -> &Self {
        self.spmsc1_reg().with(f);
        self
    }

    #[doc="Get the SPMSC2 Register."]
    #[inline] pub fn spmsc2_reg(&self) -> ::bobbin_mcu::register::Register<Spmsc2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Spmsc2, 0x1)
    }

    #[doc="Get the *mut pointer for the SPMSC2 register."]
    #[inline] pub fn spmsc2_mut(&self) -> *mut Spmsc2 { 
        self.spmsc2_reg().ptr()
    }

    #[doc="Get the *const pointer for the SPMSC2 register."]
    #[inline] pub fn spmsc2_ptr(&self) -> *const Spmsc2 { 
        self.spmsc2_reg().ptr()
    }

    #[doc="Read the SPMSC2 register."]
    #[inline] pub fn spmsc2(&self) -> Spmsc2 { 
        self.spmsc2_reg().read()
    }

    #[doc="Write the SPMSC2 register."]
    #[inline] pub fn write_spmsc2(&self, value: Spmsc2) -> &Self { 
        self.spmsc2_reg().write(value);
        self
    }

    #[doc="Set the SPMSC2 register."]
    #[inline] pub fn set_spmsc2<F: FnOnce(Spmsc2) -> Spmsc2>(&self, f: F) -> &Self {
        self.spmsc2_reg().set(f);
        self
    }

    #[doc="Modify the SPMSC2 register."]
    #[inline] pub fn with_spmsc2<F: FnOnce(Spmsc2) -> Spmsc2>(&self, f: F) -> &Self {
        self.spmsc2_reg().with(f);
        self
    }

}

#[doc="System Power Management Status and Control 1 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Spmsc1(pub u8);
impl Spmsc1 {
    #[doc="Bandgap Buffer Enable"]
    #[inline] pub fn bgbe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BGBE != 0"]
    #[inline] pub fn test_bgbe(&self) -> bool {
        self.bgbe() != 0
    }

    #[doc="Sets the BGBE field."]
    #[inline] pub fn set_bgbe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Low-Voltage Detect Enable"]
    #[inline] pub fn lvde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LVDE != 0"]
    #[inline] pub fn test_lvde(&self) -> bool {
        self.lvde() != 0
    }

    #[doc="Sets the LVDE field."]
    #[inline] pub fn set_lvde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Low-Voltage Detect Stop Enable"]
    #[inline] pub fn lvdse(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if LVDSE != 0"]
    #[inline] pub fn test_lvdse(&self) -> bool {
        self.lvdse() != 0
    }

    #[doc="Sets the LVDSE field."]
    #[inline] pub fn set_lvdse<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Low-Voltage Detect Reset Enable"]
    #[inline] pub fn lvdre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if LVDRE != 0"]
    #[inline] pub fn test_lvdre(&self) -> bool {
        self.lvdre() != 0
    }

    #[doc="Sets the LVDRE field."]
    #[inline] pub fn set_lvdre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Low-Voltage Warning Interrupt Enable"]
    #[inline] pub fn lvwie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LVWIE != 0"]
    #[inline] pub fn test_lvwie(&self) -> bool {
        self.lvwie() != 0
    }

    #[doc="Sets the LVWIE field."]
    #[inline] pub fn set_lvwie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Low-Voltage Warning Acknowledge"]
    #[inline] pub fn lvwack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LVWACK != 0"]
    #[inline] pub fn test_lvwack(&self) -> bool {
        self.lvwack() != 0
    }

    #[doc="Sets the LVWACK field."]
    #[inline] pub fn set_lvwack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Low-Voltage Warning Flag"]
    #[inline] pub fn lvwf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LVWF != 0"]
    #[inline] pub fn test_lvwf(&self) -> bool {
        self.lvwf() != 0
    }

    #[doc="Sets the LVWF field."]
    #[inline] pub fn set_lvwf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Spmsc1 {
    #[inline]
    fn from(other: u8) -> Self {
         Spmsc1(other)
    }
}

impl ::core::fmt::Display for Spmsc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Spmsc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.bgbe() != 0 { try!(write!(f, " bgbe"))}
        if self.lvde() != 0 { try!(write!(f, " lvde"))}
        if self.lvdse() != 0 { try!(write!(f, " lvdse"))}
        if self.lvdre() != 0 { try!(write!(f, " lvdre"))}
        if self.lvwie() != 0 { try!(write!(f, " lvwie"))}
        if self.lvwack() != 0 { try!(write!(f, " lvwack"))}
        if self.lvwf() != 0 { try!(write!(f, " lvwf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Power Management Status and Control 2 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Spmsc2(pub u8);
impl Spmsc2 {
    #[doc="Low-Voltage Warning Voltage Select"]
    #[inline] pub fn lvwv(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if LVWV != 0"]
    #[inline] pub fn test_lvwv(&self) -> bool {
        self.lvwv() != 0
    }

    #[doc="Sets the LVWV field."]
    #[inline] pub fn set_lvwv<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Low-Voltage Detect Voltage Select"]
    #[inline] pub fn lvdv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LVDV != 0"]
    #[inline] pub fn test_lvdv(&self) -> bool {
        self.lvdv() != 0
    }

    #[doc="Sets the LVDV field."]
    #[inline] pub fn set_lvdv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Spmsc2 {
    #[inline]
    fn from(other: u8) -> Self {
         Spmsc2(other)
    }
}

impl ::core::fmt::Display for Spmsc2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Spmsc2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lvwv() != 0 { try!(write!(f, " lvwv=0x{:x}", self.lvwv()))}
        if self.lvdv() != 0 { try!(write!(f, " lvdv"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

