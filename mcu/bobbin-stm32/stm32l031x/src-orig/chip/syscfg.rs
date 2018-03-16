//! System configuration controller
#[allow(unused_imports)] use bobbin_common::*;

periph!(SYSCFG, Syscfg, 0x40010000);

#[doc="System configuration controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Syscfg(pub usize);
impl Syscfg {
    #[doc="Get the *mut pointer for the CFGR1 register."]
    #[inline] pub fn cfgr1_mut(&self) -> *mut Cfgr1 { 
        (self.0 + 0x0) as *mut Cfgr1
    }

    #[doc="Get the *const pointer for the CFGR1 register."]
    #[inline] pub fn cfgr1_ptr(&self) -> *const Cfgr1 { 
           self.cfgr1_mut()
    }

    #[doc="Read the CFGR1 register."]
    #[inline] pub fn cfgr1(&self) -> Cfgr1 { 
        unsafe {
            read_volatile(self.cfgr1_ptr())
        }
    }

    #[doc="Write the CFGR1 register."]
    #[inline] pub fn set_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr1_mut(), f(Cfgr1(0)));
        }
        self
    }

    #[doc="Modify the CFGR1 register."]
    #[inline] pub fn with_cfgr1<F: FnOnce(Cfgr1) -> Cfgr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr1_mut(), f(self.cfgr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGR2 register."]
    #[inline] pub fn cfgr2_mut(&self) -> *mut Cfgr2 { 
        (self.0 + 0x4) as *mut Cfgr2
    }

    #[doc="Get the *const pointer for the CFGR2 register."]
    #[inline] pub fn cfgr2_ptr(&self) -> *const Cfgr2 { 
           self.cfgr2_mut()
    }

    #[doc="Read the CFGR2 register."]
    #[inline] pub fn cfgr2(&self) -> Cfgr2 { 
        unsafe {
            read_volatile(self.cfgr2_ptr())
        }
    }

    #[doc="Write the CFGR2 register."]
    #[inline] pub fn set_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr2_mut(), f(Cfgr2(0)));
        }
        self
    }

    #[doc="Modify the CFGR2 register."]
    #[inline] pub fn with_cfgr2<F: FnOnce(Cfgr2) -> Cfgr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr2_mut(), f(self.cfgr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTICR1 register."]
    #[inline] pub fn exticr1_mut(&self) -> *mut Exticr1 { 
        (self.0 + 0x8) as *mut Exticr1
    }

    #[doc="Get the *const pointer for the EXTICR1 register."]
    #[inline] pub fn exticr1_ptr(&self) -> *const Exticr1 { 
           self.exticr1_mut()
    }

    #[doc="Read the EXTICR1 register."]
    #[inline] pub fn exticr1(&self) -> Exticr1 { 
        unsafe {
            read_volatile(self.exticr1_ptr())
        }
    }

    #[doc="Write the EXTICR1 register."]
    #[inline] pub fn set_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr1_mut(), f(Exticr1(0)));
        }
        self
    }

    #[doc="Modify the EXTICR1 register."]
    #[inline] pub fn with_exticr1<F: FnOnce(Exticr1) -> Exticr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr1_mut(), f(self.exticr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTICR2 register."]
    #[inline] pub fn exticr2_mut(&self) -> *mut Exticr2 { 
        (self.0 + 0xc) as *mut Exticr2
    }

    #[doc="Get the *const pointer for the EXTICR2 register."]
    #[inline] pub fn exticr2_ptr(&self) -> *const Exticr2 { 
           self.exticr2_mut()
    }

    #[doc="Read the EXTICR2 register."]
    #[inline] pub fn exticr2(&self) -> Exticr2 { 
        unsafe {
            read_volatile(self.exticr2_ptr())
        }
    }

    #[doc="Write the EXTICR2 register."]
    #[inline] pub fn set_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr2_mut(), f(Exticr2(0)));
        }
        self
    }

    #[doc="Modify the EXTICR2 register."]
    #[inline] pub fn with_exticr2<F: FnOnce(Exticr2) -> Exticr2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr2_mut(), f(self.exticr2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTICR3 register."]
    #[inline] pub fn exticr3_mut(&self) -> *mut Exticr3 { 
        (self.0 + 0x10) as *mut Exticr3
    }

    #[doc="Get the *const pointer for the EXTICR3 register."]
    #[inline] pub fn exticr3_ptr(&self) -> *const Exticr3 { 
           self.exticr3_mut()
    }

    #[doc="Read the EXTICR3 register."]
    #[inline] pub fn exticr3(&self) -> Exticr3 { 
        unsafe {
            read_volatile(self.exticr3_ptr())
        }
    }

    #[doc="Write the EXTICR3 register."]
    #[inline] pub fn set_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr3_mut(), f(Exticr3(0)));
        }
        self
    }

    #[doc="Modify the EXTICR3 register."]
    #[inline] pub fn with_exticr3<F: FnOnce(Exticr3) -> Exticr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr3_mut(), f(self.exticr3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the EXTICR4 register."]
    #[inline] pub fn exticr4_mut(&self) -> *mut Exticr4 { 
        (self.0 + 0x14) as *mut Exticr4
    }

    #[doc="Get the *const pointer for the EXTICR4 register."]
    #[inline] pub fn exticr4_ptr(&self) -> *const Exticr4 { 
           self.exticr4_mut()
    }

    #[doc="Read the EXTICR4 register."]
    #[inline] pub fn exticr4(&self) -> Exticr4 { 
        unsafe {
            read_volatile(self.exticr4_ptr())
        }
    }

    #[doc="Write the EXTICR4 register."]
    #[inline] pub fn set_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr4_mut(), f(Exticr4(0)));
        }
        self
    }

    #[doc="Modify the EXTICR4 register."]
    #[inline] pub fn with_exticr4<F: FnOnce(Exticr4) -> Exticr4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.exticr4_mut(), f(self.exticr4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CFGR3 register."]
    #[inline] pub fn cfgr3_mut(&self) -> *mut Cfgr3 { 
        (self.0 + 0x20) as *mut Cfgr3
    }

    #[doc="Get the *const pointer for the CFGR3 register."]
    #[inline] pub fn cfgr3_ptr(&self) -> *const Cfgr3 { 
           self.cfgr3_mut()
    }

    #[doc="Read the CFGR3 register."]
    #[inline] pub fn cfgr3(&self) -> Cfgr3 { 
        unsafe {
            read_volatile(self.cfgr3_ptr())
        }
    }

    #[doc="Write the CFGR3 register."]
    #[inline] pub fn set_cfgr3<F: FnOnce(Cfgr3) -> Cfgr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr3_mut(), f(Cfgr3(0)));
        }
        self
    }

    #[doc="Modify the CFGR3 register."]
    #[inline] pub fn with_cfgr3<F: FnOnce(Cfgr3) -> Cfgr3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cfgr3_mut(), f(self.cfgr3()));
        }
        self
    }

}

#[doc="SYSCFG configuration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr1(pub u32);
impl Cfgr1 {
    #[doc="Boot mode selected by the boot pins status bits"]
    #[inline] pub fn boot_mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if BOOT_MODE != 0"]
    #[inline] pub fn test_boot_mode(&self) -> bool {
        self.boot_mode() != 0
    }

    #[doc="Sets the BOOT_MODE field."]
    #[inline] pub fn set_boot_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Memory mapping selection bits"]
    #[inline] pub fn mem_mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MEM_MODE != 0"]
    #[inline] pub fn test_mem_mode(&self) -> bool {
        self.mem_mode() != 0
    }

    #[doc="Sets the MEM_MODE field."]
    #[inline] pub fn set_mem_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr1(other)
    }
}

impl ::core::fmt::Display for Cfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.boot_mode() != 0 { try!(write!(f, " boot_mode=0x{:x}", self.boot_mode()))}
        if self.mem_mode() != 0 { try!(write!(f, " mem_mode=0x{:x}", self.mem_mode()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SYSCFG configuration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr2(pub u32);
impl Cfgr2 {
    #[doc="I2C2 Fm+ drive capability enable bit"]
    #[inline] pub fn i2c2_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if I2C2_FMP != 0"]
    #[inline] pub fn test_i2c2_fmp(&self) -> bool {
        self.i2c2_fmp() != 0
    }

    #[doc="Sets the I2C2_FMP field."]
    #[inline] pub fn set_i2c2_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="I2C1 Fm+ drive capability enable bit"]
    #[inline] pub fn i2c1_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if I2C1_FMP != 0"]
    #[inline] pub fn test_i2c1_fmp(&self) -> bool {
        self.i2c1_fmp() != 0
    }

    #[doc="Sets the I2C1_FMP field."]
    #[inline] pub fn set_i2c1_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Fm+ drive capability on PB9 enable bit"]
    #[inline] pub fn i2c_pb9_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if I2C_PB9_FMP != 0"]
    #[inline] pub fn test_i2c_pb9_fmp(&self) -> bool {
        self.i2c_pb9_fmp() != 0
    }

    #[doc="Sets the I2C_PB9_FMP field."]
    #[inline] pub fn set_i2c_pb9_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Fm+ drive capability on PB8 enable bit"]
    #[inline] pub fn i2c_pb8_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if I2C_PB8_FMP != 0"]
    #[inline] pub fn test_i2c_pb8_fmp(&self) -> bool {
        self.i2c_pb8_fmp() != 0
    }

    #[doc="Sets the I2C_PB8_FMP field."]
    #[inline] pub fn set_i2c_pb8_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Fm+ drive capability on PB7 enable bit"]
    #[inline] pub fn i2c_pb7_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if I2C_PB7_FMP != 0"]
    #[inline] pub fn test_i2c_pb7_fmp(&self) -> bool {
        self.i2c_pb7_fmp() != 0
    }

    #[doc="Sets the I2C_PB7_FMP field."]
    #[inline] pub fn set_i2c_pb7_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Fm+ drive capability on PB6 enable bit"]
    #[inline] pub fn i2c_pb6_fmp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if I2C_PB6_FMP != 0"]
    #[inline] pub fn test_i2c_pb6_fmp(&self) -> bool {
        self.i2c_pb6_fmp() != 0
    }

    #[doc="Sets the I2C_PB6_FMP field."]
    #[inline] pub fn set_i2c_pb6_fmp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Configuration of internal VLCD rail connection to optional external capacitor"]
    #[inline] pub fn capa(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
    }

    #[doc="Returns true if CAPA != 0"]
    #[inline] pub fn test_capa(&self) -> bool {
        self.capa() != 0
    }

    #[doc="Sets the CAPA field."]
    #[inline] pub fn set_capa<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Firewall disable bit"]
    #[inline] pub fn fwdisen(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FWDISEN != 0"]
    #[inline] pub fn test_fwdisen(&self) -> bool {
        self.fwdisen() != 0
    }

    #[doc="Sets the FWDISEN field."]
    #[inline] pub fn set_fwdisen<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr2(other)
    }
}

impl ::core::fmt::Display for Cfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.i2c2_fmp() != 0 { try!(write!(f, " i2c2_fmp"))}
        if self.i2c1_fmp() != 0 { try!(write!(f, " i2c1_fmp"))}
        if self.i2c_pb9_fmp() != 0 { try!(write!(f, " i2c_pb9_fmp"))}
        if self.i2c_pb8_fmp() != 0 { try!(write!(f, " i2c_pb8_fmp"))}
        if self.i2c_pb7_fmp() != 0 { try!(write!(f, " i2c_pb7_fmp"))}
        if self.i2c_pb6_fmp() != 0 { try!(write!(f, " i2c_pb6_fmp"))}
        if self.capa() != 0 { try!(write!(f, " capa=0x{:x}", self.capa()))}
        if self.fwdisen() != 0 { try!(write!(f, " fwdisen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr1(pub u32);
impl Exticr1 {
    #[doc="EXTI x configuration (x = 0 to 3)"]
    #[inline] pub fn exti3(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI3 != 0"]
    #[inline] pub fn test_exti3(&self) -> bool {
        self.exti3() != 0
    }

    #[doc="Sets the EXTI3 field."]
    #[inline] pub fn set_exti3<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI x configuration (x = 0 to 3)"]
    #[inline] pub fn exti2(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI2 != 0"]
    #[inline] pub fn test_exti2(&self) -> bool {
        self.exti2() != 0
    }

    #[doc="Sets the EXTI2 field."]
    #[inline] pub fn set_exti2<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI x configuration (x = 0 to 3)"]
    #[inline] pub fn exti1(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI1 != 0"]
    #[inline] pub fn test_exti1(&self) -> bool {
        self.exti1() != 0
    }

    #[doc="Sets the EXTI1 field."]
    #[inline] pub fn set_exti1<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI x configuration (x = 0 to 3)"]
    #[inline] pub fn exti0(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI0 != 0"]
    #[inline] pub fn test_exti0(&self) -> bool {
        self.exti0() != 0
    }

    #[doc="Sets the EXTI0 field."]
    #[inline] pub fn set_exti0<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr1(other)
    }
}

impl ::core::fmt::Display for Exticr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti3() != 0 { try!(write!(f, " exti3=0x{:x}", self.exti3()))}
        if self.exti2() != 0 { try!(write!(f, " exti2=0x{:x}", self.exti2()))}
        if self.exti1() != 0 { try!(write!(f, " exti1=0x{:x}", self.exti1()))}
        if self.exti0() != 0 { try!(write!(f, " exti0=0x{:x}", self.exti0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr2(pub u32);
impl Exticr2 {
    #[doc="EXTI x configuration (x = 4 to 7)"]
    #[inline] pub fn exti7(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI7 != 0"]
    #[inline] pub fn test_exti7(&self) -> bool {
        self.exti7() != 0
    }

    #[doc="Sets the EXTI7 field."]
    #[inline] pub fn set_exti7<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI x configuration (x = 4 to 7)"]
    #[inline] pub fn exti6(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI6 != 0"]
    #[inline] pub fn test_exti6(&self) -> bool {
        self.exti6() != 0
    }

    #[doc="Sets the EXTI6 field."]
    #[inline] pub fn set_exti6<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI x configuration (x = 4 to 7)"]
    #[inline] pub fn exti5(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI5 != 0"]
    #[inline] pub fn test_exti5(&self) -> bool {
        self.exti5() != 0
    }

    #[doc="Sets the EXTI5 field."]
    #[inline] pub fn set_exti5<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI x configuration (x = 4 to 7)"]
    #[inline] pub fn exti4(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI4 != 0"]
    #[inline] pub fn test_exti4(&self) -> bool {
        self.exti4() != 0
    }

    #[doc="Sets the EXTI4 field."]
    #[inline] pub fn set_exti4<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr2 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr2(other)
    }
}

impl ::core::fmt::Display for Exticr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti7() != 0 { try!(write!(f, " exti7=0x{:x}", self.exti7()))}
        if self.exti6() != 0 { try!(write!(f, " exti6=0x{:x}", self.exti6()))}
        if self.exti5() != 0 { try!(write!(f, " exti5=0x{:x}", self.exti5()))}
        if self.exti4() != 0 { try!(write!(f, " exti4=0x{:x}", self.exti4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr3(pub u32);
impl Exticr3 {
    #[doc="EXTI x configuration (x = 8 to 11)"]
    #[inline] pub fn exti11(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI11 != 0"]
    #[inline] pub fn test_exti11(&self) -> bool {
        self.exti11() != 0
    }

    #[doc="Sets the EXTI11 field."]
    #[inline] pub fn set_exti11<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI10"]
    #[inline] pub fn exti10(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI10 != 0"]
    #[inline] pub fn test_exti10(&self) -> bool {
        self.exti10() != 0
    }

    #[doc="Sets the EXTI10 field."]
    #[inline] pub fn set_exti10<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI x configuration (x = 8 to 11)"]
    #[inline] pub fn exti9(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI9 != 0"]
    #[inline] pub fn test_exti9(&self) -> bool {
        self.exti9() != 0
    }

    #[doc="Sets the EXTI9 field."]
    #[inline] pub fn set_exti9<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI x configuration (x = 8 to 11)"]
    #[inline] pub fn exti8(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI8 != 0"]
    #[inline] pub fn test_exti8(&self) -> bool {
        self.exti8() != 0
    }

    #[doc="Sets the EXTI8 field."]
    #[inline] pub fn set_exti8<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr3(other)
    }
}

impl ::core::fmt::Display for Exticr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti11() != 0 { try!(write!(f, " exti11=0x{:x}", self.exti11()))}
        if self.exti10() != 0 { try!(write!(f, " exti10=0x{:x}", self.exti10()))}
        if self.exti9() != 0 { try!(write!(f, " exti9=0x{:x}", self.exti9()))}
        if self.exti8() != 0 { try!(write!(f, " exti8=0x{:x}", self.exti8()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="external interrupt configuration register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exticr4(pub u32);
impl Exticr4 {
    #[doc="EXTI x configuration (x = 12 to 15)"]
    #[inline] pub fn exti15(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if EXTI15 != 0"]
    #[inline] pub fn test_exti15(&self) -> bool {
        self.exti15() != 0
    }

    #[doc="Sets the EXTI15 field."]
    #[inline] pub fn set_exti15<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="EXTI14"]
    #[inline] pub fn exti14(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if EXTI14 != 0"]
    #[inline] pub fn test_exti14(&self) -> bool {
        self.exti14() != 0
    }

    #[doc="Sets the EXTI14 field."]
    #[inline] pub fn set_exti14<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="EXTI13"]
    #[inline] pub fn exti13(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if EXTI13 != 0"]
    #[inline] pub fn test_exti13(&self) -> bool {
        self.exti13() != 0
    }

    #[doc="Sets the EXTI13 field."]
    #[inline] pub fn set_exti13<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="EXTI12"]
    #[inline] pub fn exti12(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if EXTI12 != 0"]
    #[inline] pub fn test_exti12(&self) -> bool {
        self.exti12() != 0
    }

    #[doc="Sets the EXTI12 field."]
    #[inline] pub fn set_exti12<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Exticr4 {
    #[inline]
    fn from(other: u32) -> Self {
         Exticr4(other)
    }
}

impl ::core::fmt::Display for Exticr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exticr4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.exti15() != 0 { try!(write!(f, " exti15=0x{:x}", self.exti15()))}
        if self.exti14() != 0 { try!(write!(f, " exti14=0x{:x}", self.exti14()))}
        if self.exti13() != 0 { try!(write!(f, " exti13=0x{:x}", self.exti13()))}
        if self.exti12() != 0 { try!(write!(f, " exti12=0x{:x}", self.exti12()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SYSCFG configuration register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr3(pub u32);
impl Cfgr3 {
    #[doc="REF_CTRL lock bit"]
    #[inline] pub fn ref_lock(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if REF_LOCK != 0"]
    #[inline] pub fn test_ref_lock(&self) -> bool {
        self.ref_lock() != 0
    }

    #[doc="Sets the REF_LOCK field."]
    #[inline] pub fn set_ref_lock<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="VREFINT ready flag"]
    #[inline] pub fn vrefint_rdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if VREFINT_RDYF != 0"]
    #[inline] pub fn test_vrefint_rdyf(&self) -> bool {
        self.vrefint_rdyf() != 0
    }

    #[doc="Sets the VREFINT_RDYF field."]
    #[inline] pub fn set_vrefint_rdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="VREFINT for comparator ready flag"]
    #[inline] pub fn vrefint_comp_rdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if VREFINT_COMP_RDYF != 0"]
    #[inline] pub fn test_vrefint_comp_rdyf(&self) -> bool {
        self.vrefint_comp_rdyf() != 0
    }

    #[doc="Sets the VREFINT_COMP_RDYF field."]
    #[inline] pub fn set_vrefint_comp_rdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="VREFINT for ADC ready flag"]
    #[inline] pub fn vrefint_adc_rdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if VREFINT_ADC_RDYF != 0"]
    #[inline] pub fn test_vrefint_adc_rdyf(&self) -> bool {
        self.vrefint_adc_rdyf() != 0
    }

    #[doc="Sets the VREFINT_ADC_RDYF field."]
    #[inline] pub fn set_vrefint_adc_rdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Sensor for ADC ready flag"]
    #[inline] pub fn sensor_adc_rdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if SENSOR_ADC_RDYF != 0"]
    #[inline] pub fn test_sensor_adc_rdyf(&self) -> bool {
        self.sensor_adc_rdyf() != 0
    }

    #[doc="Sets the SENSOR_ADC_RDYF field."]
    #[inline] pub fn set_sensor_adc_rdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="VREFINT for 48 MHz RC oscillator ready flag"]
    #[inline] pub fn ref_rc48mhz_rdyf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if REF_RC48MHz_RDYF != 0"]
    #[inline] pub fn test_ref_rc48mhz_rdyf(&self) -> bool {
        self.ref_rc48mhz_rdyf() != 0
    }

    #[doc="Sets the REF_RC48MHz_RDYF field."]
    #[inline] pub fn set_ref_rc48mhz_rdyf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="VREFINT reference for 48 MHz RC oscillator enable bit"]
    #[inline] pub fn enref_rc48mhz(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if ENREF_RC48MHz != 0"]
    #[inline] pub fn test_enref_rc48mhz(&self) -> bool {
        self.enref_rc48mhz() != 0
    }

    #[doc="Sets the ENREF_RC48MHz field."]
    #[inline] pub fn set_enref_rc48mhz<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="VREFINT reference for comparator 2 enable bit"]
    #[inline] pub fn enbuf_vrefint_comp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if ENBUF_VREFINT_COMP != 0"]
    #[inline] pub fn test_enbuf_vrefint_comp(&self) -> bool {
        self.enbuf_vrefint_comp() != 0
    }

    #[doc="Sets the ENBUF_VREFINT_COMP field."]
    #[inline] pub fn set_enbuf_vrefint_comp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Sensor reference for ADC enable bit"]
    #[inline] pub fn enbuf_sensor_adc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if ENBUF_SENSOR_ADC != 0"]
    #[inline] pub fn test_enbuf_sensor_adc(&self) -> bool {
        self.enbuf_sensor_adc() != 0
    }

    #[doc="Sets the ENBUF_SENSOR_ADC field."]
    #[inline] pub fn set_enbuf_sensor_adc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="VREFINT reference for ADC enable bit"]
    #[inline] pub fn enbuf_bgap_adc(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if ENBUF_BGAP_ADC != 0"]
    #[inline] pub fn test_enbuf_bgap_adc(&self) -> bool {
        self.enbuf_bgap_adc() != 0
    }

    #[doc="Sets the ENBUF_BGAP_ADC field."]
    #[inline] pub fn set_enbuf_bgap_adc<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="BGAP_ADC connection bit"]
    #[inline] pub fn sel_vref_out(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if SEL_VREF_OUT != 0"]
    #[inline] pub fn test_sel_vref_out(&self) -> bool {
        self.sel_vref_out() != 0
    }

    #[doc="Sets the SEL_VREF_OUT field."]
    #[inline] pub fn set_sel_vref_out<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Vref Enable bit"]
    #[inline] pub fn en_bgap(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EN_BGAP != 0"]
    #[inline] pub fn test_en_bgap(&self) -> bool {
        self.en_bgap() != 0
    }

    #[doc="Sets the EN_BGAP field."]
    #[inline] pub fn set_en_bgap<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cfgr3 {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr3(other)
    }
}

impl ::core::fmt::Display for Cfgr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ref_lock() != 0 { try!(write!(f, " ref_lock"))}
        if self.vrefint_rdyf() != 0 { try!(write!(f, " vrefint_rdyf"))}
        if self.vrefint_comp_rdyf() != 0 { try!(write!(f, " vrefint_comp_rdyf"))}
        if self.vrefint_adc_rdyf() != 0 { try!(write!(f, " vrefint_adc_rdyf"))}
        if self.sensor_adc_rdyf() != 0 { try!(write!(f, " sensor_adc_rdyf"))}
        if self.ref_rc48mhz_rdyf() != 0 { try!(write!(f, " ref_rc48mhz_rdyf"))}
        if self.enref_rc48mhz() != 0 { try!(write!(f, " enref_rc48mhz"))}
        if self.enbuf_vrefint_comp() != 0 { try!(write!(f, " enbuf_vrefint_comp"))}
        if self.enbuf_sensor_adc() != 0 { try!(write!(f, " enbuf_sensor_adc"))}
        if self.enbuf_bgap_adc() != 0 { try!(write!(f, " enbuf_bgap_adc"))}
        if self.sel_vref_out() != 0 { try!(write!(f, " sel_vref_out=0x{:x}", self.sel_vref_out()))}
        if self.en_bgap() != 0 { try!(write!(f, " en_bgap"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

