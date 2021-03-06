
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DMAMUX Peripheral"]
pub struct DmamuxPeriph(pub usize); 

impl DmamuxPeriph {
    #[doc="Get the CHCFG Register."]
    #[inline] pub fn chcfg_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Chcfg, ::bobbin_bits::R16> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Chcfg, 0x0, 0x1)
    }

    #[doc="Get the *mut pointer for the CHCFG register."]
    #[inline] pub fn chcfg_mut<I: Into<::bobbin_bits::R16>>(&self, index: I) -> *mut Chcfg { 
        self.chcfg_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CHCFG register."]
    #[inline] pub fn chcfg_ptr<I: Into<::bobbin_bits::R16>>(&self, index: I) -> *const Chcfg { 
        self.chcfg_reg().ptr(index.into())
    }

    #[doc="Read the CHCFG register."]
    #[inline] pub fn chcfg<I: Into<::bobbin_bits::R16>>(&self, index: I) -> Chcfg { 
        self.chcfg_reg().read(index.into())
    }

    #[doc="Write the CHCFG register."]
    #[inline] pub fn write_chcfg<I: Into<::bobbin_bits::R16>>(&self, index: I, value: Chcfg) -> &Self {
        self.chcfg_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CHCFG register."]
    #[inline] pub fn set_chcfg<I: Into<::bobbin_bits::R16>, F: FnOnce(Chcfg) -> Chcfg>(&self, index: I, f: F) -> &Self {
        self.chcfg_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CHCFG register."]
    #[inline] pub fn with_chcfg<I: Into<::bobbin_bits::R16> + Copy, F: FnOnce(Chcfg) -> Chcfg>(&self, index: I, f: F) -> &Self {
        self.chcfg_reg().with(index.into(), f);
        self
    }

}

#[doc="Channel Configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Chcfg(pub u8);
impl Chcfg {
    #[doc="DMA Channel Source (Slot)"]
    #[inline] pub fn source(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if SOURCE != 0"]
    #[inline] pub fn test_source(&self) -> bool {
        self.source() != 0
    }

    #[doc="Sets the SOURCE field."]
    #[inline] pub fn set_source<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="DMA Channel Trigger Enable"]
    #[inline] pub fn trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TRIG != 0"]
    #[inline] pub fn test_trig(&self) -> bool {
        self.trig() != 0
    }

    #[doc="Sets the TRIG field."]
    #[inline] pub fn set_trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="DMA Channel Enable"]
    #[inline] pub fn enbl(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ENBL != 0"]
    #[inline] pub fn test_enbl(&self) -> bool {
        self.enbl() != 0
    }

    #[doc="Sets the ENBL field."]
    #[inline] pub fn set_enbl<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Chcfg {
    #[inline]
    fn from(other: u8) -> Self {
         Chcfg(other)
    }
}

impl ::core::fmt::Display for Chcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Chcfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.source() != 0 { try!(write!(f, " source=0x{:x}", self.source()))}
        if self.trig() != 0 { try!(write!(f, " trig"))}
        if self.enbl() != 0 { try!(write!(f, " enbl"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

