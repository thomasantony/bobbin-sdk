
#[allow(unused_imports)] use bobbin_common::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="IWDG Peripheral"]
pub struct IwdgPeriph(pub usize); 

impl IwdgPeriph {
    #[doc="Get the *mut pointer for the KR register."]
    #[inline] pub fn kr_mut(&self) -> *mut Kr { 
        (self.0 + 0x0) as *mut Kr
    }

    #[doc="Get the *const pointer for the KR register."]
    #[inline] pub fn kr_ptr(&self) -> *const Kr { 
           self.kr_mut()
    }

    #[doc="Write the KR register."]
    #[inline] pub fn set_kr<F: FnOnce(Kr) -> Kr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.kr_mut(), f(Kr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the PR register."]
    #[inline] pub fn pr_mut(&self) -> *mut Pr { 
        (self.0 + 0x4) as *mut Pr
    }

    #[doc="Get the *const pointer for the PR register."]
    #[inline] pub fn pr_ptr(&self) -> *const Pr { 
           self.pr_mut()
    }

    #[doc="Read the PR register."]
    #[inline] pub fn pr(&self) -> Pr { 
        unsafe {
            read_volatile(self.pr_ptr())
        }
    }

    #[doc="Write the PR register."]
    #[inline] pub fn set_pr<F: FnOnce(Pr) -> Pr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pr_mut(), f(Pr(0)));
        }
        self
    }

    #[doc="Modify the PR register."]
    #[inline] pub fn with_pr<F: FnOnce(Pr) -> Pr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pr_mut(), f(self.pr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RLR register."]
    #[inline] pub fn rlr_mut(&self) -> *mut Rlr { 
        (self.0 + 0x8) as *mut Rlr
    }

    #[doc="Get the *const pointer for the RLR register."]
    #[inline] pub fn rlr_ptr(&self) -> *const Rlr { 
           self.rlr_mut()
    }

    #[doc="Read the RLR register."]
    #[inline] pub fn rlr(&self) -> Rlr { 
        unsafe {
            read_volatile(self.rlr_ptr())
        }
    }

    #[doc="Write the RLR register."]
    #[inline] pub fn set_rlr<F: FnOnce(Rlr) -> Rlr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rlr_mut(), f(Rlr(0)));
        }
        self
    }

    #[doc="Modify the RLR register."]
    #[inline] pub fn with_rlr<F: FnOnce(Rlr) -> Rlr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.rlr_mut(), f(self.rlr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        (self.0 + 0xc) as *mut Sr
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
           self.sr_mut()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        unsafe {
            read_volatile(self.sr_ptr())
        }
    }

    #[doc="Get the *mut pointer for the WINR register."]
    #[inline] pub fn winr_mut(&self) -> *mut Winr { 
        (self.0 + 0x10) as *mut Winr
    }

    #[doc="Get the *const pointer for the WINR register."]
    #[inline] pub fn winr_ptr(&self) -> *const Winr { 
           self.winr_mut()
    }

    #[doc="Read the WINR register."]
    #[inline] pub fn winr(&self) -> Winr { 
        unsafe {
            read_volatile(self.winr_ptr())
        }
    }

    #[doc="Write the WINR register."]
    #[inline] pub fn set_winr<F: FnOnce(Winr) -> Winr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.winr_mut(), f(Winr(0)));
        }
        self
    }

    #[doc="Modify the WINR register."]
    #[inline] pub fn with_winr<F: FnOnce(Winr) -> Winr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.winr_mut(), f(self.winr()));
        }
        self
    }

}

#[doc="Key register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Kr(pub u32);
impl Kr {
    #[doc="Key value (write only, read 0x0000)"]
    #[inline] pub fn key(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if KEY != 0"]
    #[inline] pub fn test_key(&self) -> bool {
        self.key() != 0
    }

    #[doc="Sets the KEY field."]
    #[inline] pub fn set_key<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Kr {
    #[inline]
    fn from(other: u32) -> Self {
         Kr(other)
    }
}

impl ::core::fmt::Display for Kr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Kr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Prescaler register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pr(pub u32);
impl Pr {
    #[doc="Prescaler divider"]
    #[inline] pub fn pr(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if PR != 0"]
    #[inline] pub fn test_pr(&self) -> bool {
        self.pr() != 0
    }

    #[doc="Sets the PR field."]
    #[inline] pub fn set_pr<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pr {
    #[inline]
    fn from(other: u32) -> Self {
         Pr(other)
    }
}

impl ::core::fmt::Display for Pr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pr() != 0 { try!(write!(f, " pr=0x{:x}", self.pr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Reload register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rlr(pub u32);
impl Rlr {
    #[doc="Watchdog counter reload value"]
    #[inline] pub fn rl(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if RL != 0"]
    #[inline] pub fn test_rl(&self) -> bool {
        self.rl() != 0
    }

    #[doc="Sets the RL field."]
    #[inline] pub fn set_rl<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rlr {
    #[inline]
    fn from(other: u32) -> Self {
         Rlr(other)
    }
}

impl ::core::fmt::Display for Rlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rl() != 0 { try!(write!(f, " rl=0x{:x}", self.rl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="Watchdog counter window value update"]
    #[inline] pub fn wvu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WVU != 0"]
    #[inline] pub fn test_wvu(&self) -> bool {
        self.wvu() != 0
    }

    #[doc="Sets the WVU field."]
    #[inline] pub fn set_wvu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Watchdog counter reload value update"]
    #[inline] pub fn rvu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RVU != 0"]
    #[inline] pub fn test_rvu(&self) -> bool {
        self.rvu() != 0
    }

    #[doc="Sets the RVU field."]
    #[inline] pub fn set_rvu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Watchdog prescaler value update"]
    #[inline] pub fn pvu(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PVU != 0"]
    #[inline] pub fn test_pvu(&self) -> bool {
        self.pvu() != 0
    }

    #[doc="Sets the PVU field."]
    #[inline] pub fn set_pvu<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
    }
}

impl ::core::fmt::Display for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.wvu() != 0 { try!(write!(f, " wvu"))}
        if self.rvu() != 0 { try!(write!(f, " rvu"))}
        if self.pvu() != 0 { try!(write!(f, " pvu"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Window register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Winr(pub u32);
impl Winr {
    #[doc="Watchdog counter window value"]
    #[inline] pub fn win(&self) -> bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if WIN != 0"]
    #[inline] pub fn test_win(&self) -> bool {
        self.win() != 0
    }

    #[doc="Sets the WIN field."]
    #[inline] pub fn set_win<V: Into<bits::U12>>(mut self, value: V) -> Self {
        let value: bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Winr {
    #[inline]
    fn from(other: u32) -> Self {
         Winr(other)
    }
}

impl ::core::fmt::Display for Winr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Winr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.win() != 0 { try!(write!(f, " win=0x{:x}", self.win()))}
        try!(write!(f, "]"));
        Ok(())
    }
}
