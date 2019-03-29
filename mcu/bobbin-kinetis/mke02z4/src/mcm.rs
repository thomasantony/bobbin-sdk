::bobbin_mcu::periph!( MCM, Mcm, MCM_PERIPH, McmPeriph, MCM_OWNED, MCM_REF_COUNT, 0xf0003000, 0x00, 0x1c);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="MCM Peripheral"]
pub struct McmPeriph(pub usize); 

impl McmPeriph {
    #[doc="Get the PLASC Register."]
    #[inline] pub fn plasc_reg(&self) -> ::bobbin_mcu::register::Register<Plasc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Plasc, 0x8)
    }

    #[doc="Get the *mut pointer for the PLASC register."]
    #[inline] pub fn plasc_mut(&self) -> *mut Plasc { 
        self.plasc_reg().ptr()
    }

    #[doc="Get the *const pointer for the PLASC register."]
    #[inline] pub fn plasc_ptr(&self) -> *const Plasc { 
        self.plasc_reg().ptr()
    }

    #[doc="Read the PLASC register."]
    #[inline] pub fn plasc(&self) -> Plasc { 
        self.plasc_reg().read()
    }

    #[doc="Get the PLAMC Register."]
    #[inline] pub fn plamc_reg(&self) -> ::bobbin_mcu::register::Register<Plamc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Plamc, 0xa)
    }

    #[doc="Get the *mut pointer for the PLAMC register."]
    #[inline] pub fn plamc_mut(&self) -> *mut Plamc { 
        self.plamc_reg().ptr()
    }

    #[doc="Get the *const pointer for the PLAMC register."]
    #[inline] pub fn plamc_ptr(&self) -> *const Plamc { 
        self.plamc_reg().ptr()
    }

    #[doc="Read the PLAMC register."]
    #[inline] pub fn plamc(&self) -> Plamc { 
        self.plamc_reg().read()
    }

    #[doc="Get the PLACR Register."]
    #[inline] pub fn placr_reg(&self) -> ::bobbin_mcu::register::Register<Placr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Placr, 0xc)
    }

    #[doc="Get the *mut pointer for the PLACR register."]
    #[inline] pub fn placr_mut(&self) -> *mut Placr { 
        self.placr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PLACR register."]
    #[inline] pub fn placr_ptr(&self) -> *const Placr { 
        self.placr_reg().ptr()
    }

    #[doc="Read the PLACR register."]
    #[inline] pub fn placr(&self) -> Placr { 
        self.placr_reg().read()
    }

    #[doc="Write the PLACR register."]
    #[inline] pub fn write_placr(&self, value: Placr) -> &Self { 
        self.placr_reg().write(value);
        self
    }

    #[doc="Set the PLACR register."]
    #[inline] pub fn set_placr<F: FnOnce(Placr) -> Placr>(&self, f: F) -> &Self {
        self.placr_reg().set(f);
        self
    }

    #[doc="Modify the PLACR register."]
    #[inline] pub fn with_placr<F: FnOnce(Placr) -> Placr>(&self, f: F) -> &Self {
        self.placr_reg().with(f);
        self
    }

}

#[doc="Crossbar Switch (AXBS) Slave Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Plasc(pub u16);
impl Plasc {
    #[doc="Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch\'s slave input port."]
    #[inline] pub fn asc(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ASC != 0"]
    #[inline] pub fn test_asc(&self) -> bool {
        self.asc() != 0
    }

    #[doc="Sets the ASC field."]
    #[inline] pub fn set_asc<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Plasc {
    #[inline]
    fn from(other: u16) -> Self {
         Plasc(other)
    }
}

impl ::core::fmt::Display for Plasc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Plasc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.asc() != 0 { try!(write!(f, " asc=0x{:x}", self.asc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Crossbar Switch (AXBS) Master Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Plamc(pub u16);
impl Plamc {
    #[doc="Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port."]
    #[inline] pub fn amc(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if AMC != 0"]
    #[inline] pub fn test_amc(&self) -> bool {
        self.amc() != 0
    }

    #[doc="Sets the AMC field."]
    #[inline] pub fn set_amc<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Plamc {
    #[inline]
    fn from(other: u16) -> Self {
         Plamc(other)
    }
}

impl ::core::fmt::Display for Plamc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Plamc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.amc() != 0 { try!(write!(f, " amc=0x{:x}", self.amc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Platform Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Placr(pub u32);
impl Placr {
    #[doc="Clear Flash Controller Cache"]
    #[inline] pub fn cfcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CFCC != 0"]
    #[inline] pub fn test_cfcc(&self) -> bool {
        self.cfcc() != 0
    }

    #[doc="Sets the CFCC field."]
    #[inline] pub fn set_cfcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Disable Flash Controller Data Caching"]
    #[inline] pub fn dfcda(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DFCDA != 0"]
    #[inline] pub fn test_dfcda(&self) -> bool {
        self.dfcda() != 0
    }

    #[doc="Sets the DFCDA field."]
    #[inline] pub fn set_dfcda<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Disable Flash Controller Instruction Caching"]
    #[inline] pub fn dfcic(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DFCIC != 0"]
    #[inline] pub fn test_dfcic(&self) -> bool {
        self.dfcic() != 0
    }

    #[doc="Sets the DFCIC field."]
    #[inline] pub fn set_dfcic<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Disable Flash Controller Cache"]
    #[inline] pub fn dfcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DFCC != 0"]
    #[inline] pub fn test_dfcc(&self) -> bool {
        self.dfcc() != 0
    }

    #[doc="Sets the DFCC field."]
    #[inline] pub fn set_dfcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Enable Flash Data Speculation"]
    #[inline] pub fn efds(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if EFDS != 0"]
    #[inline] pub fn test_efds(&self) -> bool {
        self.efds() != 0
    }

    #[doc="Sets the EFDS field."]
    #[inline] pub fn set_efds<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Disable Flash Controller Speculation"]
    #[inline] pub fn dfcs(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if DFCS != 0"]
    #[inline] pub fn test_dfcs(&self) -> bool {
        self.dfcs() != 0
    }

    #[doc="Sets the DFCS field."]
    #[inline] pub fn set_dfcs<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Enable Stalling Flash Controller"]
    #[inline] pub fn esfc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if ESFC != 0"]
    #[inline] pub fn test_esfc(&self) -> bool {
        self.esfc() != 0
    }

    #[doc="Sets the ESFC field."]
    #[inline] pub fn set_esfc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Placr {
    #[inline]
    fn from(other: u32) -> Self {
         Placr(other)
    }
}

impl ::core::fmt::Display for Placr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Placr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cfcc() != 0 { try!(write!(f, " cfcc"))}
        if self.dfcda() != 0 { try!(write!(f, " dfcda"))}
        if self.dfcic() != 0 { try!(write!(f, " dfcic"))}
        if self.dfcc() != 0 { try!(write!(f, " dfcc"))}
        if self.efds() != 0 { try!(write!(f, " efds"))}
        if self.dfcs() != 0 { try!(write!(f, " dfcs"))}
        if self.esfc() != 0 { try!(write!(f, " esfc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

