::bobbin_mcu::periph!( ROM, Rom, ROM_PERIPH, RomPeriph, ROM_OWNED, ROM_REF_COUNT, 0xf0002000, 0x00, 0x1b);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ROM Peripheral"]
pub struct RomPeriph(pub usize); 

impl RomPeriph {
    #[doc="Get the ENTRY Register."]
    #[inline] pub fn entry_reg(&self) -> ::bobbin_mcu::register::Register<Entry> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Entry, 0x0)
    }

    #[doc="Get the *mut pointer for the ENTRY register."]
    #[inline] pub fn entry_mut(&self) -> *mut Entry { 
        self.entry_reg().ptr()
    }

    #[doc="Get the *const pointer for the ENTRY register."]
    #[inline] pub fn entry_ptr(&self) -> *const Entry { 
        self.entry_reg().ptr()
    }

    #[doc="Read the ENTRY register."]
    #[inline] pub fn entry(&self) -> Entry { 
        self.entry_reg().read()
    }

    #[doc="Get the TABLEMARK Register."]
    #[inline] pub fn tablemark_reg(&self) -> ::bobbin_mcu::register::Register<Tablemark> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tablemark, 0x4)
    }

    #[doc="Get the *mut pointer for the TABLEMARK register."]
    #[inline] pub fn tablemark_mut(&self) -> *mut Tablemark { 
        self.tablemark_reg().ptr()
    }

    #[doc="Get the *const pointer for the TABLEMARK register."]
    #[inline] pub fn tablemark_ptr(&self) -> *const Tablemark { 
        self.tablemark_reg().ptr()
    }

    #[doc="Read the TABLEMARK register."]
    #[inline] pub fn tablemark(&self) -> Tablemark { 
        self.tablemark_reg().read()
    }

    #[doc="Get the SYSACCESS Register."]
    #[inline] pub fn sysaccess_reg(&self) -> ::bobbin_mcu::register::Register<Sysaccess> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sysaccess, 0xfcc)
    }

    #[doc="Get the *mut pointer for the SYSACCESS register."]
    #[inline] pub fn sysaccess_mut(&self) -> *mut Sysaccess { 
        self.sysaccess_reg().ptr()
    }

    #[doc="Get the *const pointer for the SYSACCESS register."]
    #[inline] pub fn sysaccess_ptr(&self) -> *const Sysaccess { 
        self.sysaccess_reg().ptr()
    }

    #[doc="Read the SYSACCESS register."]
    #[inline] pub fn sysaccess(&self) -> Sysaccess { 
        self.sysaccess_reg().read()
    }

    #[doc="Get the PERIPHID Register."]
    #[inline] pub fn periphid_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Periphid, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Periphid, 0xfd0, 0x4)
    }

    #[doc="Get the *mut pointer for the PERIPHID register."]
    #[inline] pub fn periphid_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Periphid { 
        self.periphid_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the PERIPHID register."]
    #[inline] pub fn periphid_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Periphid { 
        self.periphid_reg().ptr(index.into())
    }

    #[doc="Read the PERIPHID register."]
    #[inline] pub fn periphid<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Periphid { 
        self.periphid_reg().read(index.into())
    }

    #[doc="Get the COMPID Register."]
    #[inline] pub fn compid_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Compid, ::bobbin_bits::R4> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Compid, 0xff0, 0x4)
    }

    #[doc="Get the *mut pointer for the COMPID register."]
    #[inline] pub fn compid_mut<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *mut Compid { 
        self.compid_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the COMPID register."]
    #[inline] pub fn compid_ptr<I: Into<::bobbin_bits::R4>>(&self, index: I) -> *const Compid { 
        self.compid_reg().ptr(index.into())
    }

    #[doc="Read the COMPID register."]
    #[inline] pub fn compid<I: Into<::bobbin_bits::R4>>(&self, index: I) -> Compid { 
        self.compid_reg().read(index.into())
    }

}

#[doc="Entry"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Entry(pub u32);
impl Entry {
    #[doc="ENTRY"]
    #[inline] pub fn entry(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ENTRY != 0"]
    #[inline] pub fn test_entry(&self) -> bool {
        self.entry() != 0
    }

    #[doc="Sets the ENTRY field."]
    #[inline] pub fn set_entry<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Entry {
    #[inline]
    fn from(other: u32) -> Self {
         Entry(other)
    }
}

impl ::core::fmt::Display for Entry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Entry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="End of Table Marker Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tablemark(pub u32);
impl Tablemark {
    #[doc="Hardwired to 0x0000_0000"]
    #[inline] pub fn mark(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if MARK != 0"]
    #[inline] pub fn test_mark(&self) -> bool {
        self.mark() != 0
    }

    #[doc="Sets the MARK field."]
    #[inline] pub fn set_mark<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tablemark {
    #[inline]
    fn from(other: u32) -> Self {
         Tablemark(other)
    }
}

impl ::core::fmt::Display for Tablemark {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tablemark {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Access Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sysaccess(pub u32);
impl Sysaccess {
    #[doc="Hardwired to 0x0000_0001"]
    #[inline] pub fn sysaccess(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if SYSACCESS != 0"]
    #[inline] pub fn test_sysaccess(&self) -> bool {
        self.sysaccess() != 0
    }

    #[doc="Sets the SYSACCESS field."]
    #[inline] pub fn set_sysaccess<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sysaccess {
    #[inline]
    fn from(other: u32) -> Self {
         Sysaccess(other)
    }
}

impl ::core::fmt::Display for Sysaccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sysaccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Peripheral ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Periphid(pub u32);
impl Periphid {
    #[doc="Peripheral ID1 is hardwired to 0x0000_00E0; ID2 to 0x0000_0008; and all the others to 0x0000_0000."]
    #[inline] pub fn periphid(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PERIPHID != 0"]
    #[inline] pub fn test_periphid(&self) -> bool {
        self.periphid() != 0
    }

    #[doc="Sets the PERIPHID field."]
    #[inline] pub fn set_periphid<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Periphid {
    #[inline]
    fn from(other: u32) -> Self {
         Periphid(other)
    }
}

impl ::core::fmt::Display for Periphid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Periphid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Component ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Compid(pub u32);
impl Compid {
    #[doc="Component ID"]
    #[inline] pub fn compid(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if COMPID != 0"]
    #[inline] pub fn test_compid(&self) -> bool {
        self.compid() != 0
    }

    #[doc="Sets the COMPID field."]
    #[inline] pub fn set_compid<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Compid {
    #[inline]
    fn from(other: u32) -> Self {
         Compid(other)
    }
}

impl ::core::fmt::Display for Compid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Compid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

