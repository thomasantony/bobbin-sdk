::bobbin_mcu::periph!( KBI0, Kbi0, KBI0_PERIPH, KbiPeriph, KBI0_OWNED, KBI0_REF_COUNT, 0x40079000, 0x00, 0x16);
::bobbin_mcu::periph!( KBI1, Kbi1, KBI1_PERIPH, KbiPeriph, KBI1_OWNED, KBI1_REF_COUNT, 0x4007a000, 0x01, 0x17);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="KBI Peripheral"]
pub struct KbiPeriph(pub usize); 

impl KbiPeriph {
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

    #[doc="Get the PE Register."]
    #[inline] pub fn pe_reg(&self) -> ::bobbin_mcu::register::Register<Pe> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pe, 0x1)
    }

    #[doc="Get the *mut pointer for the PE register."]
    #[inline] pub fn pe_mut(&self) -> *mut Pe { 
        self.pe_reg().ptr()
    }

    #[doc="Get the *const pointer for the PE register."]
    #[inline] pub fn pe_ptr(&self) -> *const Pe { 
        self.pe_reg().ptr()
    }

    #[doc="Read the PE register."]
    #[inline] pub fn pe(&self) -> Pe { 
        self.pe_reg().read()
    }

    #[doc="Write the PE register."]
    #[inline] pub fn write_pe(&self, value: Pe) -> &Self { 
        self.pe_reg().write(value);
        self
    }

    #[doc="Set the PE register."]
    #[inline] pub fn set_pe<F: FnOnce(Pe) -> Pe>(&self, f: F) -> &Self {
        self.pe_reg().set(f);
        self
    }

    #[doc="Modify the PE register."]
    #[inline] pub fn with_pe<F: FnOnce(Pe) -> Pe>(&self, f: F) -> &Self {
        self.pe_reg().with(f);
        self
    }

    #[doc="Get the ES Register."]
    #[inline] pub fn es_reg(&self) -> ::bobbin_mcu::register::Register<Es> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Es, 0x2)
    }

    #[doc="Get the *mut pointer for the ES register."]
    #[inline] pub fn es_mut(&self) -> *mut Es { 
        self.es_reg().ptr()
    }

    #[doc="Get the *const pointer for the ES register."]
    #[inline] pub fn es_ptr(&self) -> *const Es { 
        self.es_reg().ptr()
    }

    #[doc="Read the ES register."]
    #[inline] pub fn es(&self) -> Es { 
        self.es_reg().read()
    }

    #[doc="Write the ES register."]
    #[inline] pub fn write_es(&self, value: Es) -> &Self { 
        self.es_reg().write(value);
        self
    }

    #[doc="Set the ES register."]
    #[inline] pub fn set_es<F: FnOnce(Es) -> Es>(&self, f: F) -> &Self {
        self.es_reg().set(f);
        self
    }

    #[doc="Modify the ES register."]
    #[inline] pub fn with_es<F: FnOnce(Es) -> Es>(&self, f: F) -> &Self {
        self.es_reg().with(f);
        self
    }

}

#[doc="KBI Status and Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc(pub u8);
impl Sc {
    #[doc="KBI Detection Mode"]
    #[inline] pub fn kbmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if KBMOD != 0"]
    #[inline] pub fn test_kbmod(&self) -> bool {
        self.kbmod() != 0
    }

    #[doc="Sets the KBMOD field."]
    #[inline] pub fn set_kbmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="KBI Interrupt Enable"]
    #[inline] pub fn kbie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if KBIE != 0"]
    #[inline] pub fn test_kbie(&self) -> bool {
        self.kbie() != 0
    }

    #[doc="Sets the KBIE field."]
    #[inline] pub fn set_kbie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="KBI Acknowledge"]
    #[inline] pub fn kback(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if KBACK != 0"]
    #[inline] pub fn test_kback(&self) -> bool {
        self.kback() != 0
    }

    #[doc="Sets the KBACK field."]
    #[inline] pub fn set_kback<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="KBI Interrupt Flag"]
    #[inline] pub fn kbf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if KBF != 0"]
    #[inline] pub fn test_kbf(&self) -> bool {
        self.kbf() != 0
    }

    #[doc="Sets the KBF field."]
    #[inline] pub fn set_kbf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u8> for Sc {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.kbmod() != 0 { try!(write!(f, " kbmod"))}
        if self.kbie() != 0 { try!(write!(f, " kbie"))}
        if self.kback() != 0 { try!(write!(f, " kback"))}
        if self.kbf() != 0 { try!(write!(f, " kbf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="KBI Pin Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pe(pub u8);
impl Pe {
    #[doc="KBI Pin Enables"]
    #[inline] pub fn kbipe(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if KBIPE != 0"]
    #[inline] pub fn test_kbipe(&self) -> bool {
        self.kbipe() != 0
    }

    #[doc="Sets the KBIPE field."]
    #[inline] pub fn set_kbipe<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Pe {
    #[inline]
    fn from(other: u8) -> Self {
         Pe(other)
    }
}

impl ::core::fmt::Display for Pe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pe {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.kbipe() != 0 { try!(write!(f, " kbipe=0x{:x}", self.kbipe()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="KBI Edge Select Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Es(pub u8);
impl Es {
    #[doc="KBI Edge Selects"]
    #[inline] pub fn kbedg(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if KBEDG != 0"]
    #[inline] pub fn test_kbedg(&self) -> bool {
        self.kbedg() != 0
    }

    #[doc="Sets the KBEDG field."]
    #[inline] pub fn set_kbedg<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Es {
    #[inline]
    fn from(other: u8) -> Self {
         Es(other)
    }
}

impl ::core::fmt::Display for Es {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Es {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.kbedg() != 0 { try!(write!(f, " kbedg=0x{:x}", self.kbedg()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

