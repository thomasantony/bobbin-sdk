//! Multipurpose Clock Generator module
#[allow(unused_imports)] use bobbin_common::*;

periph!(MCG, Mcg, 0x40064000);

#[doc="Multipurpose Clock Generator module"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mcg(pub usize);
impl Mcg {
    #[doc="Get the *mut pointer for the C1 register."]
    #[inline] pub fn c1_mut(&self) -> *mut C1 { 
        (self.0 + 0x0) as *mut C1
    }

    #[doc="Get the *const pointer for the C1 register."]
    #[inline] pub fn c1_ptr(&self) -> *const C1 { 
           self.c1_mut()
    }

    #[doc="Read the C1 register."]
    #[inline] pub fn c1(&self) -> C1 { 
        unsafe {
            read_volatile(self.c1_ptr())
        }
    }

    #[doc="Write the C1 register."]
    #[inline] pub fn set_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c1_mut(), f(C1(0)));
        }
        self
    }

    #[doc="Modify the C1 register."]
    #[inline] pub fn with_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c1_mut(), f(self.c1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the C2 register."]
    #[inline] pub fn c2_mut(&self) -> *mut C2 { 
        (self.0 + 0x1) as *mut C2
    }

    #[doc="Get the *const pointer for the C2 register."]
    #[inline] pub fn c2_ptr(&self) -> *const C2 { 
           self.c2_mut()
    }

    #[doc="Read the C2 register."]
    #[inline] pub fn c2(&self) -> C2 { 
        unsafe {
            read_volatile(self.c2_ptr())
        }
    }

    #[doc="Write the C2 register."]
    #[inline] pub fn set_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c2_mut(), f(C2(0)));
        }
        self
    }

    #[doc="Modify the C2 register."]
    #[inline] pub fn with_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c2_mut(), f(self.c2()));
        }
        self
    }

    #[doc="Get the *mut pointer for the C3 register."]
    #[inline] pub fn c3_mut(&self) -> *mut C3 { 
        (self.0 + 0x2) as *mut C3
    }

    #[doc="Get the *const pointer for the C3 register."]
    #[inline] pub fn c3_ptr(&self) -> *const C3 { 
           self.c3_mut()
    }

    #[doc="Read the C3 register."]
    #[inline] pub fn c3(&self) -> C3 { 
        unsafe {
            read_volatile(self.c3_ptr())
        }
    }

    #[doc="Write the C3 register."]
    #[inline] pub fn set_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c3_mut(), f(C3(0)));
        }
        self
    }

    #[doc="Modify the C3 register."]
    #[inline] pub fn with_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c3_mut(), f(self.c3()));
        }
        self
    }

    #[doc="Get the *mut pointer for the C4 register."]
    #[inline] pub fn c4_mut(&self) -> *mut C4 { 
        (self.0 + 0x3) as *mut C4
    }

    #[doc="Get the *const pointer for the C4 register."]
    #[inline] pub fn c4_ptr(&self) -> *const C4 { 
           self.c4_mut()
    }

    #[doc="Read the C4 register."]
    #[inline] pub fn c4(&self) -> C4 { 
        unsafe {
            read_volatile(self.c4_ptr())
        }
    }

    #[doc="Write the C4 register."]
    #[inline] pub fn set_c4<F: FnOnce(C4) -> C4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c4_mut(), f(C4(0)));
        }
        self
    }

    #[doc="Modify the C4 register."]
    #[inline] pub fn with_c4<F: FnOnce(C4) -> C4>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c4_mut(), f(self.c4()));
        }
        self
    }

    #[doc="Get the *mut pointer for the C5 register."]
    #[inline] pub fn c5_mut(&self) -> *mut C5 { 
        (self.0 + 0x4) as *mut C5
    }

    #[doc="Get the *const pointer for the C5 register."]
    #[inline] pub fn c5_ptr(&self) -> *const C5 { 
           self.c5_mut()
    }

    #[doc="Read the C5 register."]
    #[inline] pub fn c5(&self) -> C5 { 
        unsafe {
            read_volatile(self.c5_ptr())
        }
    }

    #[doc="Write the C5 register."]
    #[inline] pub fn set_c5<F: FnOnce(C5) -> C5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c5_mut(), f(C5(0)));
        }
        self
    }

    #[doc="Modify the C5 register."]
    #[inline] pub fn with_c5<F: FnOnce(C5) -> C5>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c5_mut(), f(self.c5()));
        }
        self
    }

    #[doc="Get the *mut pointer for the C6 register."]
    #[inline] pub fn c6_mut(&self) -> *mut C6 { 
        (self.0 + 0x5) as *mut C6
    }

    #[doc="Get the *const pointer for the C6 register."]
    #[inline] pub fn c6_ptr(&self) -> *const C6 { 
           self.c6_mut()
    }

    #[doc="Read the C6 register."]
    #[inline] pub fn c6(&self) -> C6 { 
        unsafe {
            read_volatile(self.c6_ptr())
        }
    }

    #[doc="Write the C6 register."]
    #[inline] pub fn set_c6<F: FnOnce(C6) -> C6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c6_mut(), f(C6(0)));
        }
        self
    }

    #[doc="Modify the C6 register."]
    #[inline] pub fn with_c6<F: FnOnce(C6) -> C6>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c6_mut(), f(self.c6()));
        }
        self
    }

    #[doc="Get the *mut pointer for the S register."]
    #[inline] pub fn s_mut(&self) -> *mut S { 
        (self.0 + 0x6) as *mut S
    }

    #[doc="Get the *const pointer for the S register."]
    #[inline] pub fn s_ptr(&self) -> *const S { 
           self.s_mut()
    }

    #[doc="Read the S register."]
    #[inline] pub fn s(&self) -> S { 
        unsafe {
            read_volatile(self.s_ptr())
        }
    }

    #[doc="Write the S register."]
    #[inline] pub fn set_s<F: FnOnce(S) -> S>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.s_mut(), f(S(0)));
        }
        self
    }

    #[doc="Modify the S register."]
    #[inline] pub fn with_s<F: FnOnce(S) -> S>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.s_mut(), f(self.s()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SC register."]
    #[inline] pub fn sc_mut(&self) -> *mut Sc { 
        (self.0 + 0x8) as *mut Sc
    }

    #[doc="Get the *const pointer for the SC register."]
    #[inline] pub fn sc_ptr(&self) -> *const Sc { 
           self.sc_mut()
    }

    #[doc="Read the SC register."]
    #[inline] pub fn sc(&self) -> Sc { 
        unsafe {
            read_volatile(self.sc_ptr())
        }
    }

    #[doc="Write the SC register."]
    #[inline] pub fn set_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sc_mut(), f(Sc(0)));
        }
        self
    }

    #[doc="Modify the SC register."]
    #[inline] pub fn with_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sc_mut(), f(self.sc()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ATCVH register."]
    #[inline] pub fn atcvh_mut(&self) -> *mut Atcvh { 
        (self.0 + 0xa) as *mut Atcvh
    }

    #[doc="Get the *const pointer for the ATCVH register."]
    #[inline] pub fn atcvh_ptr(&self) -> *const Atcvh { 
           self.atcvh_mut()
    }

    #[doc="Read the ATCVH register."]
    #[inline] pub fn atcvh(&self) -> Atcvh { 
        unsafe {
            read_volatile(self.atcvh_ptr())
        }
    }

    #[doc="Write the ATCVH register."]
    #[inline] pub fn set_atcvh<F: FnOnce(Atcvh) -> Atcvh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atcvh_mut(), f(Atcvh(0)));
        }
        self
    }

    #[doc="Modify the ATCVH register."]
    #[inline] pub fn with_atcvh<F: FnOnce(Atcvh) -> Atcvh>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atcvh_mut(), f(self.atcvh()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ATCVL register."]
    #[inline] pub fn atcvl_mut(&self) -> *mut Atcvl { 
        (self.0 + 0xb) as *mut Atcvl
    }

    #[doc="Get the *const pointer for the ATCVL register."]
    #[inline] pub fn atcvl_ptr(&self) -> *const Atcvl { 
           self.atcvl_mut()
    }

    #[doc="Read the ATCVL register."]
    #[inline] pub fn atcvl(&self) -> Atcvl { 
        unsafe {
            read_volatile(self.atcvl_ptr())
        }
    }

    #[doc="Write the ATCVL register."]
    #[inline] pub fn set_atcvl<F: FnOnce(Atcvl) -> Atcvl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atcvl_mut(), f(Atcvl(0)));
        }
        self
    }

    #[doc="Modify the ATCVL register."]
    #[inline] pub fn with_atcvl<F: FnOnce(Atcvl) -> Atcvl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.atcvl_mut(), f(self.atcvl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the C7 register."]
    #[inline] pub fn c7_mut(&self) -> *mut C7 { 
        (self.0 + 0xc) as *mut C7
    }

    #[doc="Get the *const pointer for the C7 register."]
    #[inline] pub fn c7_ptr(&self) -> *const C7 { 
           self.c7_mut()
    }

    #[doc="Read the C7 register."]
    #[inline] pub fn c7(&self) -> C7 { 
        unsafe {
            read_volatile(self.c7_ptr())
        }
    }

    #[doc="Write the C7 register."]
    #[inline] pub fn set_c7<F: FnOnce(C7) -> C7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c7_mut(), f(C7(0)));
        }
        self
    }

    #[doc="Modify the C7 register."]
    #[inline] pub fn with_c7<F: FnOnce(C7) -> C7>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c7_mut(), f(self.c7()));
        }
        self
    }

    #[doc="Get the *mut pointer for the C8 register."]
    #[inline] pub fn c8_mut(&self) -> *mut C8 { 
        (self.0 + 0xd) as *mut C8
    }

    #[doc="Get the *const pointer for the C8 register."]
    #[inline] pub fn c8_ptr(&self) -> *const C8 { 
           self.c8_mut()
    }

    #[doc="Read the C8 register."]
    #[inline] pub fn c8(&self) -> C8 { 
        unsafe {
            read_volatile(self.c8_ptr())
        }
    }

    #[doc="Write the C8 register."]
    #[inline] pub fn set_c8<F: FnOnce(C8) -> C8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c8_mut(), f(C8(0)));
        }
        self
    }

    #[doc="Modify the C8 register."]
    #[inline] pub fn with_c8<F: FnOnce(C8) -> C8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.c8_mut(), f(self.c8()));
        }
        self
    }

}

#[doc="MCG Control 1 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1(pub u8);
impl C1 {
    #[doc="Internal Reference Stop Enable"]
    #[inline] pub fn irefsten(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IREFSTEN != 0"]
    #[inline] pub fn test_irefsten(&self) -> bool {
        self.irefsten() != 0
    }

    #[doc="Sets the IREFSTEN field."]
    #[inline] pub fn set_irefsten<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Internal Reference Clock Enable"]
    #[inline] pub fn irclken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IRCLKEN != 0"]
    #[inline] pub fn test_irclken(&self) -> bool {
        self.irclken() != 0
    }

    #[doc="Sets the IRCLKEN field."]
    #[inline] pub fn set_irclken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Internal Reference Select"]
    #[inline] pub fn irefs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IREFS != 0"]
    #[inline] pub fn test_irefs(&self) -> bool {
        self.irefs() != 0
    }

    #[doc="Sets the IREFS field."]
    #[inline] pub fn set_irefs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FLL External Reference Divider"]
    #[inline] pub fn frdiv(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if FRDIV != 0"]
    #[inline] pub fn test_frdiv(&self) -> bool {
        self.frdiv() != 0
    }

    #[doc="Sets the FRDIV field."]
    #[inline] pub fn set_frdiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Clock Source Select"]
    #[inline] pub fn clks(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if CLKS != 0"]
    #[inline] pub fn test_clks(&self) -> bool {
        self.clks() != 0
    }

    #[doc="Sets the CLKS field."]
    #[inline] pub fn set_clks<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
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
        if self.frdiv() != 0 { try!(write!(f, " frdiv=0x{:x}", self.frdiv()))}
        if self.clks() != 0 { try!(write!(f, " clks=0x{:x}", self.clks()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MCG Control 2 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2(pub u8);
impl C2 {
    #[doc="Internal Reference Clock Select"]
    #[inline] pub fn ircs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IRCS != 0"]
    #[inline] pub fn test_ircs(&self) -> bool {
        self.ircs() != 0
    }

    #[doc="Sets the IRCS field."]
    #[inline] pub fn set_ircs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Low Power Select"]
    #[inline] pub fn lp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LP != 0"]
    #[inline] pub fn test_lp(&self) -> bool {
        self.lp() != 0
    }

    #[doc="Sets the LP field."]
    #[inline] pub fn set_lp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="External Reference Select"]
    #[inline] pub fn erefs(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if EREFS != 0"]
    #[inline] pub fn test_erefs(&self) -> bool {
        self.erefs() != 0
    }

    #[doc="Sets the EREFS field."]
    #[inline] pub fn set_erefs<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="High Gain Oscillator Select"]
    #[inline] pub fn hgo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if HGO != 0"]
    #[inline] pub fn test_hgo(&self) -> bool {
        self.hgo() != 0
    }

    #[doc="Sets the HGO field."]
    #[inline] pub fn set_hgo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Frequency Range Select"]
    #[inline] pub fn range(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if RANGE != 0"]
    #[inline] pub fn test_range(&self) -> bool {
        self.range() != 0
    }

    #[doc="Sets the RANGE field."]
    #[inline] pub fn set_range<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Fast Internal Reference Clock Fine Trim"]
    #[inline] pub fn fcftrim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FCFTRIM != 0"]
    #[inline] pub fn test_fcftrim(&self) -> bool {
        self.fcftrim() != 0
    }

    #[doc="Sets the FCFTRIM field."]
    #[inline] pub fn set_fcftrim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Loss of Clock Reset Enable"]
    #[inline] pub fn locre0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LOCRE0 != 0"]
    #[inline] pub fn test_locre0(&self) -> bool {
        self.locre0() != 0
    }

    #[doc="Sets the LOCRE0 field."]
    #[inline] pub fn set_locre0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.ircs() != 0 { try!(write!(f, " ircs"))}
        if self.lp() != 0 { try!(write!(f, " lp"))}
        if self.erefs() != 0 { try!(write!(f, " erefs"))}
        if self.hgo() != 0 { try!(write!(f, " hgo"))}
        if self.range() != 0 { try!(write!(f, " range=0x{:x}", self.range()))}
        if self.fcftrim() != 0 { try!(write!(f, " fcftrim"))}
        if self.locre0() != 0 { try!(write!(f, " locre0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MCG Control 3 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C3(pub u8);
impl C3 {
    #[doc="Slow Internal Reference Clock Trim Setting"]
    #[inline] pub fn sctrim(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SCTRIM != 0"]
    #[inline] pub fn test_sctrim(&self) -> bool {
        self.sctrim() != 0
    }

    #[doc="Sets the SCTRIM field."]
    #[inline] pub fn set_sctrim<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
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

#[doc="MCG Control 4 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C4(pub u8);
impl C4 {
    #[doc="Slow Internal Reference Clock Fine Trim"]
    #[inline] pub fn scftrim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SCFTRIM != 0"]
    #[inline] pub fn test_scftrim(&self) -> bool {
        self.scftrim() != 0
    }

    #[doc="Sets the SCFTRIM field."]
    #[inline] pub fn set_scftrim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fast Internal Reference Clock Trim Setting"]
    #[inline] pub fn fctrim(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0xf) as u8) } // [4:1]
    }

    #[doc="Returns true if FCTRIM != 0"]
    #[inline] pub fn test_fctrim(&self) -> bool {
        self.fctrim() != 0
    }

    #[doc="Sets the FCTRIM field."]
    #[inline] pub fn set_fctrim<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="DCO Range Select"]
    #[inline] pub fn drst_drs(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if DRST_DRS != 0"]
    #[inline] pub fn test_drst_drs(&self) -> bool {
        self.drst_drs() != 0
    }

    #[doc="Sets the DRST_DRS field."]
    #[inline] pub fn set_drst_drs<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="DCO Maximum Frequency with 32.768 kHz Reference"]
    #[inline] pub fn dmx32(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DMX32 != 0"]
    #[inline] pub fn test_dmx32(&self) -> bool {
        self.dmx32() != 0
    }

    #[doc="Sets the DMX32 field."]
    #[inline] pub fn set_dmx32<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.fctrim() != 0 { try!(write!(f, " fctrim=0x{:x}", self.fctrim()))}
        if self.drst_drs() != 0 { try!(write!(f, " drst_drs=0x{:x}", self.drst_drs()))}
        if self.dmx32() != 0 { try!(write!(f, " dmx32"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MCG Control 5 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C5(pub u8);
impl C5 {
    #[doc="PLL External Reference Divider"]
    #[inline] pub fn prdiv0(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if PRDIV0 != 0"]
    #[inline] pub fn test_prdiv0(&self) -> bool {
        self.prdiv0() != 0
    }

    #[doc="Sets the PRDIV0 field."]
    #[inline] pub fn set_prdiv0<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PLL Stop Enable"]
    #[inline] pub fn pllsten0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PLLSTEN0 != 0"]
    #[inline] pub fn test_pllsten0(&self) -> bool {
        self.pllsten0() != 0
    }

    #[doc="Sets the PLLSTEN0 field."]
    #[inline] pub fn set_pllsten0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="PLL Clock Enable"]
    #[inline] pub fn pllclken0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PLLCLKEN0 != 0"]
    #[inline] pub fn test_pllclken0(&self) -> bool {
        self.pllclken0() != 0
    }

    #[doc="Sets the PLLCLKEN0 field."]
    #[inline] pub fn set_pllclken0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for C5 {
    #[inline]
    fn from(other: u8) -> Self {
         C5(other)
    }
}

impl ::core::fmt::Display for C5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.prdiv0() != 0 { try!(write!(f, " prdiv0=0x{:x}", self.prdiv0()))}
        if self.pllsten0() != 0 { try!(write!(f, " pllsten0"))}
        if self.pllclken0() != 0 { try!(write!(f, " pllclken0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MCG Control 6 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C6(pub u8);
impl C6 {
    #[doc="VCO 0 Divider"]
    #[inline] pub fn vdiv0(&self) -> bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if VDIV0 != 0"]
    #[inline] pub fn test_vdiv0(&self) -> bool {
        self.vdiv0() != 0
    }

    #[doc="Sets the VDIV0 field."]
    #[inline] pub fn set_vdiv0<V: Into<bits::U5>>(mut self, value: V) -> Self {
        let value: bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Monitor Enable"]
    #[inline] pub fn cme0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CME0 != 0"]
    #[inline] pub fn test_cme0(&self) -> bool {
        self.cme0() != 0
    }

    #[doc="Sets the CME0 field."]
    #[inline] pub fn set_cme0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="PLL Select"]
    #[inline] pub fn plls(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PLLS != 0"]
    #[inline] pub fn test_plls(&self) -> bool {
        self.plls() != 0
    }

    #[doc="Sets the PLLS field."]
    #[inline] pub fn set_plls<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Loss of Lock Interrrupt Enable"]
    #[inline] pub fn lolie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LOLIE0 != 0"]
    #[inline] pub fn test_lolie0(&self) -> bool {
        self.lolie0() != 0
    }

    #[doc="Sets the LOLIE0 field."]
    #[inline] pub fn set_lolie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for C6 {
    #[inline]
    fn from(other: u8) -> Self {
         C6(other)
    }
}

impl ::core::fmt::Display for C6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.vdiv0() != 0 { try!(write!(f, " vdiv0=0x{:x}", self.vdiv0()))}
        if self.cme0() != 0 { try!(write!(f, " cme0"))}
        if self.plls() != 0 { try!(write!(f, " plls"))}
        if self.lolie0() != 0 { try!(write!(f, " lolie0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MCG Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct S(pub u8);
impl S {
    #[doc="Internal Reference Clock Status"]
    #[inline] pub fn ircst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IRCST != 0"]
    #[inline] pub fn test_ircst(&self) -> bool {
        self.ircst() != 0
    }

    #[doc="Sets the IRCST field."]
    #[inline] pub fn set_ircst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="OSC Initialization"]
    #[inline] pub fn oscinit0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if OSCINIT0 != 0"]
    #[inline] pub fn test_oscinit0(&self) -> bool {
        self.oscinit0() != 0
    }

    #[doc="Sets the OSCINIT0 field."]
    #[inline] pub fn set_oscinit0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Clock Mode Status"]
    #[inline] pub fn clkst(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if CLKST != 0"]
    #[inline] pub fn test_clkst(&self) -> bool {
        self.clkst() != 0
    }

    #[doc="Sets the CLKST field."]
    #[inline] pub fn set_clkst<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Internal Reference Status"]
    #[inline] pub fn irefst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IREFST != 0"]
    #[inline] pub fn test_irefst(&self) -> bool {
        self.irefst() != 0
    }

    #[doc="Sets the IREFST field."]
    #[inline] pub fn set_irefst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="PLL Select Status"]
    #[inline] pub fn pllst(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PLLST != 0"]
    #[inline] pub fn test_pllst(&self) -> bool {
        self.pllst() != 0
    }

    #[doc="Sets the PLLST field."]
    #[inline] pub fn set_pllst<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Lock Status"]
    #[inline] pub fn lock0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LOCK0 != 0"]
    #[inline] pub fn test_lock0(&self) -> bool {
        self.lock0() != 0
    }

    #[doc="Sets the LOCK0 field."]
    #[inline] pub fn set_lock0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Loss of Lock Status"]
    #[inline] pub fn lols0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LOLS0 != 0"]
    #[inline] pub fn test_lols0(&self) -> bool {
        self.lols0() != 0
    }

    #[doc="Sets the LOLS0 field."]
    #[inline] pub fn set_lols0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.ircst() != 0 { try!(write!(f, " ircst"))}
        if self.oscinit0() != 0 { try!(write!(f, " oscinit0"))}
        if self.clkst() != 0 { try!(write!(f, " clkst=0x{:x}", self.clkst()))}
        if self.irefst() != 0 { try!(write!(f, " irefst"))}
        if self.pllst() != 0 { try!(write!(f, " pllst"))}
        if self.lock0() != 0 { try!(write!(f, " lock0"))}
        if self.lols0() != 0 { try!(write!(f, " lols0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MCG Status and Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc(pub u8);
impl Sc {
    #[doc="OSC0 Loss of Clock Status"]
    #[inline] pub fn locs0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LOCS0 != 0"]
    #[inline] pub fn test_locs0(&self) -> bool {
        self.locs0() != 0
    }

    #[doc="Sets the LOCS0 field."]
    #[inline] pub fn set_locs0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fast Clock Internal Reference Divider"]
    #[inline] pub fn fcrdiv(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x7) as u8) } // [3:1]
    }

    #[doc="Returns true if FCRDIV != 0"]
    #[inline] pub fn test_fcrdiv(&self) -> bool {
        self.fcrdiv() != 0
    }

    #[doc="Sets the FCRDIV field."]
    #[inline] pub fn set_fcrdiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FLL Filter Preserve Enable"]
    #[inline] pub fn fltprsrv(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FLTPRSRV != 0"]
    #[inline] pub fn test_fltprsrv(&self) -> bool {
        self.fltprsrv() != 0
    }

    #[doc="Sets the FLTPRSRV field."]
    #[inline] pub fn set_fltprsrv<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Automatic Trim Machine Fail Flag"]
    #[inline] pub fn atmf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ATMF != 0"]
    #[inline] pub fn test_atmf(&self) -> bool {
        self.atmf() != 0
    }

    #[doc="Sets the ATMF field."]
    #[inline] pub fn set_atmf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Automatic Trim Machine Select"]
    #[inline] pub fn atms(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ATMS != 0"]
    #[inline] pub fn test_atms(&self) -> bool {
        self.atms() != 0
    }

    #[doc="Sets the ATMS field."]
    #[inline] pub fn set_atms<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Automatic Trim Machine Enable"]
    #[inline] pub fn atme(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ATME != 0"]
    #[inline] pub fn test_atme(&self) -> bool {
        self.atme() != 0
    }

    #[doc="Sets the ATME field."]
    #[inline] pub fn set_atme<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.locs0() != 0 { try!(write!(f, " locs0"))}
        if self.fcrdiv() != 0 { try!(write!(f, " fcrdiv=0x{:x}", self.fcrdiv()))}
        if self.fltprsrv() != 0 { try!(write!(f, " fltprsrv"))}
        if self.atmf() != 0 { try!(write!(f, " atmf"))}
        if self.atms() != 0 { try!(write!(f, " atms"))}
        if self.atme() != 0 { try!(write!(f, " atme"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MCG Auto Trim Compare Value High Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Atcvh(pub u8);
impl Atcvh {
    #[doc="ATM Compare Value High"]
    #[inline] pub fn atcvh(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ATCVH != 0"]
    #[inline] pub fn test_atcvh(&self) -> bool {
        self.atcvh() != 0
    }

    #[doc="Sets the ATCVH field."]
    #[inline] pub fn set_atcvh<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Atcvh {
    #[inline]
    fn from(other: u8) -> Self {
         Atcvh(other)
    }
}

impl ::core::fmt::Display for Atcvh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Atcvh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.atcvh() != 0 { try!(write!(f, " atcvh=0x{:x}", self.atcvh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MCG Auto Trim Compare Value Low Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Atcvl(pub u8);
impl Atcvl {
    #[doc="ATM Compare Value Low"]
    #[inline] pub fn atcvl(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if ATCVL != 0"]
    #[inline] pub fn test_atcvl(&self) -> bool {
        self.atcvl() != 0
    }

    #[doc="Sets the ATCVL field."]
    #[inline] pub fn set_atcvl<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Atcvl {
    #[inline]
    fn from(other: u8) -> Self {
         Atcvl(other)
    }
}

impl ::core::fmt::Display for Atcvl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Atcvl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.atcvl() != 0 { try!(write!(f, " atcvl=0x{:x}", self.atcvl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MCG Control 7 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C7(pub u8);
impl C7 {
    #[doc="MCG OSC Clock Select"]
    #[inline] pub fn oscsel(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if OSCSEL != 0"]
    #[inline] pub fn test_oscsel(&self) -> bool {
        self.oscsel() != 0
    }

    #[doc="Sets the OSCSEL field."]
    #[inline] pub fn set_oscsel<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for C7 {
    #[inline]
    fn from(other: u8) -> Self {
         C7(other)
    }
}

impl ::core::fmt::Display for C7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.oscsel() != 0 { try!(write!(f, " oscsel=0x{:x}", self.oscsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="MCG Control 8 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C8(pub u8);
impl C8 {
    #[doc="RTC Loss of Clock Status"]
    #[inline] pub fn locs1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LOCS1 != 0"]
    #[inline] pub fn test_locs1(&self) -> bool {
        self.locs1() != 0
    }

    #[doc="Sets the LOCS1 field."]
    #[inline] pub fn set_locs1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Monitor Enable1"]
    #[inline] pub fn cme1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CME1 != 0"]
    #[inline] pub fn test_cme1(&self) -> bool {
        self.cme1() != 0
    }

    #[doc="Sets the CME1 field."]
    #[inline] pub fn set_cme1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="PLL Loss of Lock Reset Enable"]
    #[inline] pub fn lolre(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LOLRE != 0"]
    #[inline] pub fn test_lolre(&self) -> bool {
        self.lolre() != 0
    }

    #[doc="Sets the LOLRE field."]
    #[inline] pub fn set_lolre<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Loss of Clock Reset Enable"]
    #[inline] pub fn locre1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LOCRE1 != 0"]
    #[inline] pub fn test_locre1(&self) -> bool {
        self.locre1() != 0
    }

    #[doc="Sets the LOCRE1 field."]
    #[inline] pub fn set_locre1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for C8 {
    #[inline]
    fn from(other: u8) -> Self {
         C8(other)
    }
}

impl ::core::fmt::Display for C8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for C8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.locs1() != 0 { try!(write!(f, " locs1"))}
        if self.cme1() != 0 { try!(write!(f, " cme1"))}
        if self.lolre() != 0 { try!(write!(f, " lolre"))}
        if self.locre1() != 0 { try!(write!(f, " locre1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

