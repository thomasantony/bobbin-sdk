#[allow(unused_imports)] use bobbin_common::bits;
pub const UDMA: Udma = Periph(0x400ff000, UdmaId {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="UDMA Peripheral"]
pub struct Periph<T>(pub u32, pub T); 

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct UdmaId {}
pub type Udma = Periph<UdmaId>;



impl<T> Periph<T> {
#[doc="Get the *const pointer for the STAT register."]
  #[inline] pub fn stat_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the STAT register."]
  #[inline] pub fn stat_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the STAT register."]
  #[inline] pub fn stat(&self) -> Stat { 
     unsafe {
        Stat(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the STAT register."]
  #[inline] pub fn set_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
     let value = f(Stat(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the STAT register."]
  #[inline] pub fn with_stat<F: FnOnce(Stat) -> Stat>(&self, f: F) -> &Self {
     let tmp = self.stat();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CFG register."]
  #[inline] pub fn cfg_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the CFG register."]
  #[inline] pub fn cfg_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Write the CFG register."]
  #[inline] pub fn set_cfg<F: FnOnce(Cfg) -> Cfg>(&self, f: F) -> &Self {
     let value = f(Cfg(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CTLBASE register."]
  #[inline] pub fn ctlbase_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the CTLBASE register."]
  #[inline] pub fn ctlbase_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the CTLBASE register."]
  #[inline] pub fn ctlbase(&self) -> Ctlbase { 
     unsafe {
        Ctlbase(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the CTLBASE register."]
  #[inline] pub fn set_ctlbase<F: FnOnce(Ctlbase) -> Ctlbase>(&self, f: F) -> &Self {
     let value = f(Ctlbase(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CTLBASE register."]
  #[inline] pub fn with_ctlbase<F: FnOnce(Ctlbase) -> Ctlbase>(&self, f: F) -> &Self {
     let tmp = self.ctlbase();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ALTBASE register."]
  #[inline] pub fn altbase_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the ALTBASE register."]
  #[inline] pub fn altbase_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the ALTBASE register."]
  #[inline] pub fn altbase(&self) -> Altbase { 
     unsafe {
        Altbase(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the ALTBASE register."]
  #[inline] pub fn set_altbase<F: FnOnce(Altbase) -> Altbase>(&self, f: F) -> &Self {
     let value = f(Altbase(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ALTBASE register."]
  #[inline] pub fn with_altbase<F: FnOnce(Altbase) -> Altbase>(&self, f: F) -> &Self {
     let tmp = self.altbase();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the WAITSTAT register."]
  #[inline] pub fn waitstat_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the WAITSTAT register."]
  #[inline] pub fn waitstat_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the WAITSTAT register."]
  #[inline] pub fn waitstat(&self) -> Waitstat { 
     unsafe {
        Waitstat(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the WAITSTAT register."]
  #[inline] pub fn set_waitstat<F: FnOnce(Waitstat) -> Waitstat>(&self, f: F) -> &Self {
     let value = f(Waitstat(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the WAITSTAT register."]
  #[inline] pub fn with_waitstat<F: FnOnce(Waitstat) -> Waitstat>(&self, f: F) -> &Self {
     let tmp = self.waitstat();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SWREQ register."]
  #[inline] pub fn swreq_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the SWREQ register."]
  #[inline] pub fn swreq_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Write the SWREQ register."]
  #[inline] pub fn set_swreq<F: FnOnce(Swreq) -> Swreq>(&self, f: F) -> &Self {
     let value = f(Swreq(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the USEBURSTSET register."]
  #[inline] pub fn useburstset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x18) as *const u32
  }
#[doc="Get the *mut pointer for the USEBURSTSET register."]
  #[inline] pub fn useburstset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x18) as *mut u32
  }
#[doc="Read the USEBURSTSET register."]
  #[inline] pub fn useburstset(&self) -> Useburstset { 
     unsafe {
        Useburstset(::core::ptr::read_volatile(((self.0 as usize) + 0x18) as *const u32))
     }
  }
#[doc="Write the USEBURSTSET register."]
  #[inline] pub fn set_useburstset<F: FnOnce(Useburstset) -> Useburstset>(&self, f: F) -> &Self {
     let value = f(Useburstset(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the USEBURSTSET register."]
  #[inline] pub fn with_useburstset<F: FnOnce(Useburstset) -> Useburstset>(&self, f: F) -> &Self {
     let tmp = self.useburstset();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x18) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the USEBURSTCLR register."]
  #[inline] pub fn useburstclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the USEBURSTCLR register."]
  #[inline] pub fn useburstclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Write the USEBURSTCLR register."]
  #[inline] pub fn set_useburstclr<F: FnOnce(Useburstclr) -> Useburstclr>(&self, f: F) -> &Self {
     let value = f(Useburstclr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the REQMASKSET register."]
  #[inline] pub fn reqmaskset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the REQMASKSET register."]
  #[inline] pub fn reqmaskset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the REQMASKSET register."]
  #[inline] pub fn reqmaskset(&self) -> Reqmaskset { 
     unsafe {
        Reqmaskset(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the REQMASKSET register."]
  #[inline] pub fn set_reqmaskset<F: FnOnce(Reqmaskset) -> Reqmaskset>(&self, f: F) -> &Self {
     let value = f(Reqmaskset(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the REQMASKSET register."]
  #[inline] pub fn with_reqmaskset<F: FnOnce(Reqmaskset) -> Reqmaskset>(&self, f: F) -> &Self {
     let tmp = self.reqmaskset();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the REQMASKCLR register."]
  #[inline] pub fn reqmaskclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the REQMASKCLR register."]
  #[inline] pub fn reqmaskclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Write the REQMASKCLR register."]
  #[inline] pub fn set_reqmaskclr<F: FnOnce(Reqmaskclr) -> Reqmaskclr>(&self, f: F) -> &Self {
     let value = f(Reqmaskclr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ENASET register."]
  #[inline] pub fn enaset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the ENASET register."]
  #[inline] pub fn enaset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the ENASET register."]
  #[inline] pub fn enaset(&self) -> Enaset { 
     unsafe {
        Enaset(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }
#[doc="Write the ENASET register."]
  #[inline] pub fn set_enaset<F: FnOnce(Enaset) -> Enaset>(&self, f: F) -> &Self {
     let value = f(Enaset(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ENASET register."]
  #[inline] pub fn with_enaset<F: FnOnce(Enaset) -> Enaset>(&self, f: F) -> &Self {
     let tmp = self.enaset();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x28) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ENACLR register."]
  #[inline] pub fn enaclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
#[doc="Get the *mut pointer for the ENACLR register."]
  #[inline] pub fn enaclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
#[doc="Write the ENACLR register."]
  #[inline] pub fn set_enaclr<F: FnOnce(Enaclr) -> Enaclr>(&self, f: F) -> &Self {
     let value = f(Enaclr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ALTSET register."]
  #[inline] pub fn altset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
#[doc="Get the *mut pointer for the ALTSET register."]
  #[inline] pub fn altset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
#[doc="Read the ALTSET register."]
  #[inline] pub fn altset(&self) -> Altset { 
     unsafe {
        Altset(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }
#[doc="Write the ALTSET register."]
  #[inline] pub fn set_altset<F: FnOnce(Altset) -> Altset>(&self, f: F) -> &Self {
     let value = f(Altset(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ALTSET register."]
  #[inline] pub fn with_altset<F: FnOnce(Altset) -> Altset>(&self, f: F) -> &Self {
     let tmp = self.altset();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x30) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ALTCLR register."]
  #[inline] pub fn altclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
#[doc="Get the *mut pointer for the ALTCLR register."]
  #[inline] pub fn altclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
#[doc="Write the ALTCLR register."]
  #[inline] pub fn set_altclr<F: FnOnce(Altclr) -> Altclr>(&self, f: F) -> &Self {
     let value = f(Altclr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x34) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PRIOSET register."]
  #[inline] pub fn prioset_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
#[doc="Get the *mut pointer for the PRIOSET register."]
  #[inline] pub fn prioset_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
#[doc="Read the PRIOSET register."]
  #[inline] pub fn prioset(&self) -> Prioset { 
     unsafe {
        Prioset(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }
#[doc="Write the PRIOSET register."]
  #[inline] pub fn set_prioset<F: FnOnce(Prioset) -> Prioset>(&self, f: F) -> &Self {
     let value = f(Prioset(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PRIOSET register."]
  #[inline] pub fn with_prioset<F: FnOnce(Prioset) -> Prioset>(&self, f: F) -> &Self {
     let tmp = self.prioset();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x38) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the PRIOCLR register."]
  #[inline] pub fn prioclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3c) as *const u32
  }
#[doc="Get the *mut pointer for the PRIOCLR register."]
  #[inline] pub fn prioclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3c) as *mut u32
  }
#[doc="Write the PRIOCLR register."]
  #[inline] pub fn set_prioclr<F: FnOnce(Prioclr) -> Prioclr>(&self, f: F) -> &Self {
     let value = f(Prioclr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the ERRCLR register."]
  #[inline] pub fn errclr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4c) as *const u32
  }
#[doc="Get the *mut pointer for the ERRCLR register."]
  #[inline] pub fn errclr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4c) as *mut u32
  }
#[doc="Read the ERRCLR register."]
  #[inline] pub fn errclr(&self) -> Errclr { 
     unsafe {
        Errclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4c) as *const u32))
     }
  }
#[doc="Write the ERRCLR register."]
  #[inline] pub fn set_errclr<F: FnOnce(Errclr) -> Errclr>(&self, f: F) -> &Self {
     let value = f(Errclr(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ERRCLR register."]
  #[inline] pub fn with_errclr<F: FnOnce(Errclr) -> Errclr>(&self, f: F) -> &Self {
     let tmp = self.errclr();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CHASGN register."]
  #[inline] pub fn chasgn_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x500) as *const u32
  }
#[doc="Get the *mut pointer for the CHASGN register."]
  #[inline] pub fn chasgn_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x500) as *mut u32
  }
#[doc="Read the CHASGN register."]
  #[inline] pub fn chasgn(&self) -> Chasgn { 
     unsafe {
        Chasgn(::core::ptr::read_volatile(((self.0 as usize) + 0x500) as *const u32))
     }
  }
#[doc="Write the CHASGN register."]
  #[inline] pub fn set_chasgn<F: FnOnce(Chasgn) -> Chasgn>(&self, f: F) -> &Self {
     let value = f(Chasgn(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x500) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CHASGN register."]
  #[inline] pub fn with_chasgn<F: FnOnce(Chasgn) -> Chasgn>(&self, f: F) -> &Self {
     let tmp = self.chasgn();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x500) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CHMAP0 register."]
  #[inline] pub fn chmap0_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x510) as *const u32
  }
#[doc="Get the *mut pointer for the CHMAP0 register."]
  #[inline] pub fn chmap0_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x510) as *mut u32
  }
#[doc="Read the CHMAP0 register."]
  #[inline] pub fn chmap0(&self) -> Chmap0 { 
     unsafe {
        Chmap0(::core::ptr::read_volatile(((self.0 as usize) + 0x510) as *const u32))
     }
  }
#[doc="Write the CHMAP0 register."]
  #[inline] pub fn set_chmap0<F: FnOnce(Chmap0) -> Chmap0>(&self, f: F) -> &Self {
     let value = f(Chmap0(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x510) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CHMAP0 register."]
  #[inline] pub fn with_chmap0<F: FnOnce(Chmap0) -> Chmap0>(&self, f: F) -> &Self {
     let tmp = self.chmap0();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x510) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CHMAP1 register."]
  #[inline] pub fn chmap1_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x514) as *const u32
  }
#[doc="Get the *mut pointer for the CHMAP1 register."]
  #[inline] pub fn chmap1_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x514) as *mut u32
  }
#[doc="Read the CHMAP1 register."]
  #[inline] pub fn chmap1(&self) -> Chmap1 { 
     unsafe {
        Chmap1(::core::ptr::read_volatile(((self.0 as usize) + 0x514) as *const u32))
     }
  }
#[doc="Write the CHMAP1 register."]
  #[inline] pub fn set_chmap1<F: FnOnce(Chmap1) -> Chmap1>(&self, f: F) -> &Self {
     let value = f(Chmap1(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x514) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CHMAP1 register."]
  #[inline] pub fn with_chmap1<F: FnOnce(Chmap1) -> Chmap1>(&self, f: F) -> &Self {
     let tmp = self.chmap1();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x514) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CHMAP2 register."]
  #[inline] pub fn chmap2_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x518) as *const u32
  }
#[doc="Get the *mut pointer for the CHMAP2 register."]
  #[inline] pub fn chmap2_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x518) as *mut u32
  }
#[doc="Read the CHMAP2 register."]
  #[inline] pub fn chmap2(&self) -> Chmap2 { 
     unsafe {
        Chmap2(::core::ptr::read_volatile(((self.0 as usize) + 0x518) as *const u32))
     }
  }
#[doc="Write the CHMAP2 register."]
  #[inline] pub fn set_chmap2<F: FnOnce(Chmap2) -> Chmap2>(&self, f: F) -> &Self {
     let value = f(Chmap2(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x518) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CHMAP2 register."]
  #[inline] pub fn with_chmap2<F: FnOnce(Chmap2) -> Chmap2>(&self, f: F) -> &Self {
     let tmp = self.chmap2();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x518) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the CHMAP3 register."]
  #[inline] pub fn chmap3_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x51c) as *const u32
  }
#[doc="Get the *mut pointer for the CHMAP3 register."]
  #[inline] pub fn chmap3_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x51c) as *mut u32
  }
#[doc="Read the CHMAP3 register."]
  #[inline] pub fn chmap3(&self) -> Chmap3 { 
     unsafe {
        Chmap3(::core::ptr::read_volatile(((self.0 as usize) + 0x51c) as *const u32))
     }
  }
#[doc="Write the CHMAP3 register."]
  #[inline] pub fn set_chmap3<F: FnOnce(Chmap3) -> Chmap3>(&self, f: F) -> &Self {
     let value = f(Chmap3(0));
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x51c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CHMAP3 register."]
  #[inline] pub fn with_chmap3<F: FnOnce(Chmap3) -> Chmap3>(&self, f: F) -> &Self {
     let tmp = self.chmap3();
     let value = f(tmp);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x51c) as *mut u32, value.0);
     }
     self
  }

}

#[doc="DMA Status"]
#[derive(PartialEq, Eq)]
pub struct Stat(pub u32);
impl Stat {
#[doc="Master Enable Status"]
  #[inline] pub fn masten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Master Enable Status"]
  #[inline] pub fn set_masten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Control State Machine Status"]
  #[inline] pub fn state(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
  }
#[doc="Control State Machine Status"]
  #[inline] pub fn set_state<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Available uDMA Channels Minus 1"]
  #[inline] pub fn dmachans(&self) -> bits::U5 {
     unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1f) as u8) } // [20:16]
  }
#[doc="Available uDMA Channels Minus 1"]
  #[inline] pub fn set_dmachans<V: Into<bits::U5>>(mut self, value: V) -> Self {
     let value: bits::U5 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1f << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Stat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Stat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.masten() != 0 { try!(write!(f, " masten"))}
      if self.state() != 0 { try!(write!(f, " state=0x{:x}", self.state()))}
      if self.dmachans() != 0 { try!(write!(f, " dmachans=0x{:x}", self.dmachans()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Configuration"]
#[derive(PartialEq, Eq)]
pub struct Cfg(pub u32);
impl Cfg {
#[doc="Controller Master Enable"]
  #[inline] pub fn masten(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
  }
#[doc="Controller Master Enable"]
  #[inline] pub fn set_masten<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Cfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cfg {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.masten() != 0 { try!(write!(f, " masten"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Control Base Pointer"]
#[derive(PartialEq, Eq)]
pub struct Ctlbase(pub u32);
impl Ctlbase {
#[doc="Channel Control Base Address"]
  #[inline] pub fn addr(&self) -> bits::U22 {
     unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3fffff) as u32) } // [31:10]
  }
#[doc="Channel Control Base Address"]
  #[inline] pub fn set_addr<V: Into<bits::U22>>(mut self, value: V) -> Self {
     let value: bits::U22 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3fffff << 10);
     self.0 |= value << 10;
     self
  }

}
impl ::core::fmt::Display for Ctlbase {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctlbase {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.addr() != 0 { try!(write!(f, " addr=0x{:x}", self.addr()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Alternate Channel Control Base Pointer"]
#[derive(PartialEq, Eq)]
pub struct Altbase(pub u32);
impl Altbase {
#[doc="Alternate Channel Address Pointer"]
  #[inline] pub fn addr(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Alternate Channel Address Pointer"]
  #[inline] pub fn set_addr<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Altbase {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Altbase {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Wait-on-Request Status"]
#[derive(PartialEq, Eq)]
pub struct Waitstat(pub u32);
impl Waitstat {
#[doc="Channel [n] Wait Status"]
  #[inline] pub fn waitreq<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel [n] Wait Status"]
  #[inline] pub fn set_waitreq<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Waitstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Waitstat {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.waitreq(0) != 0 { try!(write!(f, " waitreq[0]"))}
      if self.waitreq(1) != 0 { try!(write!(f, " waitreq[1]"))}
      if self.waitreq(2) != 0 { try!(write!(f, " waitreq[2]"))}
      if self.waitreq(3) != 0 { try!(write!(f, " waitreq[3]"))}
      if self.waitreq(4) != 0 { try!(write!(f, " waitreq[4]"))}
      if self.waitreq(5) != 0 { try!(write!(f, " waitreq[5]"))}
      if self.waitreq(6) != 0 { try!(write!(f, " waitreq[6]"))}
      if self.waitreq(7) != 0 { try!(write!(f, " waitreq[7]"))}
      if self.waitreq(8) != 0 { try!(write!(f, " waitreq[8]"))}
      if self.waitreq(9) != 0 { try!(write!(f, " waitreq[9]"))}
      if self.waitreq(10) != 0 { try!(write!(f, " waitreq[10]"))}
      if self.waitreq(11) != 0 { try!(write!(f, " waitreq[11]"))}
      if self.waitreq(12) != 0 { try!(write!(f, " waitreq[12]"))}
      if self.waitreq(13) != 0 { try!(write!(f, " waitreq[13]"))}
      if self.waitreq(14) != 0 { try!(write!(f, " waitreq[14]"))}
      if self.waitreq(15) != 0 { try!(write!(f, " waitreq[15]"))}
      if self.waitreq(16) != 0 { try!(write!(f, " waitreq[16]"))}
      if self.waitreq(17) != 0 { try!(write!(f, " waitreq[17]"))}
      if self.waitreq(18) != 0 { try!(write!(f, " waitreq[18]"))}
      if self.waitreq(19) != 0 { try!(write!(f, " waitreq[19]"))}
      if self.waitreq(20) != 0 { try!(write!(f, " waitreq[20]"))}
      if self.waitreq(21) != 0 { try!(write!(f, " waitreq[21]"))}
      if self.waitreq(22) != 0 { try!(write!(f, " waitreq[22]"))}
      if self.waitreq(23) != 0 { try!(write!(f, " waitreq[23]"))}
      if self.waitreq(24) != 0 { try!(write!(f, " waitreq[24]"))}
      if self.waitreq(25) != 0 { try!(write!(f, " waitreq[25]"))}
      if self.waitreq(26) != 0 { try!(write!(f, " waitreq[26]"))}
      if self.waitreq(27) != 0 { try!(write!(f, " waitreq[27]"))}
      if self.waitreq(28) != 0 { try!(write!(f, " waitreq[28]"))}
      if self.waitreq(29) != 0 { try!(write!(f, " waitreq[29]"))}
      if self.waitreq(30) != 0 { try!(write!(f, " waitreq[30]"))}
      if self.waitreq(31) != 0 { try!(write!(f, " waitreq[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Software Request"]
#[derive(PartialEq, Eq)]
pub struct Swreq(pub u32);
impl Swreq {
#[doc="Channel [n] Software Request"]
  #[inline] pub fn swreq<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel [n] Software Request"]
  #[inline] pub fn set_swreq<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Swreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Swreq {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.swreq(0) != 0 { try!(write!(f, " swreq[0]"))}
      if self.swreq(1) != 0 { try!(write!(f, " swreq[1]"))}
      if self.swreq(2) != 0 { try!(write!(f, " swreq[2]"))}
      if self.swreq(3) != 0 { try!(write!(f, " swreq[3]"))}
      if self.swreq(4) != 0 { try!(write!(f, " swreq[4]"))}
      if self.swreq(5) != 0 { try!(write!(f, " swreq[5]"))}
      if self.swreq(6) != 0 { try!(write!(f, " swreq[6]"))}
      if self.swreq(7) != 0 { try!(write!(f, " swreq[7]"))}
      if self.swreq(8) != 0 { try!(write!(f, " swreq[8]"))}
      if self.swreq(9) != 0 { try!(write!(f, " swreq[9]"))}
      if self.swreq(10) != 0 { try!(write!(f, " swreq[10]"))}
      if self.swreq(11) != 0 { try!(write!(f, " swreq[11]"))}
      if self.swreq(12) != 0 { try!(write!(f, " swreq[12]"))}
      if self.swreq(13) != 0 { try!(write!(f, " swreq[13]"))}
      if self.swreq(14) != 0 { try!(write!(f, " swreq[14]"))}
      if self.swreq(15) != 0 { try!(write!(f, " swreq[15]"))}
      if self.swreq(16) != 0 { try!(write!(f, " swreq[16]"))}
      if self.swreq(17) != 0 { try!(write!(f, " swreq[17]"))}
      if self.swreq(18) != 0 { try!(write!(f, " swreq[18]"))}
      if self.swreq(19) != 0 { try!(write!(f, " swreq[19]"))}
      if self.swreq(20) != 0 { try!(write!(f, " swreq[20]"))}
      if self.swreq(21) != 0 { try!(write!(f, " swreq[21]"))}
      if self.swreq(22) != 0 { try!(write!(f, " swreq[22]"))}
      if self.swreq(23) != 0 { try!(write!(f, " swreq[23]"))}
      if self.swreq(24) != 0 { try!(write!(f, " swreq[24]"))}
      if self.swreq(25) != 0 { try!(write!(f, " swreq[25]"))}
      if self.swreq(26) != 0 { try!(write!(f, " swreq[26]"))}
      if self.swreq(27) != 0 { try!(write!(f, " swreq[27]"))}
      if self.swreq(28) != 0 { try!(write!(f, " swreq[28]"))}
      if self.swreq(29) != 0 { try!(write!(f, " swreq[29]"))}
      if self.swreq(30) != 0 { try!(write!(f, " swreq[30]"))}
      if self.swreq(31) != 0 { try!(write!(f, " swreq[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Useburst Set"]
#[derive(PartialEq, Eq)]
pub struct Useburstset(pub u32);
impl Useburstset {
#[doc="Channel [n] Useburst Set"]
  #[inline] pub fn set<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel [n] Useburst Set"]
  #[inline] pub fn set_set<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Useburstset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Useburstset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.set(0) != 0 { try!(write!(f, " set[0]"))}
      if self.set(1) != 0 { try!(write!(f, " set[1]"))}
      if self.set(2) != 0 { try!(write!(f, " set[2]"))}
      if self.set(3) != 0 { try!(write!(f, " set[3]"))}
      if self.set(4) != 0 { try!(write!(f, " set[4]"))}
      if self.set(5) != 0 { try!(write!(f, " set[5]"))}
      if self.set(6) != 0 { try!(write!(f, " set[6]"))}
      if self.set(7) != 0 { try!(write!(f, " set[7]"))}
      if self.set(8) != 0 { try!(write!(f, " set[8]"))}
      if self.set(9) != 0 { try!(write!(f, " set[9]"))}
      if self.set(10) != 0 { try!(write!(f, " set[10]"))}
      if self.set(11) != 0 { try!(write!(f, " set[11]"))}
      if self.set(12) != 0 { try!(write!(f, " set[12]"))}
      if self.set(13) != 0 { try!(write!(f, " set[13]"))}
      if self.set(14) != 0 { try!(write!(f, " set[14]"))}
      if self.set(15) != 0 { try!(write!(f, " set[15]"))}
      if self.set(16) != 0 { try!(write!(f, " set[16]"))}
      if self.set(17) != 0 { try!(write!(f, " set[17]"))}
      if self.set(18) != 0 { try!(write!(f, " set[18]"))}
      if self.set(19) != 0 { try!(write!(f, " set[19]"))}
      if self.set(20) != 0 { try!(write!(f, " set[20]"))}
      if self.set(21) != 0 { try!(write!(f, " set[21]"))}
      if self.set(22) != 0 { try!(write!(f, " set[22]"))}
      if self.set(23) != 0 { try!(write!(f, " set[23]"))}
      if self.set(24) != 0 { try!(write!(f, " set[24]"))}
      if self.set(25) != 0 { try!(write!(f, " set[25]"))}
      if self.set(26) != 0 { try!(write!(f, " set[26]"))}
      if self.set(27) != 0 { try!(write!(f, " set[27]"))}
      if self.set(28) != 0 { try!(write!(f, " set[28]"))}
      if self.set(29) != 0 { try!(write!(f, " set[29]"))}
      if self.set(30) != 0 { try!(write!(f, " set[30]"))}
      if self.set(31) != 0 { try!(write!(f, " set[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Useburst Clear"]
#[derive(PartialEq, Eq)]
pub struct Useburstclr(pub u32);
impl Useburstclr {
#[doc="Channel [n] Useburst Clear"]
  #[inline] pub fn clr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel [n] Useburst Clear"]
  #[inline] pub fn set_clr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Useburstclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Useburstclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clr(0) != 0 { try!(write!(f, " clr[0]"))}
      if self.clr(1) != 0 { try!(write!(f, " clr[1]"))}
      if self.clr(2) != 0 { try!(write!(f, " clr[2]"))}
      if self.clr(3) != 0 { try!(write!(f, " clr[3]"))}
      if self.clr(4) != 0 { try!(write!(f, " clr[4]"))}
      if self.clr(5) != 0 { try!(write!(f, " clr[5]"))}
      if self.clr(6) != 0 { try!(write!(f, " clr[6]"))}
      if self.clr(7) != 0 { try!(write!(f, " clr[7]"))}
      if self.clr(8) != 0 { try!(write!(f, " clr[8]"))}
      if self.clr(9) != 0 { try!(write!(f, " clr[9]"))}
      if self.clr(10) != 0 { try!(write!(f, " clr[10]"))}
      if self.clr(11) != 0 { try!(write!(f, " clr[11]"))}
      if self.clr(12) != 0 { try!(write!(f, " clr[12]"))}
      if self.clr(13) != 0 { try!(write!(f, " clr[13]"))}
      if self.clr(14) != 0 { try!(write!(f, " clr[14]"))}
      if self.clr(15) != 0 { try!(write!(f, " clr[15]"))}
      if self.clr(16) != 0 { try!(write!(f, " clr[16]"))}
      if self.clr(17) != 0 { try!(write!(f, " clr[17]"))}
      if self.clr(18) != 0 { try!(write!(f, " clr[18]"))}
      if self.clr(19) != 0 { try!(write!(f, " clr[19]"))}
      if self.clr(20) != 0 { try!(write!(f, " clr[20]"))}
      if self.clr(21) != 0 { try!(write!(f, " clr[21]"))}
      if self.clr(22) != 0 { try!(write!(f, " clr[22]"))}
      if self.clr(23) != 0 { try!(write!(f, " clr[23]"))}
      if self.clr(24) != 0 { try!(write!(f, " clr[24]"))}
      if self.clr(25) != 0 { try!(write!(f, " clr[25]"))}
      if self.clr(26) != 0 { try!(write!(f, " clr[26]"))}
      if self.clr(27) != 0 { try!(write!(f, " clr[27]"))}
      if self.clr(28) != 0 { try!(write!(f, " clr[28]"))}
      if self.clr(29) != 0 { try!(write!(f, " clr[29]"))}
      if self.clr(30) != 0 { try!(write!(f, " clr[30]"))}
      if self.clr(31) != 0 { try!(write!(f, " clr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Request Mask Set"]
#[derive(PartialEq, Eq)]
pub struct Reqmaskset(pub u32);
impl Reqmaskset {
#[doc="Channel [n] Request Mask Set"]
  #[inline] pub fn set<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel [n] Request Mask Set"]
  #[inline] pub fn set_set<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Reqmaskset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Reqmaskset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.set(0) != 0 { try!(write!(f, " set[0]"))}
      if self.set(1) != 0 { try!(write!(f, " set[1]"))}
      if self.set(2) != 0 { try!(write!(f, " set[2]"))}
      if self.set(3) != 0 { try!(write!(f, " set[3]"))}
      if self.set(4) != 0 { try!(write!(f, " set[4]"))}
      if self.set(5) != 0 { try!(write!(f, " set[5]"))}
      if self.set(6) != 0 { try!(write!(f, " set[6]"))}
      if self.set(7) != 0 { try!(write!(f, " set[7]"))}
      if self.set(8) != 0 { try!(write!(f, " set[8]"))}
      if self.set(9) != 0 { try!(write!(f, " set[9]"))}
      if self.set(10) != 0 { try!(write!(f, " set[10]"))}
      if self.set(11) != 0 { try!(write!(f, " set[11]"))}
      if self.set(12) != 0 { try!(write!(f, " set[12]"))}
      if self.set(13) != 0 { try!(write!(f, " set[13]"))}
      if self.set(14) != 0 { try!(write!(f, " set[14]"))}
      if self.set(15) != 0 { try!(write!(f, " set[15]"))}
      if self.set(16) != 0 { try!(write!(f, " set[16]"))}
      if self.set(17) != 0 { try!(write!(f, " set[17]"))}
      if self.set(18) != 0 { try!(write!(f, " set[18]"))}
      if self.set(19) != 0 { try!(write!(f, " set[19]"))}
      if self.set(20) != 0 { try!(write!(f, " set[20]"))}
      if self.set(21) != 0 { try!(write!(f, " set[21]"))}
      if self.set(22) != 0 { try!(write!(f, " set[22]"))}
      if self.set(23) != 0 { try!(write!(f, " set[23]"))}
      if self.set(24) != 0 { try!(write!(f, " set[24]"))}
      if self.set(25) != 0 { try!(write!(f, " set[25]"))}
      if self.set(26) != 0 { try!(write!(f, " set[26]"))}
      if self.set(27) != 0 { try!(write!(f, " set[27]"))}
      if self.set(28) != 0 { try!(write!(f, " set[28]"))}
      if self.set(29) != 0 { try!(write!(f, " set[29]"))}
      if self.set(30) != 0 { try!(write!(f, " set[30]"))}
      if self.set(31) != 0 { try!(write!(f, " set[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Request Mask Clear"]
#[derive(PartialEq, Eq)]
pub struct Reqmaskclr(pub u32);
impl Reqmaskclr {
#[doc="Channel [n] Request Mask Clear"]
  #[inline] pub fn clr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel [n] Request Mask Clear"]
  #[inline] pub fn set_clr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Reqmaskclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Reqmaskclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clr(0) != 0 { try!(write!(f, " clr[0]"))}
      if self.clr(1) != 0 { try!(write!(f, " clr[1]"))}
      if self.clr(2) != 0 { try!(write!(f, " clr[2]"))}
      if self.clr(3) != 0 { try!(write!(f, " clr[3]"))}
      if self.clr(4) != 0 { try!(write!(f, " clr[4]"))}
      if self.clr(5) != 0 { try!(write!(f, " clr[5]"))}
      if self.clr(6) != 0 { try!(write!(f, " clr[6]"))}
      if self.clr(7) != 0 { try!(write!(f, " clr[7]"))}
      if self.clr(8) != 0 { try!(write!(f, " clr[8]"))}
      if self.clr(9) != 0 { try!(write!(f, " clr[9]"))}
      if self.clr(10) != 0 { try!(write!(f, " clr[10]"))}
      if self.clr(11) != 0 { try!(write!(f, " clr[11]"))}
      if self.clr(12) != 0 { try!(write!(f, " clr[12]"))}
      if self.clr(13) != 0 { try!(write!(f, " clr[13]"))}
      if self.clr(14) != 0 { try!(write!(f, " clr[14]"))}
      if self.clr(15) != 0 { try!(write!(f, " clr[15]"))}
      if self.clr(16) != 0 { try!(write!(f, " clr[16]"))}
      if self.clr(17) != 0 { try!(write!(f, " clr[17]"))}
      if self.clr(18) != 0 { try!(write!(f, " clr[18]"))}
      if self.clr(19) != 0 { try!(write!(f, " clr[19]"))}
      if self.clr(20) != 0 { try!(write!(f, " clr[20]"))}
      if self.clr(21) != 0 { try!(write!(f, " clr[21]"))}
      if self.clr(22) != 0 { try!(write!(f, " clr[22]"))}
      if self.clr(23) != 0 { try!(write!(f, " clr[23]"))}
      if self.clr(24) != 0 { try!(write!(f, " clr[24]"))}
      if self.clr(25) != 0 { try!(write!(f, " clr[25]"))}
      if self.clr(26) != 0 { try!(write!(f, " clr[26]"))}
      if self.clr(27) != 0 { try!(write!(f, " clr[27]"))}
      if self.clr(28) != 0 { try!(write!(f, " clr[28]"))}
      if self.clr(29) != 0 { try!(write!(f, " clr[29]"))}
      if self.clr(30) != 0 { try!(write!(f, " clr[30]"))}
      if self.clr(31) != 0 { try!(write!(f, " clr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Enable Set"]
#[derive(PartialEq, Eq)]
pub struct Enaset(pub u32);
impl Enaset {
#[doc="Channel [n] Enable Set"]
  #[inline] pub fn set<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel [n] Enable Set"]
  #[inline] pub fn set_set<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Enaset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Enaset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.set(0) != 0 { try!(write!(f, " set[0]"))}
      if self.set(1) != 0 { try!(write!(f, " set[1]"))}
      if self.set(2) != 0 { try!(write!(f, " set[2]"))}
      if self.set(3) != 0 { try!(write!(f, " set[3]"))}
      if self.set(4) != 0 { try!(write!(f, " set[4]"))}
      if self.set(5) != 0 { try!(write!(f, " set[5]"))}
      if self.set(6) != 0 { try!(write!(f, " set[6]"))}
      if self.set(7) != 0 { try!(write!(f, " set[7]"))}
      if self.set(8) != 0 { try!(write!(f, " set[8]"))}
      if self.set(9) != 0 { try!(write!(f, " set[9]"))}
      if self.set(10) != 0 { try!(write!(f, " set[10]"))}
      if self.set(11) != 0 { try!(write!(f, " set[11]"))}
      if self.set(12) != 0 { try!(write!(f, " set[12]"))}
      if self.set(13) != 0 { try!(write!(f, " set[13]"))}
      if self.set(14) != 0 { try!(write!(f, " set[14]"))}
      if self.set(15) != 0 { try!(write!(f, " set[15]"))}
      if self.set(16) != 0 { try!(write!(f, " set[16]"))}
      if self.set(17) != 0 { try!(write!(f, " set[17]"))}
      if self.set(18) != 0 { try!(write!(f, " set[18]"))}
      if self.set(19) != 0 { try!(write!(f, " set[19]"))}
      if self.set(20) != 0 { try!(write!(f, " set[20]"))}
      if self.set(21) != 0 { try!(write!(f, " set[21]"))}
      if self.set(22) != 0 { try!(write!(f, " set[22]"))}
      if self.set(23) != 0 { try!(write!(f, " set[23]"))}
      if self.set(24) != 0 { try!(write!(f, " set[24]"))}
      if self.set(25) != 0 { try!(write!(f, " set[25]"))}
      if self.set(26) != 0 { try!(write!(f, " set[26]"))}
      if self.set(27) != 0 { try!(write!(f, " set[27]"))}
      if self.set(28) != 0 { try!(write!(f, " set[28]"))}
      if self.set(29) != 0 { try!(write!(f, " set[29]"))}
      if self.set(30) != 0 { try!(write!(f, " set[30]"))}
      if self.set(31) != 0 { try!(write!(f, " set[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Enable Clear"]
#[derive(PartialEq, Eq)]
pub struct Enaclr(pub u32);
impl Enaclr {
#[doc="Clear Channel [n] Enable Clear"]
  #[inline] pub fn clr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Clear Channel [n] Enable Clear"]
  #[inline] pub fn set_clr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Enaclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Enaclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clr(0) != 0 { try!(write!(f, " clr[0]"))}
      if self.clr(1) != 0 { try!(write!(f, " clr[1]"))}
      if self.clr(2) != 0 { try!(write!(f, " clr[2]"))}
      if self.clr(3) != 0 { try!(write!(f, " clr[3]"))}
      if self.clr(4) != 0 { try!(write!(f, " clr[4]"))}
      if self.clr(5) != 0 { try!(write!(f, " clr[5]"))}
      if self.clr(6) != 0 { try!(write!(f, " clr[6]"))}
      if self.clr(7) != 0 { try!(write!(f, " clr[7]"))}
      if self.clr(8) != 0 { try!(write!(f, " clr[8]"))}
      if self.clr(9) != 0 { try!(write!(f, " clr[9]"))}
      if self.clr(10) != 0 { try!(write!(f, " clr[10]"))}
      if self.clr(11) != 0 { try!(write!(f, " clr[11]"))}
      if self.clr(12) != 0 { try!(write!(f, " clr[12]"))}
      if self.clr(13) != 0 { try!(write!(f, " clr[13]"))}
      if self.clr(14) != 0 { try!(write!(f, " clr[14]"))}
      if self.clr(15) != 0 { try!(write!(f, " clr[15]"))}
      if self.clr(16) != 0 { try!(write!(f, " clr[16]"))}
      if self.clr(17) != 0 { try!(write!(f, " clr[17]"))}
      if self.clr(18) != 0 { try!(write!(f, " clr[18]"))}
      if self.clr(19) != 0 { try!(write!(f, " clr[19]"))}
      if self.clr(20) != 0 { try!(write!(f, " clr[20]"))}
      if self.clr(21) != 0 { try!(write!(f, " clr[21]"))}
      if self.clr(22) != 0 { try!(write!(f, " clr[22]"))}
      if self.clr(23) != 0 { try!(write!(f, " clr[23]"))}
      if self.clr(24) != 0 { try!(write!(f, " clr[24]"))}
      if self.clr(25) != 0 { try!(write!(f, " clr[25]"))}
      if self.clr(26) != 0 { try!(write!(f, " clr[26]"))}
      if self.clr(27) != 0 { try!(write!(f, " clr[27]"))}
      if self.clr(28) != 0 { try!(write!(f, " clr[28]"))}
      if self.clr(29) != 0 { try!(write!(f, " clr[29]"))}
      if self.clr(30) != 0 { try!(write!(f, " clr[30]"))}
      if self.clr(31) != 0 { try!(write!(f, " clr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Primary Alternate Set"]
#[derive(PartialEq, Eq)]
pub struct Altset(pub u32);
impl Altset {
#[doc="Channel [n] Alternate Set"]
  #[inline] pub fn set<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel [n] Alternate Set"]
  #[inline] pub fn set_set<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Altset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Altset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.set(0) != 0 { try!(write!(f, " set[0]"))}
      if self.set(1) != 0 { try!(write!(f, " set[1]"))}
      if self.set(2) != 0 { try!(write!(f, " set[2]"))}
      if self.set(3) != 0 { try!(write!(f, " set[3]"))}
      if self.set(4) != 0 { try!(write!(f, " set[4]"))}
      if self.set(5) != 0 { try!(write!(f, " set[5]"))}
      if self.set(6) != 0 { try!(write!(f, " set[6]"))}
      if self.set(7) != 0 { try!(write!(f, " set[7]"))}
      if self.set(8) != 0 { try!(write!(f, " set[8]"))}
      if self.set(9) != 0 { try!(write!(f, " set[9]"))}
      if self.set(10) != 0 { try!(write!(f, " set[10]"))}
      if self.set(11) != 0 { try!(write!(f, " set[11]"))}
      if self.set(12) != 0 { try!(write!(f, " set[12]"))}
      if self.set(13) != 0 { try!(write!(f, " set[13]"))}
      if self.set(14) != 0 { try!(write!(f, " set[14]"))}
      if self.set(15) != 0 { try!(write!(f, " set[15]"))}
      if self.set(16) != 0 { try!(write!(f, " set[16]"))}
      if self.set(17) != 0 { try!(write!(f, " set[17]"))}
      if self.set(18) != 0 { try!(write!(f, " set[18]"))}
      if self.set(19) != 0 { try!(write!(f, " set[19]"))}
      if self.set(20) != 0 { try!(write!(f, " set[20]"))}
      if self.set(21) != 0 { try!(write!(f, " set[21]"))}
      if self.set(22) != 0 { try!(write!(f, " set[22]"))}
      if self.set(23) != 0 { try!(write!(f, " set[23]"))}
      if self.set(24) != 0 { try!(write!(f, " set[24]"))}
      if self.set(25) != 0 { try!(write!(f, " set[25]"))}
      if self.set(26) != 0 { try!(write!(f, " set[26]"))}
      if self.set(27) != 0 { try!(write!(f, " set[27]"))}
      if self.set(28) != 0 { try!(write!(f, " set[28]"))}
      if self.set(29) != 0 { try!(write!(f, " set[29]"))}
      if self.set(30) != 0 { try!(write!(f, " set[30]"))}
      if self.set(31) != 0 { try!(write!(f, " set[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Primary Alternate Clear"]
#[derive(PartialEq, Eq)]
pub struct Altclr(pub u32);
impl Altclr {
#[doc="Channel [n] Alternate Clear"]
  #[inline] pub fn clr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel [n] Alternate Clear"]
  #[inline] pub fn set_clr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Altclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Altclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clr(0) != 0 { try!(write!(f, " clr[0]"))}
      if self.clr(1) != 0 { try!(write!(f, " clr[1]"))}
      if self.clr(2) != 0 { try!(write!(f, " clr[2]"))}
      if self.clr(3) != 0 { try!(write!(f, " clr[3]"))}
      if self.clr(4) != 0 { try!(write!(f, " clr[4]"))}
      if self.clr(5) != 0 { try!(write!(f, " clr[5]"))}
      if self.clr(6) != 0 { try!(write!(f, " clr[6]"))}
      if self.clr(7) != 0 { try!(write!(f, " clr[7]"))}
      if self.clr(8) != 0 { try!(write!(f, " clr[8]"))}
      if self.clr(9) != 0 { try!(write!(f, " clr[9]"))}
      if self.clr(10) != 0 { try!(write!(f, " clr[10]"))}
      if self.clr(11) != 0 { try!(write!(f, " clr[11]"))}
      if self.clr(12) != 0 { try!(write!(f, " clr[12]"))}
      if self.clr(13) != 0 { try!(write!(f, " clr[13]"))}
      if self.clr(14) != 0 { try!(write!(f, " clr[14]"))}
      if self.clr(15) != 0 { try!(write!(f, " clr[15]"))}
      if self.clr(16) != 0 { try!(write!(f, " clr[16]"))}
      if self.clr(17) != 0 { try!(write!(f, " clr[17]"))}
      if self.clr(18) != 0 { try!(write!(f, " clr[18]"))}
      if self.clr(19) != 0 { try!(write!(f, " clr[19]"))}
      if self.clr(20) != 0 { try!(write!(f, " clr[20]"))}
      if self.clr(21) != 0 { try!(write!(f, " clr[21]"))}
      if self.clr(22) != 0 { try!(write!(f, " clr[22]"))}
      if self.clr(23) != 0 { try!(write!(f, " clr[23]"))}
      if self.clr(24) != 0 { try!(write!(f, " clr[24]"))}
      if self.clr(25) != 0 { try!(write!(f, " clr[25]"))}
      if self.clr(26) != 0 { try!(write!(f, " clr[26]"))}
      if self.clr(27) != 0 { try!(write!(f, " clr[27]"))}
      if self.clr(28) != 0 { try!(write!(f, " clr[28]"))}
      if self.clr(29) != 0 { try!(write!(f, " clr[29]"))}
      if self.clr(30) != 0 { try!(write!(f, " clr[30]"))}
      if self.clr(31) != 0 { try!(write!(f, " clr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Priority Set"]
#[derive(PartialEq, Eq)]
pub struct Prioset(pub u32);
impl Prioset {
#[doc="Channel [n] Priority Set"]
  #[inline] pub fn set<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel [n] Priority Set"]
  #[inline] pub fn set_set<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Prioset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prioset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.set(0) != 0 { try!(write!(f, " set[0]"))}
      if self.set(1) != 0 { try!(write!(f, " set[1]"))}
      if self.set(2) != 0 { try!(write!(f, " set[2]"))}
      if self.set(3) != 0 { try!(write!(f, " set[3]"))}
      if self.set(4) != 0 { try!(write!(f, " set[4]"))}
      if self.set(5) != 0 { try!(write!(f, " set[5]"))}
      if self.set(6) != 0 { try!(write!(f, " set[6]"))}
      if self.set(7) != 0 { try!(write!(f, " set[7]"))}
      if self.set(8) != 0 { try!(write!(f, " set[8]"))}
      if self.set(9) != 0 { try!(write!(f, " set[9]"))}
      if self.set(10) != 0 { try!(write!(f, " set[10]"))}
      if self.set(11) != 0 { try!(write!(f, " set[11]"))}
      if self.set(12) != 0 { try!(write!(f, " set[12]"))}
      if self.set(13) != 0 { try!(write!(f, " set[13]"))}
      if self.set(14) != 0 { try!(write!(f, " set[14]"))}
      if self.set(15) != 0 { try!(write!(f, " set[15]"))}
      if self.set(16) != 0 { try!(write!(f, " set[16]"))}
      if self.set(17) != 0 { try!(write!(f, " set[17]"))}
      if self.set(18) != 0 { try!(write!(f, " set[18]"))}
      if self.set(19) != 0 { try!(write!(f, " set[19]"))}
      if self.set(20) != 0 { try!(write!(f, " set[20]"))}
      if self.set(21) != 0 { try!(write!(f, " set[21]"))}
      if self.set(22) != 0 { try!(write!(f, " set[22]"))}
      if self.set(23) != 0 { try!(write!(f, " set[23]"))}
      if self.set(24) != 0 { try!(write!(f, " set[24]"))}
      if self.set(25) != 0 { try!(write!(f, " set[25]"))}
      if self.set(26) != 0 { try!(write!(f, " set[26]"))}
      if self.set(27) != 0 { try!(write!(f, " set[27]"))}
      if self.set(28) != 0 { try!(write!(f, " set[28]"))}
      if self.set(29) != 0 { try!(write!(f, " set[29]"))}
      if self.set(30) != 0 { try!(write!(f, " set[30]"))}
      if self.set(31) != 0 { try!(write!(f, " set[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Priority Clear"]
#[derive(PartialEq, Eq)]
pub struct Prioclr(pub u32);
impl Prioclr {
#[doc="Channel [n] Priority Clear"]
  #[inline] pub fn clr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel [n] Priority Clear"]
  #[inline] pub fn set_clr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Prioclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prioclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clr(0) != 0 { try!(write!(f, " clr[0]"))}
      if self.clr(1) != 0 { try!(write!(f, " clr[1]"))}
      if self.clr(2) != 0 { try!(write!(f, " clr[2]"))}
      if self.clr(3) != 0 { try!(write!(f, " clr[3]"))}
      if self.clr(4) != 0 { try!(write!(f, " clr[4]"))}
      if self.clr(5) != 0 { try!(write!(f, " clr[5]"))}
      if self.clr(6) != 0 { try!(write!(f, " clr[6]"))}
      if self.clr(7) != 0 { try!(write!(f, " clr[7]"))}
      if self.clr(8) != 0 { try!(write!(f, " clr[8]"))}
      if self.clr(9) != 0 { try!(write!(f, " clr[9]"))}
      if self.clr(10) != 0 { try!(write!(f, " clr[10]"))}
      if self.clr(11) != 0 { try!(write!(f, " clr[11]"))}
      if self.clr(12) != 0 { try!(write!(f, " clr[12]"))}
      if self.clr(13) != 0 { try!(write!(f, " clr[13]"))}
      if self.clr(14) != 0 { try!(write!(f, " clr[14]"))}
      if self.clr(15) != 0 { try!(write!(f, " clr[15]"))}
      if self.clr(16) != 0 { try!(write!(f, " clr[16]"))}
      if self.clr(17) != 0 { try!(write!(f, " clr[17]"))}
      if self.clr(18) != 0 { try!(write!(f, " clr[18]"))}
      if self.clr(19) != 0 { try!(write!(f, " clr[19]"))}
      if self.clr(20) != 0 { try!(write!(f, " clr[20]"))}
      if self.clr(21) != 0 { try!(write!(f, " clr[21]"))}
      if self.clr(22) != 0 { try!(write!(f, " clr[22]"))}
      if self.clr(23) != 0 { try!(write!(f, " clr[23]"))}
      if self.clr(24) != 0 { try!(write!(f, " clr[24]"))}
      if self.clr(25) != 0 { try!(write!(f, " clr[25]"))}
      if self.clr(26) != 0 { try!(write!(f, " clr[26]"))}
      if self.clr(27) != 0 { try!(write!(f, " clr[27]"))}
      if self.clr(28) != 0 { try!(write!(f, " clr[28]"))}
      if self.clr(29) != 0 { try!(write!(f, " clr[29]"))}
      if self.clr(30) != 0 { try!(write!(f, " clr[30]"))}
      if self.clr(31) != 0 { try!(write!(f, " clr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Bus Error Clear"]
#[derive(PartialEq, Eq)]
pub struct Errclr(pub u32);
impl Errclr {
#[doc="uDMA Bus Error Status"]
  #[inline] pub fn errclr<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="uDMA Bus Error Status"]
  #[inline] pub fn set_errclr<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Errclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Errclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.errclr(0) != 0 { try!(write!(f, " errclr[0]"))}
      if self.errclr(1) != 0 { try!(write!(f, " errclr[1]"))}
      if self.errclr(2) != 0 { try!(write!(f, " errclr[2]"))}
      if self.errclr(3) != 0 { try!(write!(f, " errclr[3]"))}
      if self.errclr(4) != 0 { try!(write!(f, " errclr[4]"))}
      if self.errclr(5) != 0 { try!(write!(f, " errclr[5]"))}
      if self.errclr(6) != 0 { try!(write!(f, " errclr[6]"))}
      if self.errclr(7) != 0 { try!(write!(f, " errclr[7]"))}
      if self.errclr(8) != 0 { try!(write!(f, " errclr[8]"))}
      if self.errclr(9) != 0 { try!(write!(f, " errclr[9]"))}
      if self.errclr(10) != 0 { try!(write!(f, " errclr[10]"))}
      if self.errclr(11) != 0 { try!(write!(f, " errclr[11]"))}
      if self.errclr(12) != 0 { try!(write!(f, " errclr[12]"))}
      if self.errclr(13) != 0 { try!(write!(f, " errclr[13]"))}
      if self.errclr(14) != 0 { try!(write!(f, " errclr[14]"))}
      if self.errclr(15) != 0 { try!(write!(f, " errclr[15]"))}
      if self.errclr(16) != 0 { try!(write!(f, " errclr[16]"))}
      if self.errclr(17) != 0 { try!(write!(f, " errclr[17]"))}
      if self.errclr(18) != 0 { try!(write!(f, " errclr[18]"))}
      if self.errclr(19) != 0 { try!(write!(f, " errclr[19]"))}
      if self.errclr(20) != 0 { try!(write!(f, " errclr[20]"))}
      if self.errclr(21) != 0 { try!(write!(f, " errclr[21]"))}
      if self.errclr(22) != 0 { try!(write!(f, " errclr[22]"))}
      if self.errclr(23) != 0 { try!(write!(f, " errclr[23]"))}
      if self.errclr(24) != 0 { try!(write!(f, " errclr[24]"))}
      if self.errclr(25) != 0 { try!(write!(f, " errclr[25]"))}
      if self.errclr(26) != 0 { try!(write!(f, " errclr[26]"))}
      if self.errclr(27) != 0 { try!(write!(f, " errclr[27]"))}
      if self.errclr(28) != 0 { try!(write!(f, " errclr[28]"))}
      if self.errclr(29) != 0 { try!(write!(f, " errclr[29]"))}
      if self.errclr(30) != 0 { try!(write!(f, " errclr[30]"))}
      if self.errclr(31) != 0 { try!(write!(f, " errclr[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Assignment"]
#[derive(PartialEq, Eq)]
pub struct Chasgn(pub u32);
impl Chasgn {
#[doc="Channel [n] Assignment Select"]
  #[inline] pub fn chasgn<I: Into<bits::R32>>(&self, index: I) -> bits::U1 {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + index;
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
  }
#[doc="Channel [n] Assignment Select"]
  #[inline] pub fn set_chasgn<I: Into<bits::R32>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
     let index: bits::R32 = index.into();
     let index: usize = index.value();
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     assert!(index < 32);
     let shift: usize = 0 + index;
     self.0 &= !(0x1 << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Chasgn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chasgn {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chasgn(0) != 0 { try!(write!(f, " chasgn[0]"))}
      if self.chasgn(1) != 0 { try!(write!(f, " chasgn[1]"))}
      if self.chasgn(2) != 0 { try!(write!(f, " chasgn[2]"))}
      if self.chasgn(3) != 0 { try!(write!(f, " chasgn[3]"))}
      if self.chasgn(4) != 0 { try!(write!(f, " chasgn[4]"))}
      if self.chasgn(5) != 0 { try!(write!(f, " chasgn[5]"))}
      if self.chasgn(6) != 0 { try!(write!(f, " chasgn[6]"))}
      if self.chasgn(7) != 0 { try!(write!(f, " chasgn[7]"))}
      if self.chasgn(8) != 0 { try!(write!(f, " chasgn[8]"))}
      if self.chasgn(9) != 0 { try!(write!(f, " chasgn[9]"))}
      if self.chasgn(10) != 0 { try!(write!(f, " chasgn[10]"))}
      if self.chasgn(11) != 0 { try!(write!(f, " chasgn[11]"))}
      if self.chasgn(12) != 0 { try!(write!(f, " chasgn[12]"))}
      if self.chasgn(13) != 0 { try!(write!(f, " chasgn[13]"))}
      if self.chasgn(14) != 0 { try!(write!(f, " chasgn[14]"))}
      if self.chasgn(15) != 0 { try!(write!(f, " chasgn[15]"))}
      if self.chasgn(16) != 0 { try!(write!(f, " chasgn[16]"))}
      if self.chasgn(17) != 0 { try!(write!(f, " chasgn[17]"))}
      if self.chasgn(18) != 0 { try!(write!(f, " chasgn[18]"))}
      if self.chasgn(19) != 0 { try!(write!(f, " chasgn[19]"))}
      if self.chasgn(20) != 0 { try!(write!(f, " chasgn[20]"))}
      if self.chasgn(21) != 0 { try!(write!(f, " chasgn[21]"))}
      if self.chasgn(22) != 0 { try!(write!(f, " chasgn[22]"))}
      if self.chasgn(23) != 0 { try!(write!(f, " chasgn[23]"))}
      if self.chasgn(24) != 0 { try!(write!(f, " chasgn[24]"))}
      if self.chasgn(25) != 0 { try!(write!(f, " chasgn[25]"))}
      if self.chasgn(26) != 0 { try!(write!(f, " chasgn[26]"))}
      if self.chasgn(27) != 0 { try!(write!(f, " chasgn[27]"))}
      if self.chasgn(28) != 0 { try!(write!(f, " chasgn[28]"))}
      if self.chasgn(29) != 0 { try!(write!(f, " chasgn[29]"))}
      if self.chasgn(30) != 0 { try!(write!(f, " chasgn[30]"))}
      if self.chasgn(31) != 0 { try!(write!(f, " chasgn[31]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Map Select 0"]
#[derive(PartialEq, Eq)]
pub struct Chmap0(pub u32);
impl Chmap0 {
#[doc="uDMA Channel n Source Select"]
  #[inline] pub fn chsel<I: Into<bits::R8>>(&self, index: I) -> bits::U4 {
     let index: bits::R8 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + (index << 2);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
  }
#[doc="uDMA Channel n Source Select"]
  #[inline] pub fn set_chsel<I: Into<bits::R8>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value();
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Chmap0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chmap0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chsel(0) != 0 { try!(write!(f, " chsel[0]=0x{:x}", self.chsel(0)))}
      if self.chsel(1) != 0 { try!(write!(f, " chsel[1]=0x{:x}", self.chsel(1)))}
      if self.chsel(2) != 0 { try!(write!(f, " chsel[2]=0x{:x}", self.chsel(2)))}
      if self.chsel(3) != 0 { try!(write!(f, " chsel[3]=0x{:x}", self.chsel(3)))}
      if self.chsel(4) != 0 { try!(write!(f, " chsel[4]=0x{:x}", self.chsel(4)))}
      if self.chsel(5) != 0 { try!(write!(f, " chsel[5]=0x{:x}", self.chsel(5)))}
      if self.chsel(6) != 0 { try!(write!(f, " chsel[6]=0x{:x}", self.chsel(6)))}
      if self.chsel(7) != 0 { try!(write!(f, " chsel[7]=0x{:x}", self.chsel(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Map Select 1"]
#[derive(PartialEq, Eq)]
pub struct Chmap1(pub u32);
impl Chmap1 {
#[doc="uDMA Channel n Source Select"]
  #[inline] pub fn chsel<I: Into<bits::R8>>(&self, index: I) -> bits::U4 {
     let index: bits::R8 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + (index << 2);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
  }
#[doc="uDMA Channel n Source Select"]
  #[inline] pub fn set_chsel<I: Into<bits::R8>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value();
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Chmap1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chmap1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chsel(0) != 0 { try!(write!(f, " chsel[0]=0x{:x}", self.chsel(0)))}
      if self.chsel(1) != 0 { try!(write!(f, " chsel[1]=0x{:x}", self.chsel(1)))}
      if self.chsel(2) != 0 { try!(write!(f, " chsel[2]=0x{:x}", self.chsel(2)))}
      if self.chsel(3) != 0 { try!(write!(f, " chsel[3]=0x{:x}", self.chsel(3)))}
      if self.chsel(4) != 0 { try!(write!(f, " chsel[4]=0x{:x}", self.chsel(4)))}
      if self.chsel(5) != 0 { try!(write!(f, " chsel[5]=0x{:x}", self.chsel(5)))}
      if self.chsel(6) != 0 { try!(write!(f, " chsel[6]=0x{:x}", self.chsel(6)))}
      if self.chsel(7) != 0 { try!(write!(f, " chsel[7]=0x{:x}", self.chsel(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Map Select 2"]
#[derive(PartialEq, Eq)]
pub struct Chmap2(pub u32);
impl Chmap2 {
#[doc="uDMA Channel n Source Select"]
  #[inline] pub fn chsel<I: Into<bits::R8>>(&self, index: I) -> bits::U4 {
     let index: bits::R8 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + (index << 2);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
  }
#[doc="uDMA Channel n Source Select"]
  #[inline] pub fn set_chsel<I: Into<bits::R8>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value();
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Chmap2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chmap2 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chsel(0) != 0 { try!(write!(f, " chsel[0]=0x{:x}", self.chsel(0)))}
      if self.chsel(1) != 0 { try!(write!(f, " chsel[1]=0x{:x}", self.chsel(1)))}
      if self.chsel(2) != 0 { try!(write!(f, " chsel[2]=0x{:x}", self.chsel(2)))}
      if self.chsel(3) != 0 { try!(write!(f, " chsel[3]=0x{:x}", self.chsel(3)))}
      if self.chsel(4) != 0 { try!(write!(f, " chsel[4]=0x{:x}", self.chsel(4)))}
      if self.chsel(5) != 0 { try!(write!(f, " chsel[5]=0x{:x}", self.chsel(5)))}
      if self.chsel(6) != 0 { try!(write!(f, " chsel[6]=0x{:x}", self.chsel(6)))}
      if self.chsel(7) != 0 { try!(write!(f, " chsel[7]=0x{:x}", self.chsel(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Map Select 3"]
#[derive(PartialEq, Eq)]
pub struct Chmap3(pub u32);
impl Chmap3 {
#[doc="uDMA Channel n Source Select"]
  #[inline] pub fn chsel<I: Into<bits::R8>>(&self, index: I) -> bits::U4 {
     let index: bits::R8 = index.into();
     let index: usize = index.value();
     let shift: usize = 0 + (index << 2);
     unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [3:0]
  }
#[doc="uDMA Channel n Source Select"]
  #[inline] pub fn set_chsel<I: Into<bits::R8>, V: Into<bits::U4>>(mut self, index: I, value: V) -> Self {
     let index: bits::R8 = index.into();
     let index: usize = index.value();
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     assert!(index < 8);
     let shift: usize = 0 + (index << 2);
     self.0 &= !(0xf << shift);
     self.0 |= value << shift;
     self
  }

}
impl ::core::fmt::Display for Chmap3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chmap3 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.chsel(0) != 0 { try!(write!(f, " chsel[0]=0x{:x}", self.chsel(0)))}
      if self.chsel(1) != 0 { try!(write!(f, " chsel[1]=0x{:x}", self.chsel(1)))}
      if self.chsel(2) != 0 { try!(write!(f, " chsel[2]=0x{:x}", self.chsel(2)))}
      if self.chsel(3) != 0 { try!(write!(f, " chsel[3]=0x{:x}", self.chsel(3)))}
      if self.chsel(4) != 0 { try!(write!(f, " chsel[4]=0x{:x}", self.chsel(4)))}
      if self.chsel(5) != 0 { try!(write!(f, " chsel[5]=0x{:x}", self.chsel(5)))}
      if self.chsel(6) != 0 { try!(write!(f, " chsel[6]=0x{:x}", self.chsel(6)))}
      if self.chsel(7) != 0 { try!(write!(f, " chsel[7]=0x{:x}", self.chsel(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}

#[doc="DMA Descriptor"]
pub struct Chdesc(pub [u8; 16]);

impl Chdesc {
#[doc="Read the SRCENDP register."]
   #[inline] pub fn srcendp(&self) -> Srcendp { 
      unsafe {
         Srcendp(::core::ptr::read_volatile(self.0.as_ptr().offset(0x0) as *const u32))
      }
   }
#[doc="Write the SRCENDP register."]
   #[inline] pub fn set_srcendp(&mut self, value: Srcendp) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x0) as *mut u32, value.0);
      }
      self
  }
#[doc="Modfy the SRCENDP register."]
   #[inline] pub fn with_srcendp<F: FnOnce(Srcendp) -> Srcendp>(&mut self, f: F) -> &mut Self {
      let tmp = self.srcendp();
      self.set_srcendp(f(tmp))
   }

#[doc="Read the DSTENDP register."]
   #[inline] pub fn dstendp(&self) -> Dstendp { 
      unsafe {
         Dstendp(::core::ptr::read_volatile(self.0.as_ptr().offset(0x4) as *const u32))
      }
   }
#[doc="Write the DSTENDP register."]
   #[inline] pub fn set_dstendp(&mut self, value: Dstendp) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x4) as *mut u32, value.0);
      }
      self
  }
#[doc="Modfy the DSTENDP register."]
   #[inline] pub fn with_dstendp<F: FnOnce(Dstendp) -> Dstendp>(&mut self, f: F) -> &mut Self {
      let tmp = self.dstendp();
      self.set_dstendp(f(tmp))
   }

#[doc="Read the CHCTL register."]
   #[inline] pub fn chctl(&self) -> Chctl { 
      unsafe {
         Chctl(::core::ptr::read_volatile(self.0.as_ptr().offset(0x8) as *const u32))
      }
   }
#[doc="Write the CHCTL register."]
   #[inline] pub fn set_chctl(&mut self, value: Chctl) -> &mut Self {
      unsafe {
         ::core::ptr::write_volatile(self.0.as_mut_ptr().offset(0x8) as *mut u32, value.0);
      }
      self
  }
#[doc="Modfy the CHCTL register."]
   #[inline] pub fn with_chctl<F: FnOnce(Chctl) -> Chctl>(&mut self, f: F) -> &mut Self {
      let tmp = self.chctl();
      self.set_chctl(f(tmp))
   }

}
#[doc="DMA Channel Source Address End Pointer"]
#[derive(PartialEq, Eq)]
pub struct Srcendp(pub u32);
impl Srcendp {
#[doc="Source Address End Pointer. This field points to the last address of the μDMA transfer source (inclusive). If the source address is not incrementing (the SRCINC field in the DMACHCTL register is 0x3), then this field points at the source location itself (such as a peripheral data register)."]
  #[inline] pub fn addr(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Source Address End Pointer. This field points to the last address of the μDMA transfer source (inclusive). If the source address is not incrementing (the SRCINC field in the DMACHCTL register is 0x3), then this field points at the source location itself (such as a peripheral data register)."]
  #[inline] pub fn set_addr<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Srcendp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Srcendp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Destination Address End Pointer"]
#[derive(PartialEq, Eq)]
pub struct Dstendp(pub u32);
impl Dstendp {
#[doc="Destination Address End Pointer. This field points to the last address of the μDMA transfer destination (inclusive). If the destination address is not incrementing (the DSTINC field in the DMACHCTL register is 0x3), then this field points at the destination location itself (such as a peripheral data register)."]
  #[inline] pub fn addr(&self) -> bits::U32 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
  }
#[doc="Destination Address End Pointer. This field points to the last address of the μDMA transfer destination (inclusive). If the destination address is not incrementing (the DSTINC field in the DMACHCTL register is 0x3), then this field points at the destination location itself (such as a peripheral data register)."]
  #[inline] pub fn set_addr<V: Into<bits::U32>>(mut self, value: V) -> Self {
     let value: bits::U32 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Dstendp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Dstendp {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="DMA Channel Control Word"]
#[derive(PartialEq, Eq)]
pub struct Chctl(pub u32);
impl Chctl {
#[doc="Destination Address Increment. This field configures the destination address increment. The address increment value must be equal or greater than the value of the destination size (DSTSIZE)."]
  #[inline] pub fn dstinc(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
  }
#[doc="Destination Address Increment. This field configures the destination address increment. The address increment value must be equal or greater than the value of the destination size (DSTSIZE)."]
  #[inline] pub fn set_dstinc<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Destination Data Size. This field configures the destination item data size."]
  #[inline] pub fn dstsize(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
  }
#[doc="Destination Data Size. This field configures the destination item data size."]
  #[inline] pub fn set_dstsize<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Source Address Increment. This field configures the source address increment. The address increment value must be equal or greater than the value of the source size (SRCSIZE)."]
  #[inline] pub fn srcinc(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x3) as u8) } // [27:26]
  }
#[doc="Source Address Increment. This field configures the source address increment. The address increment value must be equal or greater than the value of the source size (SRCSIZE)."]
  #[inline] pub fn set_srcinc<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 26);
     self.0 |= value << 26;
     self
  }

#[doc="Source Data Size. This field configures the source item data size."]
  #[inline] pub fn srcsize(&self) -> bits::U2 {
     unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
  }
#[doc="Source Data Size. This field configures the source item data size."]
  #[inline] pub fn set_srcsize<V: Into<bits::U2>>(mut self, value: V) -> Self {
     let value: bits::U2 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3 << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Destination Privilege Access. This bit controls the privilege access protection for destination data writes."]
  #[inline] pub fn dstproto(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
  }
#[doc="Destination Privilege Access. This bit controls the privilege access protection for destination data writes."]
  #[inline] pub fn set_dstproto<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Source Privilege Access. This bit controls the privilege access protection for source data reads."]
  #[inline] pub fn srcproto(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
  }
#[doc="Source Privilege Access. This bit controls the privilege access protection for source data reads."]
  #[inline] pub fn set_srcproto<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Arbitration Size. This field configures the number of transfers that can occur before the μDMA controller re-arbitrates."]
  #[inline] pub fn arbsize(&self) -> bits::U4 {
     unsafe { ::core::mem::transmute(((self.0 >> 14) & 0xf) as u8) } // [17:14]
  }
#[doc="Arbitration Size. This field configures the number of transfers that can occur before the μDMA controller re-arbitrates."]
  #[inline] pub fn set_arbsize<V: Into<bits::U4>>(mut self, value: V) -> Self {
     let value: bits::U4 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0xf << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Transfer Size (minus 1). This field configures the total number of items to transfer. The value of this field is 1 less than the number to transfer (value 0 means transfer 1 item). The maximum value for this 10-bit field is 1023 which represents a transfer size of 1024 items."]
  #[inline] pub fn xfersize(&self) -> bits::U10 {
     unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3ff) as u16) } // [13:4]
  }
#[doc="Transfer Size (minus 1). This field configures the total number of items to transfer. The value of this field is 1 less than the number to transfer (value 0 means transfer 1 item). The maximum value for this 10-bit field is 1023 which represents a transfer size of 1024 items."]
  #[inline] pub fn set_xfersize<V: Into<bits::U10>>(mut self, value: V) -> Self {
     let value: bits::U10 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x3ff << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Next Useburst. This field controls whether the Useburst SET[n] bit is automatically set for the last transfer of a peripheral scatter-gather operation. Normally, for the last transfer, if the number of remaining items to transfer is less than the arbitration size, the μDMA controller uses single transfers to complete the transaction. If this bit is set, then the controller uses a burst transfer to complete the last transfer."]
  #[inline] pub fn nxtuseburst(&self) -> bits::U1 {
     unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
  }
#[doc="Next Useburst. This field controls whether the Useburst SET[n] bit is automatically set for the last transfer of a peripheral scatter-gather operation. Normally, for the last transfer, if the number of remaining items to transfer is less than the arbitration size, the μDMA controller uses single transfers to complete the transaction. If this bit is set, then the controller uses a burst transfer to complete the last transfer."]
  #[inline] pub fn set_nxtuseburst<V: Into<bits::U1>>(mut self, value: V) -> Self {
     let value: bits::U1 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="μDMA Transfer Mode. This field configures the operating mode of the μDMA cycle. Refer to “Transfer Modes” on page 685 for a detailed explanation of transfer modes. Because this register is in system RAM, it has no reset value. Therefore, this field should be initialized to 0 before the channel is enabled."]
  #[inline] pub fn xfermode(&self) -> bits::U3 {
     unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
  }
#[doc="μDMA Transfer Mode. This field configures the operating mode of the μDMA cycle. Refer to “Transfer Modes” on page 685 for a detailed explanation of transfer modes. Because this register is in system RAM, it has no reset value. Therefore, this field should be initialized to 0 before the channel is enabled."]
  #[inline] pub fn set_xfermode<V: Into<bits::U3>>(mut self, value: V) -> Self {
     let value: bits::U3 = value.into();
     let value: u32 = value.into();
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Chctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Chctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dstinc() != 0 { try!(write!(f, " dstinc=0x{:x}", self.dstinc()))}
      if self.dstsize() != 0 { try!(write!(f, " dstsize=0x{:x}", self.dstsize()))}
      if self.srcinc() != 0 { try!(write!(f, " srcinc=0x{:x}", self.srcinc()))}
      if self.srcsize() != 0 { try!(write!(f, " srcsize=0x{:x}", self.srcsize()))}
      if self.dstproto() != 0 { try!(write!(f, " dstproto"))}
      if self.srcproto() != 0 { try!(write!(f, " srcproto"))}
      if self.arbsize() != 0 { try!(write!(f, " arbsize=0x{:x}", self.arbsize()))}
      if self.xfersize() != 0 { try!(write!(f, " xfersize=0x{:x}", self.xfersize()))}
      if self.nxtuseburst() != 0 { try!(write!(f, " nxtuseburst"))}
      if self.xfermode() != 0 { try!(write!(f, " xfermode=0x{:x}", self.xfermode()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(Clone, Copy, PartialEq)]
#[doc="UDMA Channel"]
pub struct Channel<P, T> { pub periph: Periph<T>, pub index: usize, pub id: P }

impl<P,T> Channel<P,T> {
   #[inline] pub fn periph(&self) -> &Periph<T> { &self.periph }
   #[inline] pub fn index(&self) -> usize { self.index }
}

pub const UDMA_CH0: Channel<UdmaCh0Id, UdmaId> = Channel { periph: UDMA, index: 0, id: UdmaCh0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh0Id {}
pub type UdmaCh0 = Channel<UdmaCh0Id, UdmaId>;

pub const UDMA_CH1: Channel<UdmaCh1Id, UdmaId> = Channel { periph: UDMA, index: 1, id: UdmaCh1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh1Id {}
pub type UdmaCh1 = Channel<UdmaCh1Id, UdmaId>;

pub const UDMA_CH2: Channel<UdmaCh2Id, UdmaId> = Channel { periph: UDMA, index: 2, id: UdmaCh2Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh2Id {}
pub type UdmaCh2 = Channel<UdmaCh2Id, UdmaId>;

pub const UDMA_CH3: Channel<UdmaCh3Id, UdmaId> = Channel { periph: UDMA, index: 3, id: UdmaCh3Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh3Id {}
pub type UdmaCh3 = Channel<UdmaCh3Id, UdmaId>;

pub const UDMA_CH4: Channel<UdmaCh4Id, UdmaId> = Channel { periph: UDMA, index: 4, id: UdmaCh4Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh4Id {}
pub type UdmaCh4 = Channel<UdmaCh4Id, UdmaId>;

pub const UDMA_CH5: Channel<UdmaCh5Id, UdmaId> = Channel { periph: UDMA, index: 5, id: UdmaCh5Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh5Id {}
pub type UdmaCh5 = Channel<UdmaCh5Id, UdmaId>;

pub const UDMA_CH6: Channel<UdmaCh6Id, UdmaId> = Channel { periph: UDMA, index: 6, id: UdmaCh6Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh6Id {}
pub type UdmaCh6 = Channel<UdmaCh6Id, UdmaId>;

pub const UDMA_CH7: Channel<UdmaCh7Id, UdmaId> = Channel { periph: UDMA, index: 7, id: UdmaCh7Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh7Id {}
pub type UdmaCh7 = Channel<UdmaCh7Id, UdmaId>;

pub const UDMA_CH8: Channel<UdmaCh8Id, UdmaId> = Channel { periph: UDMA, index: 8, id: UdmaCh8Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh8Id {}
pub type UdmaCh8 = Channel<UdmaCh8Id, UdmaId>;

pub const UDMA_CH9: Channel<UdmaCh9Id, UdmaId> = Channel { periph: UDMA, index: 9, id: UdmaCh9Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh9Id {}
pub type UdmaCh9 = Channel<UdmaCh9Id, UdmaId>;

pub const UDMA_CH10: Channel<UdmaCh10Id, UdmaId> = Channel { periph: UDMA, index: 10, id: UdmaCh10Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh10Id {}
pub type UdmaCh10 = Channel<UdmaCh10Id, UdmaId>;

pub const UDMA_CH11: Channel<UdmaCh11Id, UdmaId> = Channel { periph: UDMA, index: 11, id: UdmaCh11Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh11Id {}
pub type UdmaCh11 = Channel<UdmaCh11Id, UdmaId>;

pub const UDMA_CH12: Channel<UdmaCh12Id, UdmaId> = Channel { periph: UDMA, index: 12, id: UdmaCh12Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh12Id {}
pub type UdmaCh12 = Channel<UdmaCh12Id, UdmaId>;

pub const UDMA_CH13: Channel<UdmaCh13Id, UdmaId> = Channel { periph: UDMA, index: 13, id: UdmaCh13Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh13Id {}
pub type UdmaCh13 = Channel<UdmaCh13Id, UdmaId>;

pub const UDMA_CH14: Channel<UdmaCh14Id, UdmaId> = Channel { periph: UDMA, index: 14, id: UdmaCh14Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh14Id {}
pub type UdmaCh14 = Channel<UdmaCh14Id, UdmaId>;

pub const UDMA_CH15: Channel<UdmaCh15Id, UdmaId> = Channel { periph: UDMA, index: 15, id: UdmaCh15Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh15Id {}
pub type UdmaCh15 = Channel<UdmaCh15Id, UdmaId>;

pub const UDMA_CH16: Channel<UdmaCh16Id, UdmaId> = Channel { periph: UDMA, index: 16, id: UdmaCh16Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh16Id {}
pub type UdmaCh16 = Channel<UdmaCh16Id, UdmaId>;

pub const UDMA_CH17: Channel<UdmaCh17Id, UdmaId> = Channel { periph: UDMA, index: 17, id: UdmaCh17Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh17Id {}
pub type UdmaCh17 = Channel<UdmaCh17Id, UdmaId>;

pub const UDMA_CH18: Channel<UdmaCh18Id, UdmaId> = Channel { periph: UDMA, index: 18, id: UdmaCh18Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh18Id {}
pub type UdmaCh18 = Channel<UdmaCh18Id, UdmaId>;

pub const UDMA_CH19: Channel<UdmaCh19Id, UdmaId> = Channel { periph: UDMA, index: 19, id: UdmaCh19Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh19Id {}
pub type UdmaCh19 = Channel<UdmaCh19Id, UdmaId>;

pub const UDMA_CH20: Channel<UdmaCh20Id, UdmaId> = Channel { periph: UDMA, index: 20, id: UdmaCh20Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh20Id {}
pub type UdmaCh20 = Channel<UdmaCh20Id, UdmaId>;

pub const UDMA_CH21: Channel<UdmaCh21Id, UdmaId> = Channel { periph: UDMA, index: 21, id: UdmaCh21Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh21Id {}
pub type UdmaCh21 = Channel<UdmaCh21Id, UdmaId>;

pub const UDMA_CH22: Channel<UdmaCh22Id, UdmaId> = Channel { periph: UDMA, index: 22, id: UdmaCh22Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh22Id {}
pub type UdmaCh22 = Channel<UdmaCh22Id, UdmaId>;

pub const UDMA_CH23: Channel<UdmaCh23Id, UdmaId> = Channel { periph: UDMA, index: 23, id: UdmaCh23Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh23Id {}
pub type UdmaCh23 = Channel<UdmaCh23Id, UdmaId>;

pub const UDMA_CH24: Channel<UdmaCh24Id, UdmaId> = Channel { periph: UDMA, index: 24, id: UdmaCh24Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh24Id {}
pub type UdmaCh24 = Channel<UdmaCh24Id, UdmaId>;

pub const UDMA_CH25: Channel<UdmaCh25Id, UdmaId> = Channel { periph: UDMA, index: 25, id: UdmaCh25Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh25Id {}
pub type UdmaCh25 = Channel<UdmaCh25Id, UdmaId>;

pub const UDMA_CH26: Channel<UdmaCh26Id, UdmaId> = Channel { periph: UDMA, index: 26, id: UdmaCh26Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh26Id {}
pub type UdmaCh26 = Channel<UdmaCh26Id, UdmaId>;

pub const UDMA_CH27: Channel<UdmaCh27Id, UdmaId> = Channel { periph: UDMA, index: 27, id: UdmaCh27Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh27Id {}
pub type UdmaCh27 = Channel<UdmaCh27Id, UdmaId>;

pub const UDMA_CH28: Channel<UdmaCh28Id, UdmaId> = Channel { periph: UDMA, index: 28, id: UdmaCh28Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh28Id {}
pub type UdmaCh28 = Channel<UdmaCh28Id, UdmaId>;

pub const UDMA_CH29: Channel<UdmaCh29Id, UdmaId> = Channel { periph: UDMA, index: 29, id: UdmaCh29Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh29Id {}
pub type UdmaCh29 = Channel<UdmaCh29Id, UdmaId>;

pub const UDMA_CH30: Channel<UdmaCh30Id, UdmaId> = Channel { periph: UDMA, index: 30, id: UdmaCh30Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh30Id {}
pub type UdmaCh30 = Channel<UdmaCh30Id, UdmaId>;

pub const UDMA_CH31: Channel<UdmaCh31Id, UdmaId> = Channel { periph: UDMA, index: 31, id: UdmaCh31Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct UdmaCh31Id {}
pub type UdmaCh31 = Channel<UdmaCh31Id, UdmaId>;
