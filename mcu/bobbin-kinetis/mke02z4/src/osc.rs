::bobbin_mcu::periph!( OSC, Osc, OSC_PERIPH, OscPeriph, OSC_OWNED, OSC_REF_COUNT, 0x40065000, 0x00, 0x0d);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="OSC Peripheral"]
pub struct OscPeriph(pub usize); 

impl OscPeriph {
    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x0)
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        self.cr_reg().read()
    }

    #[doc="Write the CR register."]
    #[inline] pub fn write_cr(&self, value: Cr) -> &Self { 
        self.cr_reg().write(value);
        self
    }

    #[doc="Set the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().set(f);
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().with(f);
        self
    }

}

#[doc="OSC Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u8);
impl Cr {
    #[doc="OSC Initialization"]
    #[inline] pub fn oscinit(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if OSCINIT != 0"]
    #[inline] pub fn test_oscinit(&self) -> bool {
        self.oscinit() != 0
    }

    #[doc="Sets the OSCINIT field."]
    #[inline] pub fn set_oscinit<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="High Gain Oscillator Select"]
    #[inline] pub fn hgo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if HGO != 0"]
    #[inline] pub fn test_hgo(&self) -> bool {
        self.hgo() != 0
    }

    #[doc="Sets the HGO field."]
    #[inline] pub fn set_hgo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Frequency Range Select"]
    #[inline] pub fn range(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RANGE != 0"]
    #[inline] pub fn test_range(&self) -> bool {
        self.range() != 0
    }

    #[doc="Sets the RANGE field."]
    #[inline] pub fn set_range<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="OSC Output Select"]
    #[inline] pub fn oscos(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if OSCOS != 0"]
    #[inline] pub fn test_oscos(&self) -> bool {
        self.oscos() != 0
    }

    #[doc="Sets the OSCOS field."]
    #[inline] pub fn set_oscos<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="OSC Enable in Stop mode"]
    #[inline] pub fn oscsten(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if OSCSTEN != 0"]
    #[inline] pub fn test_oscsten(&self) -> bool {
        self.oscsten() != 0
    }

    #[doc="Sets the OSCSTEN field."]
    #[inline] pub fn set_oscsten<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="OSC Enable"]
    #[inline] pub fn oscen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if OSCEN != 0"]
    #[inline] pub fn test_oscen(&self) -> bool {
        self.oscen() != 0
    }

    #[doc="Sets the OSCEN field."]
    #[inline] pub fn set_oscen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Cr {
    #[inline]
    fn from(other: u8) -> Self {
         Cr(other)
    }
}

impl ::core::fmt::Display for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oscinit() != 0 { try!(write!(f, " oscinit"))}
        if self.hgo() != 0 { try!(write!(f, " hgo"))}
        if self.range() != 0 { try!(write!(f, " range"))}
        if self.oscos() != 0 { try!(write!(f, " oscos"))}
        if self.oscsten() != 0 { try!(write!(f, " oscsten"))}
        if self.oscen() != 0 { try!(write!(f, " oscen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

