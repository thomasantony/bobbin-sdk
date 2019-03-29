::bobbin_mcu::periph!( RTC, Rtc, RTC_PERIPH, RtcPeriph, RTC_OWNED, RTC_REF_COUNT, 0x4003d000, 0x00, 0x08);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="RTC Peripheral"]
pub struct RtcPeriph(pub usize); 

impl RtcPeriph {
    #[doc="Get the SC Register."]
    #[inline] pub fn sc_reg(&self) -> ::bobbin_mcu::register::Register<Sc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sc, 0x0)
    }

    #[doc="Get the *mut pointer for the SC register."]
    #[inline] pub fn sc_mut(&self) -> *mut Sc { 
        self.sc_reg().ptr()
    }

    #[doc="Get the *const pointer for the SC register."]
    #[inline] pub fn sc_ptr(&self) -> *const Sc { 
        self.sc_reg().ptr()
    }

    #[doc="Read the SC register."]
    #[inline] pub fn sc(&self) -> Sc { 
        self.sc_reg().read()
    }

    #[doc="Write the SC register."]
    #[inline] pub fn write_sc(&self, value: Sc) -> &Self { 
        self.sc_reg().write(value);
        self
    }

    #[doc="Set the SC register."]
    #[inline] pub fn set_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
        self.sc_reg().set(f);
        self
    }

    #[doc="Modify the SC register."]
    #[inline] pub fn with_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
        self.sc_reg().with(f);
        self
    }

    #[doc="Get the MOD Register."]
    #[inline] pub fn mod_reg(&self) -> ::bobbin_mcu::register::Register<Mod> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mod, 0x4)
    }

    #[doc="Get the *mut pointer for the MOD register."]
    #[inline] pub fn mod_mut(&self) -> *mut Mod { 
        self.mod_reg().ptr()
    }

    #[doc="Get the *const pointer for the MOD register."]
    #[inline] pub fn mod_ptr(&self) -> *const Mod { 
        self.mod_reg().ptr()
    }

    #[doc="Read the MOD register."]
    #[inline] pub fn _mod(&self) -> Mod { 
        self.mod_reg().read()
    }

    #[doc="Write the MOD register."]
    #[inline] pub fn write_mod(&self, value: Mod) -> &Self { 
        self.mod_reg().write(value);
        self
    }

    #[doc="Set the MOD register."]
    #[inline] pub fn set_mod<F: FnOnce(Mod) -> Mod>(&self, f: F) -> &Self {
        self.mod_reg().set(f);
        self
    }

    #[doc="Modify the MOD register."]
    #[inline] pub fn with_mod<F: FnOnce(Mod) -> Mod>(&self, f: F) -> &Self {
        self.mod_reg().with(f);
        self
    }

    #[doc="Get the CNT Register."]
    #[inline] pub fn cnt_reg(&self) -> ::bobbin_mcu::register::Register<Cnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cnt, 0x8)
    }

    #[doc="Get the *mut pointer for the CNT register."]
    #[inline] pub fn cnt_mut(&self) -> *mut Cnt { 
        self.cnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the CNT register."]
    #[inline] pub fn cnt_ptr(&self) -> *const Cnt { 
        self.cnt_reg().ptr()
    }

    #[doc="Read the CNT register."]
    #[inline] pub fn cnt(&self) -> Cnt { 
        self.cnt_reg().read()
    }

}

#[doc="RTC Status and Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc(pub u32);
impl Sc {
    #[doc="Real-Time Counter Output"]
    #[inline] pub fn rtco(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RTCO != 0"]
    #[inline] pub fn test_rtco(&self) -> bool {
        self.rtco() != 0
    }

    #[doc="Sets the RTCO field."]
    #[inline] pub fn set_rtco<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Real-Time Interrupt Enable"]
    #[inline] pub fn rtie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RTIE != 0"]
    #[inline] pub fn test_rtie(&self) -> bool {
        self.rtie() != 0
    }

    #[doc="Sets the RTIE field."]
    #[inline] pub fn set_rtie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Real-Time Interrupt Flag"]
    #[inline] pub fn rtif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if RTIF != 0"]
    #[inline] pub fn test_rtif(&self) -> bool {
        self.rtif() != 0
    }

    #[doc="Sets the RTIF field."]
    #[inline] pub fn set_rtif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Real-Time Clock Prescaler Select"]
    #[inline] pub fn rtcps(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x7) as u8) } // [10:8]
    }

    #[doc="Returns true if RTCPS != 0"]
    #[inline] pub fn test_rtcps(&self) -> bool {
        self.rtcps() != 0
    }

    #[doc="Sets the RTCPS field."]
    #[inline] pub fn set_rtcps<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Real-Time Clock Source Select"]
    #[inline] pub fn rtclks(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if RTCLKS != 0"]
    #[inline] pub fn test_rtclks(&self) -> bool {
        self.rtclks() != 0
    }

    #[doc="Sets the RTCLKS field."]
    #[inline] pub fn set_rtclks<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

}

impl From<u32> for Sc {
    #[inline]
    fn from(other: u32) -> Self {
         Sc(other)
    }
}

impl ::core::fmt::Display for Sc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rtco() != 0 { try!(write!(f, " rtco"))}
        if self.rtie() != 0 { try!(write!(f, " rtie"))}
        if self.rtif() != 0 { try!(write!(f, " rtif"))}
        if self.rtcps() != 0 { try!(write!(f, " rtcps=0x{:x}", self.rtcps()))}
        if self.rtclks() != 0 { try!(write!(f, " rtclks=0x{:x}", self.rtclks()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Modulo Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mod(pub u32);
impl Mod {
    #[doc="RTC Modulo"]
    #[inline] pub fn _mod(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MOD != 0"]
    #[inline] pub fn test_mod(&self) -> bool {
        self._mod() != 0
    }

    #[doc="Sets the MOD field."]
    #[inline] pub fn set_mod<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mod {
    #[inline]
    fn from(other: u32) -> Self {
         Mod(other)
    }
}

impl ::core::fmt::Display for Mod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self._mod() != 0 { try!(write!(f, " mod=0x{:x}", self._mod()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="RTC Counter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc="RTC Count"]
    #[inline] pub fn cnt(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if CNT != 0"]
    #[inline] pub fn test_cnt(&self) -> bool {
        self.cnt() != 0
    }

    #[doc="Sets the CNT field."]
    #[inline] pub fn set_cnt<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cnt {
    #[inline]
    fn from(other: u32) -> Self {
         Cnt(other)
    }
}

impl ::core::fmt::Display for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnt() != 0 { try!(write!(f, " cnt=0x{:x}", self.cnt()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

