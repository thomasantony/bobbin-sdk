#[allow(unused_imports)] use bobbin_common::*;

periph!( SSI0, Ssi0, _SSI0, SsiPeriph, 0x40008000);
periph!( SSI1, Ssi1, _SSI1, SsiPeriph, 0x40009000);
periph!( SSI2, Ssi2, _SSI2, SsiPeriph, 0x4000a000);
periph!( SSI3, Ssi3, _SSI3, SsiPeriph, 0x4000b000);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SSI Peripheral"]
pub struct SsiPeriph(pub usize); 

impl super::sig::Signal<super::sig::Ssi0clk> for Ssi0 {}
impl super::sig::SignalSsiClk<super::sig::Ssi0clk> for Ssi0 {}
impl super::sig::Signal<super::sig::Ssi0fss> for Ssi0 {}
impl super::sig::SignalSsiFss<super::sig::Ssi0fss> for Ssi0 {}
impl super::sig::Signal<super::sig::Ssi0dat0> for Ssi0 {}
impl super::sig::SignalSsiDat0<super::sig::Ssi0dat0> for Ssi0 {}
impl super::sig::Signal<super::sig::Ssi0dat1> for Ssi0 {}
impl super::sig::SignalSsiDat1<super::sig::Ssi0dat1> for Ssi0 {}
impl super::sig::Signal<super::sig::Ssi0dat2> for Ssi0 {}
impl super::sig::SignalSsiDat2<super::sig::Ssi0dat2> for Ssi0 {}
impl super::sig::Signal<super::sig::Ssi0dat3> for Ssi0 {}
impl super::sig::SignalSsiDat3<super::sig::Ssi0dat3> for Ssi0 {}

impl super::sig::Signal<super::sig::Ssi1clk> for Ssi1 {}
impl super::sig::SignalSsiClk<super::sig::Ssi1clk> for Ssi1 {}
impl super::sig::Signal<super::sig::Ssi1fss> for Ssi1 {}
impl super::sig::SignalSsiFss<super::sig::Ssi1fss> for Ssi1 {}
impl super::sig::Signal<super::sig::Ssi1dat0> for Ssi1 {}
impl super::sig::SignalSsiDat0<super::sig::Ssi1dat0> for Ssi1 {}
impl super::sig::Signal<super::sig::Ssi1dat1> for Ssi1 {}
impl super::sig::SignalSsiDat1<super::sig::Ssi1dat1> for Ssi1 {}
impl super::sig::Signal<super::sig::Ssi1dat2> for Ssi1 {}
impl super::sig::SignalSsiDat2<super::sig::Ssi1dat2> for Ssi1 {}
impl super::sig::Signal<super::sig::Ssi1dat3> for Ssi1 {}
impl super::sig::SignalSsiDat3<super::sig::Ssi1dat3> for Ssi1 {}

impl super::sig::Signal<super::sig::Ssi2clk> for Ssi2 {}
impl super::sig::SignalSsiClk<super::sig::Ssi2clk> for Ssi2 {}
impl super::sig::Signal<super::sig::Ssi2fss> for Ssi2 {}
impl super::sig::SignalSsiFss<super::sig::Ssi2fss> for Ssi2 {}
impl super::sig::Signal<super::sig::Ssi2xdat0> for Ssi2 {}
impl super::sig::SignalSsiDat0<super::sig::Ssi2xdat0> for Ssi2 {}
impl super::sig::Signal<super::sig::Ssi2xdat1> for Ssi2 {}
impl super::sig::SignalSsiDat1<super::sig::Ssi2xdat1> for Ssi2 {}
impl super::sig::Signal<super::sig::Ssi2xdat2> for Ssi2 {}
impl super::sig::SignalSsiDat2<super::sig::Ssi2xdat2> for Ssi2 {}
impl super::sig::Signal<super::sig::Ssi2xdat3> for Ssi2 {}
impl super::sig::SignalSsiDat3<super::sig::Ssi2xdat3> for Ssi2 {}

impl super::sig::Signal<super::sig::Ssi3clk> for Ssi3 {}
impl super::sig::SignalSsiClk<super::sig::Ssi3clk> for Ssi3 {}
impl super::sig::Signal<super::sig::Ssi3fss> for Ssi3 {}
impl super::sig::SignalSsiFss<super::sig::Ssi3fss> for Ssi3 {}
impl super::sig::Signal<super::sig::Ssi3xdat0> for Ssi3 {}
impl super::sig::SignalSsiDat0<super::sig::Ssi3xdat0> for Ssi3 {}
impl super::sig::Signal<super::sig::Ssi3xdat1> for Ssi3 {}
impl super::sig::SignalSsiDat1<super::sig::Ssi3xdat1> for Ssi3 {}
impl super::sig::Signal<super::sig::Ssi3xdat2> for Ssi3 {}
impl super::sig::SignalSsiDat2<super::sig::Ssi3xdat2> for Ssi3 {}
impl super::sig::Signal<super::sig::Ssi3xdat3> for Ssi3 {}
impl super::sig::SignalSsiDat3<super::sig::Ssi3xdat3> for Ssi3 {}


impl SsiPeriph {
    #[doc="Get the *mut pointer for the CR0 register."]
    #[inline] pub fn cr0_mut(&self) -> *mut Cr0 { 
        (self.0 + 0x0) as *mut Cr0
    }

    #[doc="Get the *const pointer for the CR0 register."]
    #[inline] pub fn cr0_ptr(&self) -> *const Cr0 { 
           self.cr0_mut()
    }

    #[doc="Read the CR0 register."]
    #[inline] pub fn cr0(&self) -> Cr0 { 
        unsafe {
            read_volatile(self.cr0_ptr())
        }
    }

    #[doc="Write the CR0 register."]
    #[inline] pub fn set_cr0<F: FnOnce(Cr0) -> Cr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr0_mut(), f(Cr0(0)));
        }
        self
    }

    #[doc="Modify the CR0 register."]
    #[inline] pub fn with_cr0<F: FnOnce(Cr0) -> Cr0>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr0_mut(), f(self.cr0()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CR1 register."]
    #[inline] pub fn cr1_mut(&self) -> *mut Cr1 { 
        (self.0 + 0x4) as *mut Cr1
    }

    #[doc="Get the *const pointer for the CR1 register."]
    #[inline] pub fn cr1_ptr(&self) -> *const Cr1 { 
           self.cr1_mut()
    }

    #[doc="Read the CR1 register."]
    #[inline] pub fn cr1(&self) -> Cr1 { 
        unsafe {
            read_volatile(self.cr1_ptr())
        }
    }

    #[doc="Write the CR1 register."]
    #[inline] pub fn set_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr1_mut(), f(Cr1(0)));
        }
        self
    }

    #[doc="Modify the CR1 register."]
    #[inline] pub fn with_cr1<F: FnOnce(Cr1) -> Cr1>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cr1_mut(), f(self.cr1()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DR register."]
    #[inline] pub fn dr_mut(&self) -> *mut Dr { 
        (self.0 + 0x8) as *mut Dr
    }

    #[doc="Get the *const pointer for the DR register."]
    #[inline] pub fn dr_ptr(&self) -> *const Dr { 
           self.dr_mut()
    }

    #[doc="Read the DR register."]
    #[inline] pub fn dr(&self) -> Dr { 
        unsafe {
            read_volatile(self.dr_ptr())
        }
    }

    #[doc="Write the DR register."]
    #[inline] pub fn set_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dr_mut(), f(Dr(0)));
        }
        self
    }

    #[doc="Modify the DR register."]
    #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dr_mut(), f(self.dr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the DR8 register."]
    #[inline] pub fn dr8_mut(&self) -> *mut Dr8 { 
        (self.0 + 0x8) as *mut Dr8
    }

    #[doc="Get the *const pointer for the DR8 register."]
    #[inline] pub fn dr8_ptr(&self) -> *const Dr8 { 
           self.dr8_mut()
    }

    #[doc="Read the DR8 register."]
    #[inline] pub fn dr8(&self) -> Dr8 { 
        unsafe {
            read_volatile(self.dr8_ptr())
        }
    }

    #[doc="Write the DR8 register."]
    #[inline] pub fn set_dr8<F: FnOnce(Dr8) -> Dr8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dr8_mut(), f(Dr8(0)));
        }
        self
    }

    #[doc="Modify the DR8 register."]
    #[inline] pub fn with_dr8<F: FnOnce(Dr8) -> Dr8>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dr8_mut(), f(self.dr8()));
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

    #[doc="Write the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sr_mut(), f(Sr(0)));
        }
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.sr_mut(), f(self.sr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CPSR register."]
    #[inline] pub fn cpsr_mut(&self) -> *mut Cpsr { 
        (self.0 + 0x10) as *mut Cpsr
    }

    #[doc="Get the *const pointer for the CPSR register."]
    #[inline] pub fn cpsr_ptr(&self) -> *const Cpsr { 
           self.cpsr_mut()
    }

    #[doc="Read the CPSR register."]
    #[inline] pub fn cpsr(&self) -> Cpsr { 
        unsafe {
            read_volatile(self.cpsr_ptr())
        }
    }

    #[doc="Write the CPSR register."]
    #[inline] pub fn set_cpsr<F: FnOnce(Cpsr) -> Cpsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cpsr_mut(), f(Cpsr(0)));
        }
        self
    }

    #[doc="Modify the CPSR register."]
    #[inline] pub fn with_cpsr<F: FnOnce(Cpsr) -> Cpsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cpsr_mut(), f(self.cpsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the IM register."]
    #[inline] pub fn im_mut(&self) -> *mut Im { 
        (self.0 + 0x14) as *mut Im
    }

    #[doc="Get the *const pointer for the IM register."]
    #[inline] pub fn im_ptr(&self) -> *const Im { 
           self.im_mut()
    }

    #[doc="Read the IM register."]
    #[inline] pub fn im(&self) -> Im { 
        unsafe {
            read_volatile(self.im_ptr())
        }
    }

    #[doc="Write the IM register."]
    #[inline] pub fn set_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.im_mut(), f(Im(0)));
        }
        self
    }

    #[doc="Modify the IM register."]
    #[inline] pub fn with_im<F: FnOnce(Im) -> Im>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.im_mut(), f(self.im()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RIS register."]
    #[inline] pub fn ris_mut(&self) -> *mut Ris { 
        (self.0 + 0x18) as *mut Ris
    }

    #[doc="Get the *const pointer for the RIS register."]
    #[inline] pub fn ris_ptr(&self) -> *const Ris { 
           self.ris_mut()
    }

    #[doc="Read the RIS register."]
    #[inline] pub fn ris(&self) -> Ris { 
        unsafe {
            read_volatile(self.ris_ptr())
        }
    }

    #[doc="Write the RIS register."]
    #[inline] pub fn set_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ris_mut(), f(Ris(0)));
        }
        self
    }

    #[doc="Modify the RIS register."]
    #[inline] pub fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ris_mut(), f(self.ris()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MIS register."]
    #[inline] pub fn mis_mut(&self) -> *mut Mis { 
        (self.0 + 0x1c) as *mut Mis
    }

    #[doc="Get the *const pointer for the MIS register."]
    #[inline] pub fn mis_ptr(&self) -> *const Mis { 
           self.mis_mut()
    }

    #[doc="Read the MIS register."]
    #[inline] pub fn mis(&self) -> Mis { 
        unsafe {
            read_volatile(self.mis_ptr())
        }
    }

    #[doc="Write the MIS register."]
    #[inline] pub fn set_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mis_mut(), f(Mis(0)));
        }
        self
    }

    #[doc="Modify the MIS register."]
    #[inline] pub fn with_mis<F: FnOnce(Mis) -> Mis>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mis_mut(), f(self.mis()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ICR register."]
    #[inline] pub fn icr_mut(&self) -> *mut Icr { 
        (self.0 + 0x20) as *mut Icr
    }

    #[doc="Get the *const pointer for the ICR register."]
    #[inline] pub fn icr_ptr(&self) -> *const Icr { 
           self.icr_mut()
    }

    #[doc="Write the ICR register."]
    #[inline] pub fn set_icr<F: FnOnce(Icr) -> Icr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.icr_mut(), f(Icr(0)));
        }
        self
    }

    #[doc="Get the *mut pointer for the DMACTL register."]
    #[inline] pub fn dmactl_mut(&self) -> *mut Dmactl { 
        (self.0 + 0x24) as *mut Dmactl
    }

    #[doc="Get the *const pointer for the DMACTL register."]
    #[inline] pub fn dmactl_ptr(&self) -> *const Dmactl { 
           self.dmactl_mut()
    }

    #[doc="Read the DMACTL register."]
    #[inline] pub fn dmactl(&self) -> Dmactl { 
        unsafe {
            read_volatile(self.dmactl_ptr())
        }
    }

    #[doc="Write the DMACTL register."]
    #[inline] pub fn set_dmactl<F: FnOnce(Dmactl) -> Dmactl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmactl_mut(), f(Dmactl(0)));
        }
        self
    }

    #[doc="Modify the DMACTL register."]
    #[inline] pub fn with_dmactl<F: FnOnce(Dmactl) -> Dmactl>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.dmactl_mut(), f(self.dmactl()));
        }
        self
    }

    #[doc="Get the *mut pointer for the PP register."]
    #[inline] pub fn pp_mut(&self) -> *mut Pp { 
        (self.0 + 0xfc0) as *mut Pp
    }

    #[doc="Get the *const pointer for the PP register."]
    #[inline] pub fn pp_ptr(&self) -> *const Pp { 
           self.pp_mut()
    }

    #[doc="Read the PP register."]
    #[inline] pub fn pp(&self) -> Pp { 
        unsafe {
            read_volatile(self.pp_ptr())
        }
    }

    #[doc="Write the PP register."]
    #[inline] pub fn set_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pp_mut(), f(Pp(0)));
        }
        self
    }

    #[doc="Modify the PP register."]
    #[inline] pub fn with_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.pp_mut(), f(self.pp()));
        }
        self
    }

    #[doc="Get the *mut pointer for the CC register."]
    #[inline] pub fn cc_mut(&self) -> *mut Cc { 
        (self.0 + 0xfc8) as *mut Cc
    }

    #[doc="Get the *const pointer for the CC register."]
    #[inline] pub fn cc_ptr(&self) -> *const Cc { 
           self.cc_mut()
    }

    #[doc="Read the CC register."]
    #[inline] pub fn cc(&self) -> Cc { 
        unsafe {
            read_volatile(self.cc_ptr())
        }
    }

    #[doc="Write the CC register."]
    #[inline] pub fn set_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(), f(Cc(0)));
        }
        self
    }

    #[doc="Modify the CC register."]
    #[inline] pub fn with_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.cc_mut(), f(self.cc()));
        }
        self
    }

}

#[doc="SSI Control 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr0(pub u32);
impl Cr0 {
    #[doc="SSI Data Size Select"]
    #[inline] pub fn dss(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DSS != 0"]
    #[inline] pub fn test_dss(&self) -> bool {
        self.dss() != 0
    }

    #[doc="Sets the DSS field."]
    #[inline] pub fn set_dss<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Frame Format Select"]
    #[inline] pub fn frf(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if FRF != 0"]
    #[inline] pub fn test_frf(&self) -> bool {
        self.frf() != 0
    }

    #[doc="Sets the FRF field."]
    #[inline] pub fn set_frf<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SSI Serial Clock Polarity"]
    #[inline] pub fn spo(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SPO != 0"]
    #[inline] pub fn test_spo(&self) -> bool {
        self.spo() != 0
    }

    #[doc="Sets the SPO field."]
    #[inline] pub fn set_spo<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SSI Serial Clock Phase"]
    #[inline] pub fn sph(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SPH != 0"]
    #[inline] pub fn test_sph(&self) -> bool {
        self.sph() != 0
    }

    #[doc="Sets the SPH field."]
    #[inline] pub fn set_sph<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="SSI Serial Clock Rate"]
    #[inline] pub fn scr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if SCR != 0"]
    #[inline] pub fn test_scr(&self) -> bool {
        self.scr() != 0
    }

    #[doc="Sets the SCR field."]
    #[inline] pub fn set_scr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Cr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr0(other)
    }
}

impl ::core::fmt::Display for Cr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dss() != 0 { try!(write!(f, " dss=0x{:x}", self.dss()))}
        if self.frf() != 0 { try!(write!(f, " frf=0x{:x}", self.frf()))}
        if self.spo() != 0 { try!(write!(f, " spo"))}
        if self.sph() != 0 { try!(write!(f, " sph"))}
        if self.scr() != 0 { try!(write!(f, " scr=0x{:x}", self.scr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SSI Control 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc="SSI Loopback Mode"]
    #[inline] pub fn lbm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if LBM != 0"]
    #[inline] pub fn test_lbm(&self) -> bool {
        self.lbm() != 0
    }

    #[doc="Sets the LBM field."]
    #[inline] pub fn set_lbm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Synchronous Serial Port Enable"]
    #[inline] pub fn sse(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SSE != 0"]
    #[inline] pub fn test_sse(&self) -> bool {
        self.sse() != 0
    }

    #[doc="Sets the SSE field."]
    #[inline] pub fn set_sse<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Master/Slave Select"]
    #[inline] pub fn ms(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if MS != 0"]
    #[inline] pub fn test_ms(&self) -> bool {
        self.ms() != 0
    }

    #[doc="Sets the MS field."]
    #[inline] pub fn set_ms<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="End of Transmission"]
    #[inline] pub fn eot(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if EOT != 0"]
    #[inline] pub fn test_eot(&self) -> bool {
        self.eot() != 0
    }

    #[doc="Sets the EOT field."]
    #[inline] pub fn set_eot<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SSI Mode"]
    #[inline] pub fn mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="SSI Direction of Operation"]
    #[inline] pub fn dir(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DIR != 0"]
    #[inline] pub fn test_dir(&self) -> bool {
        self.dir() != 0
    }

    #[doc="Sets the DIR field."]
    #[inline] pub fn set_dir<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="High Speed Clock Enable"]
    #[inline] pub fn hsclken(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if HSCLKEN != 0"]
    #[inline] pub fn test_hsclken(&self) -> bool {
        self.hsclken() != 0
    }

    #[doc="Sets the HSCLKEN field."]
    #[inline] pub fn set_hsclken<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="FSS Hold Frame"]
    #[inline] pub fn fsshldfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if FSSHLDFRM != 0"]
    #[inline] pub fn test_fsshldfrm(&self) -> bool {
        self.fsshldfrm() != 0
    }

    #[doc="Sets the FSSHLDFRM field."]
    #[inline] pub fn set_fsshldfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Stop Frame (End of Message)"]
    #[inline] pub fn eom(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if EOM != 0"]
    #[inline] pub fn test_eom(&self) -> bool {
        self.eom() != 0
    }

    #[doc="Sets the EOM field."]
    #[inline] pub fn set_eom<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

}

impl From<u32> for Cr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Cr1(other)
    }
}

impl ::core::fmt::Display for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lbm() != 0 { try!(write!(f, " lbm"))}
        if self.sse() != 0 { try!(write!(f, " sse"))}
        if self.ms() != 0 { try!(write!(f, " ms"))}
        if self.eot() != 0 { try!(write!(f, " eot"))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.dir() != 0 { try!(write!(f, " dir"))}
        if self.hsclken() != 0 { try!(write!(f, " hsclken"))}
        if self.fsshldfrm() != 0 { try!(write!(f, " fsshldfrm"))}
        if self.eom() != 0 { try!(write!(f, " eom"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SSI Data"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
    #[doc="SSI Receive/Transmit Data"]
    #[inline] pub fn data(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dr {
    #[inline]
    fn from(other: u32) -> Self {
         Dr(other)
    }
}

impl ::core::fmt::Display for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SSI Data (8 bit)"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dr8(pub u32);
impl Dr8 {
    #[doc="SSI Receive/Transmit Data (8 bit)"]
    #[inline] pub fn data(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA != 0"]
    #[inline] pub fn test_data(&self) -> bool {
        self.data() != 0
    }

    #[doc="Sets the DATA field."]
    #[inline] pub fn set_data<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dr8 {
    #[inline]
    fn from(other: u32) -> Self {
         Dr8(other)
    }
}

impl ::core::fmt::Display for Dr8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dr8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data() != 0 { try!(write!(f, " data=0x{:x}", self.data()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SSI Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="SSI Transmit FIFO Empty"]
    #[inline] pub fn tfe(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TFE != 0"]
    #[inline] pub fn test_tfe(&self) -> bool {
        self.tfe() != 0
    }

    #[doc="Sets the TFE field."]
    #[inline] pub fn set_tfe<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Transmit FIFO Not Full"]
    #[inline] pub fn tnf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TNF != 0"]
    #[inline] pub fn test_tnf(&self) -> bool {
        self.tnf() != 0
    }

    #[doc="Sets the TNF field."]
    #[inline] pub fn set_tnf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Receive FIFO Not Empty"]
    #[inline] pub fn rne(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RNE != 0"]
    #[inline] pub fn test_rne(&self) -> bool {
        self.rne() != 0
    }

    #[doc="Sets the RNE field."]
    #[inline] pub fn set_rne<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SSI Receive FIFO Full"]
    #[inline] pub fn rff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RFF != 0"]
    #[inline] pub fn test_rff(&self) -> bool {
        self.rff() != 0
    }

    #[doc="Sets the RFF field."]
    #[inline] pub fn set_rff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="SSI Busy Bit"]
    #[inline] pub fn bsy(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BSY != 0"]
    #[inline] pub fn test_bsy(&self) -> bool {
        self.bsy() != 0
    }

    #[doc="Sets the BSY field."]
    #[inline] pub fn set_bsy<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
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
        if self.tfe() != 0 { try!(write!(f, " tfe"))}
        if self.tnf() != 0 { try!(write!(f, " tnf"))}
        if self.rne() != 0 { try!(write!(f, " rne"))}
        if self.rff() != 0 { try!(write!(f, " rff"))}
        if self.bsy() != 0 { try!(write!(f, " bsy"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SSI Clock Prescale"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cpsr(pub u32);
impl Cpsr {
    #[doc="SSI Clock Prescale Divisor"]
    #[inline] pub fn cpsdvsr(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CPSDVSR != 0"]
    #[inline] pub fn test_cpsdvsr(&self) -> bool {
        self.cpsdvsr() != 0
    }

    #[doc="Sets the CPSDVSR field."]
    #[inline] pub fn set_cpsdvsr<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cpsr {
    #[inline]
    fn from(other: u32) -> Self {
         Cpsr(other)
    }
}

impl ::core::fmt::Display for Cpsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cpsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cpsdvsr() != 0 { try!(write!(f, " cpsdvsr=0x{:x}", self.cpsdvsr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SSI Interrupt Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Im(pub u32);
impl Im {
    #[doc="SSI Receive Overrun Interrupt Mask"]
    #[inline] pub fn rorim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RORIM != 0"]
    #[inline] pub fn test_rorim(&self) -> bool {
        self.rorim() != 0
    }

    #[doc="Sets the RORIM field."]
    #[inline] pub fn set_rorim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Receive Time-Out Interrupt Mask"]
    #[inline] pub fn rtim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RTIM != 0"]
    #[inline] pub fn test_rtim(&self) -> bool {
        self.rtim() != 0
    }

    #[doc="Sets the RTIM field."]
    #[inline] pub fn set_rtim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Receive FIFO Interrupt Mask"]
    #[inline] pub fn rxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXIM != 0"]
    #[inline] pub fn test_rxim(&self) -> bool {
        self.rxim() != 0
    }

    #[doc="Sets the RXIM field."]
    #[inline] pub fn set_rxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SSI Transmit FIFO Interrupt Mask"]
    #[inline] pub fn txim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TXIM != 0"]
    #[inline] pub fn test_txim(&self) -> bool {
        self.txim() != 0
    }

    #[doc="Sets the TXIM field."]
    #[inline] pub fn set_txim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="SSI Receive DMA Interrupt Mask"]
    #[inline] pub fn dmarxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DMARXIM != 0"]
    #[inline] pub fn test_dmarxim(&self) -> bool {
        self.dmarxim() != 0
    }

    #[doc="Sets the DMARXIM field."]
    #[inline] pub fn set_dmarxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SSI Transmit DMA Interrupt Mask"]
    #[inline] pub fn dmatxim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMATXIM != 0"]
    #[inline] pub fn test_dmatxim(&self) -> bool {
        self.dmatxim() != 0
    }

    #[doc="Sets the DMATXIM field."]
    #[inline] pub fn set_dmatxim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="End of Transmit Interrupt Mask"]
    #[inline] pub fn eotim(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if EOTIM != 0"]
    #[inline] pub fn test_eotim(&self) -> bool {
        self.eotim() != 0
    }

    #[doc="Sets the EOTIM field."]
    #[inline] pub fn set_eotim<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Im {
    #[inline]
    fn from(other: u32) -> Self {
         Im(other)
    }
}

impl ::core::fmt::Display for Im {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Im {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rorim() != 0 { try!(write!(f, " rorim"))}
        if self.rtim() != 0 { try!(write!(f, " rtim"))}
        if self.rxim() != 0 { try!(write!(f, " rxim"))}
        if self.txim() != 0 { try!(write!(f, " txim"))}
        if self.dmarxim() != 0 { try!(write!(f, " dmarxim"))}
        if self.dmatxim() != 0 { try!(write!(f, " dmatxim"))}
        if self.eotim() != 0 { try!(write!(f, " eotim"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SSI Raw Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
    #[doc="SSI Receive Overrun Raw Interrupt Status"]
    #[inline] pub fn rorris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RORRIS != 0"]
    #[inline] pub fn test_rorris(&self) -> bool {
        self.rorris() != 0
    }

    #[doc="Sets the RORRIS field."]
    #[inline] pub fn set_rorris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Receive Time-Out Raw Interrupt Status"]
    #[inline] pub fn rtris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RTRIS != 0"]
    #[inline] pub fn test_rtris(&self) -> bool {
        self.rtris() != 0
    }

    #[doc="Sets the RTRIS field."]
    #[inline] pub fn set_rtris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Receive FIFO Raw Interrupt Status"]
    #[inline] pub fn rxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXRIS != 0"]
    #[inline] pub fn test_rxris(&self) -> bool {
        self.rxris() != 0
    }

    #[doc="Sets the RXRIS field."]
    #[inline] pub fn set_rxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SSI Transmit FIFO Raw Interrupt Status"]
    #[inline] pub fn txris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TXRIS != 0"]
    #[inline] pub fn test_txris(&self) -> bool {
        self.txris() != 0
    }

    #[doc="Sets the TXRIS field."]
    #[inline] pub fn set_txris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="SSI Receive DMA Raw Interrupt Status"]
    #[inline] pub fn dmarxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DMARXRIS != 0"]
    #[inline] pub fn test_dmarxris(&self) -> bool {
        self.dmarxris() != 0
    }

    #[doc="Sets the DMARXRIS field."]
    #[inline] pub fn set_dmarxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SSI Transmit DMA Raw Interrupt Status"]
    #[inline] pub fn dmatxris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMATXRIS != 0"]
    #[inline] pub fn test_dmatxris(&self) -> bool {
        self.dmatxris() != 0
    }

    #[doc="Sets the DMATXRIS field."]
    #[inline] pub fn set_dmatxris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="End of Transmit Raw Interrupt Status"]
    #[inline] pub fn eotris(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if EOTRIS != 0"]
    #[inline] pub fn test_eotris(&self) -> bool {
        self.eotris() != 0
    }

    #[doc="Sets the EOTRIS field."]
    #[inline] pub fn set_eotris<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Ris {
    #[inline]
    fn from(other: u32) -> Self {
         Ris(other)
    }
}

impl ::core::fmt::Display for Ris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ris {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rorris() != 0 { try!(write!(f, " rorris"))}
        if self.rtris() != 0 { try!(write!(f, " rtris"))}
        if self.rxris() != 0 { try!(write!(f, " rxris"))}
        if self.txris() != 0 { try!(write!(f, " txris"))}
        if self.dmarxris() != 0 { try!(write!(f, " dmarxris"))}
        if self.dmatxris() != 0 { try!(write!(f, " dmatxris"))}
        if self.eotris() != 0 { try!(write!(f, " eotris"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SSI Masked Interrupt Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mis(pub u32);
impl Mis {
    #[doc="SSI Receive Overrun Masked Interrupt Status"]
    #[inline] pub fn rormis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RORMIS != 0"]
    #[inline] pub fn test_rormis(&self) -> bool {
        self.rormis() != 0
    }

    #[doc="Sets the RORMIS field."]
    #[inline] pub fn set_rormis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Receive Time-Out Masked Interrupt Status"]
    #[inline] pub fn rtmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RTMIS != 0"]
    #[inline] pub fn test_rtmis(&self) -> bool {
        self.rtmis() != 0
    }

    #[doc="Sets the RTMIS field."]
    #[inline] pub fn set_rtmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Receive FIFO Masked Interrupt Status"]
    #[inline] pub fn rxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXMIS != 0"]
    #[inline] pub fn test_rxmis(&self) -> bool {
        self.rxmis() != 0
    }

    #[doc="Sets the RXMIS field."]
    #[inline] pub fn set_rxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SSI Transmit FIFO Masked Interrupt Status"]
    #[inline] pub fn txmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TXMIS != 0"]
    #[inline] pub fn test_txmis(&self) -> bool {
        self.txmis() != 0
    }

    #[doc="Sets the TXMIS field."]
    #[inline] pub fn set_txmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="SSI Receive DMA Masked Interrupt Status"]
    #[inline] pub fn dmarxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DMARXMIS != 0"]
    #[inline] pub fn test_dmarxmis(&self) -> bool {
        self.dmarxmis() != 0
    }

    #[doc="Sets the DMARXMIS field."]
    #[inline] pub fn set_dmarxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SSI Transmit DMA Masked Interrupt Status"]
    #[inline] pub fn dmatxmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMATXMIS != 0"]
    #[inline] pub fn test_dmatxmis(&self) -> bool {
        self.dmatxmis() != 0
    }

    #[doc="Sets the DMATXMIS field."]
    #[inline] pub fn set_dmatxmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="End of Transmit Masked Interrupt Status"]
    #[inline] pub fn eotmis(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if EOTMIS != 0"]
    #[inline] pub fn test_eotmis(&self) -> bool {
        self.eotmis() != 0
    }

    #[doc="Sets the EOTMIS field."]
    #[inline] pub fn set_eotmis<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Mis {
    #[inline]
    fn from(other: u32) -> Self {
         Mis(other)
    }
}

impl ::core::fmt::Display for Mis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rormis() != 0 { try!(write!(f, " rormis"))}
        if self.rtmis() != 0 { try!(write!(f, " rtmis"))}
        if self.rxmis() != 0 { try!(write!(f, " rxmis"))}
        if self.txmis() != 0 { try!(write!(f, " txmis"))}
        if self.dmarxmis() != 0 { try!(write!(f, " dmarxmis"))}
        if self.dmatxmis() != 0 { try!(write!(f, " dmatxmis"))}
        if self.eotmis() != 0 { try!(write!(f, " eotmis"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SSI Interrupt Clear"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc="SSI Receive Overrun Interrupt Clear"]
    #[inline] pub fn roric(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RORIC != 0"]
    #[inline] pub fn test_roric(&self) -> bool {
        self.roric() != 0
    }

    #[doc="Sets the RORIC field."]
    #[inline] pub fn set_roric<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="SSI Receive Time-Out Interrupt Clear"]
    #[inline] pub fn rtic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RTIC != 0"]
    #[inline] pub fn test_rtic(&self) -> bool {
        self.rtic() != 0
    }

    #[doc="Sets the RTIC field."]
    #[inline] pub fn set_rtic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="SSI Receive DMA Interrupt Clear"]
    #[inline] pub fn dmarxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DMARXIC != 0"]
    #[inline] pub fn test_dmarxic(&self) -> bool {
        self.dmarxic() != 0
    }

    #[doc="Sets the DMARXIC field."]
    #[inline] pub fn set_dmarxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SSI Transmit DMA Interrupt Clear"]
    #[inline] pub fn dmatxic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DMATXIC != 0"]
    #[inline] pub fn test_dmatxic(&self) -> bool {
        self.dmatxic() != 0
    }

    #[doc="Sets the DMATXIC field."]
    #[inline] pub fn set_dmatxic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="End of Transmit Interrupt Clear"]
    #[inline] pub fn eotic(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if EOTIC != 0"]
    #[inline] pub fn test_eotic(&self) -> bool {
        self.eotic() != 0
    }

    #[doc="Sets the EOTIC field."]
    #[inline] pub fn set_eotic<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Icr {
    #[inline]
    fn from(other: u32) -> Self {
         Icr(other)
    }
}

impl ::core::fmt::Display for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.roric() != 0 { try!(write!(f, " roric"))}
        if self.rtic() != 0 { try!(write!(f, " rtic"))}
        if self.dmarxic() != 0 { try!(write!(f, " dmarxic"))}
        if self.dmatxic() != 0 { try!(write!(f, " dmatxic"))}
        if self.eotic() != 0 { try!(write!(f, " eotic"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SSI DMA Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dmactl(pub u32);
impl Dmactl {
    #[doc="Receive DMA Enable"]
    #[inline] pub fn rxdmae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXDMAE != 0"]
    #[inline] pub fn test_rxdmae(&self) -> bool {
        self.rxdmae() != 0
    }

    #[doc="Sets the RXDMAE field."]
    #[inline] pub fn set_rxdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Transmit DMA Enable"]
    #[inline] pub fn txdmae(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXDMAE != 0"]
    #[inline] pub fn test_txdmae(&self) -> bool {
        self.txdmae() != 0
    }

    #[doc="Sets the TXDMAE field."]
    #[inline] pub fn set_txdmae<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Dmactl {
    #[inline]
    fn from(other: u32) -> Self {
         Dmactl(other)
    }
}

impl ::core::fmt::Display for Dmactl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dmactl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdmae() != 0 { try!(write!(f, " rxdmae"))}
        if self.txdmae() != 0 { try!(write!(f, " txdmae"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SSI Peripheral Properties"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
    #[doc="High Speed Capability"]
    #[inline] pub fn hsclk(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HSCLK != 0"]
    #[inline] pub fn test_hsclk(&self) -> bool {
        self.hsclk() != 0
    }

    #[doc="Sets the HSCLK field."]
    #[inline] pub fn set_hsclk<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Mode of Operation"]
    #[inline] pub fn mode(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x3) as u8) } // [2:1]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FSS Hold Frame Capability"]
    #[inline] pub fn fsshldfrm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FSSHLDFRM != 0"]
    #[inline] pub fn test_fsshldfrm(&self) -> bool {
        self.fsshldfrm() != 0
    }

    #[doc="Sets the FSSHLDFRM field."]
    #[inline] pub fn set_fsshldfrm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Pp {
    #[inline]
    fn from(other: u32) -> Self {
         Pp(other)
    }
}

impl ::core::fmt::Display for Pp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hsclk() != 0 { try!(write!(f, " hsclk"))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.fsshldfrm() != 0 { try!(write!(f, " fsshldfrm"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="SSI Clock Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
    #[doc="SSI Baud Clock Source"]
    #[inline] pub fn cs(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CS != 0"]
    #[inline] pub fn test_cs(&self) -> bool {
        self.cs() != 0
    }

    #[doc="Sets the CS field."]
    #[inline] pub fn set_cs<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cc {
    #[inline]
    fn from(other: u32) -> Self {
         Cc(other)
    }
}

impl ::core::fmt::Display for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cs() != 0 { try!(write!(f, " cs=0x{:x}", self.cs()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

