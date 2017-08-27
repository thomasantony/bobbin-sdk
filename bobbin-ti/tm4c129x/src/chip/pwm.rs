#[allow(unused_imports)] use bobbin_common::*;

periph!(PwmPeriph, PWM0, Pwm0, 0x40028000);

impl super::sig::Signal<super::sig::M0pwm0> for Pwm0Ch0 {}
impl super::sig::SignalPwm<super::sig::M0pwm0> for Pwm0Ch0 {}
impl super::sig::Signal<super::sig::M0pwm1> for Pwm0Ch1 {}
impl super::sig::SignalPwm<super::sig::M0pwm1> for Pwm0Ch1 {}
impl super::sig::Signal<super::sig::M0pwm2> for Pwm0Ch2 {}
impl super::sig::SignalPwm<super::sig::M0pwm2> for Pwm0Ch2 {}
impl super::sig::Signal<super::sig::M0pwm3> for Pwm0Ch3 {}
impl super::sig::SignalPwm<super::sig::M0pwm3> for Pwm0Ch3 {}


pub trait PwmPeriph : Base {
#[doc="Get the *const pointer for the CTL register."]
   #[inline] fn ctl_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Get the *mut pointer for the CTL register."]
   #[inline] fn ctl_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x0)
   }
#[doc="Read the CTL register."]
   #[inline] fn ctl(&self) -> Ctl { 
      unsafe {
         Ctl(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      }
   }
#[doc="Write the CTL register."]
   #[inline] fn set_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
      let value = f(Ctl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CTL register."]
   #[inline] fn with_ctl<F: FnOnce(Ctl) -> Ctl>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ctl(::core::ptr::read_volatile((self.base() + 0x0) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the SYNC register."]
   #[inline] fn sync_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Get the *mut pointer for the SYNC register."]
   #[inline] fn sync_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x4)
   }
#[doc="Read the SYNC register."]
   #[inline] fn sync(&self) -> Sync { 
      unsafe {
         Sync(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      }
   }
#[doc="Write the SYNC register."]
   #[inline] fn set_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
      let value = f(Sync(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the SYNC register."]
   #[inline] fn with_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Sync(::core::ptr::read_volatile((self.base() + 0x4) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ENABLE register."]
   #[inline] fn enable_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Get the *mut pointer for the ENABLE register."]
   #[inline] fn enable_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x8)
   }
#[doc="Read the ENABLE register."]
   #[inline] fn enable(&self) -> Enable { 
      unsafe {
         Enable(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      }
   }
#[doc="Write the ENABLE register."]
   #[inline] fn set_enable<F: FnOnce(Enable) -> Enable>(&self, f: F) -> &Self {
      let value = f(Enable(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ENABLE register."]
   #[inline] fn with_enable<F: FnOnce(Enable) -> Enable>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Enable(::core::ptr::read_volatile((self.base() + 0x8) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x8) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INVERT register."]
   #[inline] fn invert_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Get the *mut pointer for the INVERT register."]
   #[inline] fn invert_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xc)
   }
#[doc="Read the INVERT register."]
   #[inline] fn invert(&self) -> Invert { 
      unsafe {
         Invert(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      }
   }
#[doc="Write the INVERT register."]
   #[inline] fn set_invert<F: FnOnce(Invert) -> Invert>(&self, f: F) -> &Self {
      let value = f(Invert(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the INVERT register."]
   #[inline] fn with_invert<F: FnOnce(Invert) -> Invert>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Invert(::core::ptr::read_volatile((self.base() + 0xc) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xc) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FAULT register."]
   #[inline] fn fault_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Get the *mut pointer for the FAULT register."]
   #[inline] fn fault_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x10)
   }
#[doc="Read the FAULT register."]
   #[inline] fn fault(&self) -> Fault { 
      unsafe {
         Fault(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      }
   }
#[doc="Write the FAULT register."]
   #[inline] fn set_fault<F: FnOnce(Fault) -> Fault>(&self, f: F) -> &Self {
      let value = f(Fault(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FAULT register."]
   #[inline] fn with_fault<F: FnOnce(Fault) -> Fault>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Fault(::core::ptr::read_volatile((self.base() + 0x10) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x10) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the INTEN register."]
   #[inline] fn inten_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Get the *mut pointer for the INTEN register."]
   #[inline] fn inten_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x14)
   }
#[doc="Read the INTEN register."]
   #[inline] fn inten(&self) -> Inten { 
      unsafe {
         Inten(::core::ptr::read_volatile((self.base() + 0x14) as *const u32))
      }
   }
#[doc="Write the INTEN register."]
   #[inline] fn set_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
      let value = f(Inten(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the INTEN register."]
   #[inline] fn with_inten<F: FnOnce(Inten) -> Inten>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Inten(::core::ptr::read_volatile((self.base() + 0x14) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x14) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the RIS register."]
   #[inline] fn ris_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x18)
   }
#[doc="Get the *mut pointer for the RIS register."]
   #[inline] fn ris_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x18)
   }
#[doc="Read the RIS register."]
   #[inline] fn ris(&self) -> Ris { 
      unsafe {
         Ris(::core::ptr::read_volatile((self.base() + 0x18) as *const u32))
      }
   }
#[doc="Write the RIS register."]
   #[inline] fn set_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
      let value = f(Ris(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the RIS register."]
   #[inline] fn with_ris<F: FnOnce(Ris) -> Ris>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Ris(::core::ptr::read_volatile((self.base() + 0x18) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x18) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ISC register."]
   #[inline] fn isc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x1c)
   }
#[doc="Get the *mut pointer for the ISC register."]
   #[inline] fn isc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x1c)
   }
#[doc="Read the ISC register."]
   #[inline] fn isc(&self) -> Isc { 
      unsafe {
         Isc(::core::ptr::read_volatile((self.base() + 0x1c) as *const u32))
      }
   }
#[doc="Write the ISC register."]
   #[inline] fn set_isc<F: FnOnce(Isc) -> Isc>(&self, f: F) -> &Self {
      let value = f(Isc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1c) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ISC register."]
   #[inline] fn with_isc<F: FnOnce(Isc) -> Isc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Isc(::core::ptr::read_volatile((self.base() + 0x1c) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x1c) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the STATUS register."]
   #[inline] fn status_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x20)
   }
#[doc="Get the *mut pointer for the STATUS register."]
   #[inline] fn status_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x20)
   }
#[doc="Read the STATUS register."]
   #[inline] fn status(&self) -> Status { 
      unsafe {
         Status(::core::ptr::read_volatile((self.base() + 0x20) as *const u32))
      }
   }
#[doc="Write the STATUS register."]
   #[inline] fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let value = f(Status(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x20) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the STATUS register."]
   #[inline] fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Status(::core::ptr::read_volatile((self.base() + 0x20) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x20) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the FAULTVAL register."]
   #[inline] fn faultval_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x24)
   }
#[doc="Get the *mut pointer for the FAULTVAL register."]
   #[inline] fn faultval_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x24)
   }
#[doc="Read the FAULTVAL register."]
   #[inline] fn faultval(&self) -> Faultval { 
      unsafe {
         Faultval(::core::ptr::read_volatile((self.base() + 0x24) as *const u32))
      }
   }
#[doc="Write the FAULTVAL register."]
   #[inline] fn set_faultval<F: FnOnce(Faultval) -> Faultval>(&self, f: F) -> &Self {
      let value = f(Faultval(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x24) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the FAULTVAL register."]
   #[inline] fn with_faultval<F: FnOnce(Faultval) -> Faultval>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Faultval(::core::ptr::read_volatile((self.base() + 0x24) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x24) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the ENUPD register."]
   #[inline] fn enupd_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x28)
   }
#[doc="Get the *mut pointer for the ENUPD register."]
   #[inline] fn enupd_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x28)
   }
#[doc="Read the ENUPD register."]
   #[inline] fn enupd(&self) -> Enupd { 
      unsafe {
         Enupd(::core::ptr::read_volatile((self.base() + 0x28) as *const u32))
      }
   }
#[doc="Write the ENUPD register."]
   #[inline] fn set_enupd<F: FnOnce(Enupd) -> Enupd>(&self, f: F) -> &Self {
      let value = f(Enupd(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x28) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the ENUPD register."]
   #[inline] fn with_enupd<F: FnOnce(Enupd) -> Enupd>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Enupd(::core::ptr::read_volatile((self.base() + 0x28) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x28) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_CTL register."]
   #[inline] fn ch_ctl_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x40 + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_CTL register."]
   #[inline] fn ch_ctl_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x40 + (index * 64))
   }
#[doc="Read the CH_CTL register."]
   #[inline] fn ch_ctl<I: Into<bits::R4>>(&self, index: I) -> ChCtl { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChCtl(::core::ptr::read_volatile((self.base() + 0x40 + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_CTL register."]
   #[inline] fn set_ch_ctl<I: Into<bits::R4>, F: FnOnce(ChCtl) -> ChCtl>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChCtl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x40 + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_CTL register."]
   #[inline] fn with_ch_ctl<I: Into<bits::R4> + Copy, F: FnOnce(ChCtl) -> ChCtl>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChCtl(::core::ptr::read_volatile((self.base() + 0x40 + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x40 + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_INTEN register."]
   #[inline] fn ch_inten_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x44 + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_INTEN register."]
   #[inline] fn ch_inten_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x44 + (index * 64))
   }
#[doc="Read the CH_INTEN register."]
   #[inline] fn ch_inten<I: Into<bits::R4>>(&self, index: I) -> ChInten { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChInten(::core::ptr::read_volatile((self.base() + 0x44 + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_INTEN register."]
   #[inline] fn set_ch_inten<I: Into<bits::R4>, F: FnOnce(ChInten) -> ChInten>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChInten(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x44 + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_INTEN register."]
   #[inline] fn with_ch_inten<I: Into<bits::R4> + Copy, F: FnOnce(ChInten) -> ChInten>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChInten(::core::ptr::read_volatile((self.base() + 0x44 + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x44 + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_RIS register."]
   #[inline] fn ch_ris_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x48 + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_RIS register."]
   #[inline] fn ch_ris_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x48 + (index * 64))
   }
#[doc="Read the CH_RIS register."]
   #[inline] fn ch_ris<I: Into<bits::R4>>(&self, index: I) -> ChRis { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChRis(::core::ptr::read_volatile((self.base() + 0x48 + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_RIS register."]
   #[inline] fn set_ch_ris<I: Into<bits::R4>, F: FnOnce(ChRis) -> ChRis>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChRis(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x48 + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_RIS register."]
   #[inline] fn with_ch_ris<I: Into<bits::R4> + Copy, F: FnOnce(ChRis) -> ChRis>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChRis(::core::ptr::read_volatile((self.base() + 0x48 + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x48 + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_ISC register."]
   #[inline] fn ch_isc_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x4c + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_ISC register."]
   #[inline] fn ch_isc_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x4c + (index * 64))
   }
#[doc="Read the CH_ISC register."]
   #[inline] fn ch_isc<I: Into<bits::R4>>(&self, index: I) -> ChIsc { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChIsc(::core::ptr::read_volatile((self.base() + 0x4c + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_ISC register."]
   #[inline] fn set_ch_isc<I: Into<bits::R4>, F: FnOnce(ChIsc) -> ChIsc>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChIsc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4c + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_ISC register."]
   #[inline] fn with_ch_isc<I: Into<bits::R4> + Copy, F: FnOnce(ChIsc) -> ChIsc>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChIsc(::core::ptr::read_volatile((self.base() + 0x4c + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x4c + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_LOAD register."]
   #[inline] fn ch_load_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x50 + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_LOAD register."]
   #[inline] fn ch_load_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x50 + (index * 64))
   }
#[doc="Read the CH_LOAD register."]
   #[inline] fn ch_load<I: Into<bits::R4>>(&self, index: I) -> ChLoad { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChLoad(::core::ptr::read_volatile((self.base() + 0x50 + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_LOAD register."]
   #[inline] fn set_ch_load<I: Into<bits::R4>, F: FnOnce(ChLoad) -> ChLoad>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChLoad(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x50 + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_LOAD register."]
   #[inline] fn with_ch_load<I: Into<bits::R4> + Copy, F: FnOnce(ChLoad) -> ChLoad>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChLoad(::core::ptr::read_volatile((self.base() + 0x50 + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x50 + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_COUNT register."]
   #[inline] fn ch_count_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x54 + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_COUNT register."]
   #[inline] fn ch_count_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x54 + (index * 64))
   }
#[doc="Read the CH_COUNT register."]
   #[inline] fn ch_count<I: Into<bits::R4>>(&self, index: I) -> ChCount { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChCount(::core::ptr::read_volatile((self.base() + 0x54 + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_COUNT register."]
   #[inline] fn set_ch_count<I: Into<bits::R4>, F: FnOnce(ChCount) -> ChCount>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChCount(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x54 + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_COUNT register."]
   #[inline] fn with_ch_count<I: Into<bits::R4> + Copy, F: FnOnce(ChCount) -> ChCount>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChCount(::core::ptr::read_volatile((self.base() + 0x54 + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x54 + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_CMPA register."]
   #[inline] fn ch_cmpa_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x58 + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_CMPA register."]
   #[inline] fn ch_cmpa_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x58 + (index * 64))
   }
#[doc="Read the CH_CMPA register."]
   #[inline] fn ch_cmpa<I: Into<bits::R4>>(&self, index: I) -> ChCmpa { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChCmpa(::core::ptr::read_volatile((self.base() + 0x58 + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_CMPA register."]
   #[inline] fn set_ch_cmpa<I: Into<bits::R4>, F: FnOnce(ChCmpa) -> ChCmpa>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChCmpa(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x58 + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_CMPA register."]
   #[inline] fn with_ch_cmpa<I: Into<bits::R4> + Copy, F: FnOnce(ChCmpa) -> ChCmpa>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChCmpa(::core::ptr::read_volatile((self.base() + 0x58 + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x58 + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_CMPB register."]
   #[inline] fn ch_cmpb_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x5c + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_CMPB register."]
   #[inline] fn ch_cmpb_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x5c + (index * 64))
   }
#[doc="Read the CH_CMPB register."]
   #[inline] fn ch_cmpb<I: Into<bits::R4>>(&self, index: I) -> ChCmpb { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChCmpb(::core::ptr::read_volatile((self.base() + 0x5c + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_CMPB register."]
   #[inline] fn set_ch_cmpb<I: Into<bits::R4>, F: FnOnce(ChCmpb) -> ChCmpb>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChCmpb(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x5c + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_CMPB register."]
   #[inline] fn with_ch_cmpb<I: Into<bits::R4> + Copy, F: FnOnce(ChCmpb) -> ChCmpb>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChCmpb(::core::ptr::read_volatile((self.base() + 0x5c + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x5c + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_GENA register."]
   #[inline] fn ch_gena_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x60 + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_GENA register."]
   #[inline] fn ch_gena_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x60 + (index * 64))
   }
#[doc="Read the CH_GENA register."]
   #[inline] fn ch_gena<I: Into<bits::R4>>(&self, index: I) -> ChGena { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChGena(::core::ptr::read_volatile((self.base() + 0x60 + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_GENA register."]
   #[inline] fn set_ch_gena<I: Into<bits::R4>, F: FnOnce(ChGena) -> ChGena>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChGena(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x60 + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_GENA register."]
   #[inline] fn with_ch_gena<I: Into<bits::R4> + Copy, F: FnOnce(ChGena) -> ChGena>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChGena(::core::ptr::read_volatile((self.base() + 0x60 + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x60 + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_GENB register."]
   #[inline] fn ch_genb_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x64 + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_GENB register."]
   #[inline] fn ch_genb_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x64 + (index * 64))
   }
#[doc="Read the CH_GENB register."]
   #[inline] fn ch_genb<I: Into<bits::R4>>(&self, index: I) -> ChGenb { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChGenb(::core::ptr::read_volatile((self.base() + 0x64 + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_GENB register."]
   #[inline] fn set_ch_genb<I: Into<bits::R4>, F: FnOnce(ChGenb) -> ChGenb>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChGenb(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x64 + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_GENB register."]
   #[inline] fn with_ch_genb<I: Into<bits::R4> + Copy, F: FnOnce(ChGenb) -> ChGenb>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChGenb(::core::ptr::read_volatile((self.base() + 0x64 + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x64 + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_DBCTL register."]
   #[inline] fn ch_dbctl_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x68 + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_DBCTL register."]
   #[inline] fn ch_dbctl_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x68 + (index * 64))
   }
#[doc="Read the CH_DBCTL register."]
   #[inline] fn ch_dbctl<I: Into<bits::R4>>(&self, index: I) -> ChDbctl { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChDbctl(::core::ptr::read_volatile((self.base() + 0x68 + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_DBCTL register."]
   #[inline] fn set_ch_dbctl<I: Into<bits::R4>, F: FnOnce(ChDbctl) -> ChDbctl>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChDbctl(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x68 + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_DBCTL register."]
   #[inline] fn with_ch_dbctl<I: Into<bits::R4> + Copy, F: FnOnce(ChDbctl) -> ChDbctl>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChDbctl(::core::ptr::read_volatile((self.base() + 0x68 + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x68 + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_DBRISE register."]
   #[inline] fn ch_dbrise_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x6c + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_DBRISE register."]
   #[inline] fn ch_dbrise_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x6c + (index * 64))
   }
#[doc="Read the CH_DBRISE register."]
   #[inline] fn ch_dbrise<I: Into<bits::R4>>(&self, index: I) -> ChDbrise { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChDbrise(::core::ptr::read_volatile((self.base() + 0x6c + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_DBRISE register."]
   #[inline] fn set_ch_dbrise<I: Into<bits::R4>, F: FnOnce(ChDbrise) -> ChDbrise>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChDbrise(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x6c + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_DBRISE register."]
   #[inline] fn with_ch_dbrise<I: Into<bits::R4> + Copy, F: FnOnce(ChDbrise) -> ChDbrise>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChDbrise(::core::ptr::read_volatile((self.base() + 0x6c + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x6c + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_DBFALL register."]
   #[inline] fn ch_dbfall_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x70 + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_DBFALL register."]
   #[inline] fn ch_dbfall_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x70 + (index * 64))
   }
#[doc="Read the CH_DBFALL register."]
   #[inline] fn ch_dbfall<I: Into<bits::R4>>(&self, index: I) -> ChDbfall { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChDbfall(::core::ptr::read_volatile((self.base() + 0x70 + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_DBFALL register."]
   #[inline] fn set_ch_dbfall<I: Into<bits::R4>, F: FnOnce(ChDbfall) -> ChDbfall>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChDbfall(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x70 + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_DBFALL register."]
   #[inline] fn with_ch_dbfall<I: Into<bits::R4> + Copy, F: FnOnce(ChDbfall) -> ChDbfall>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChDbfall(::core::ptr::read_volatile((self.base() + 0x70 + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x70 + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_FLTSRC0 register."]
   #[inline] fn ch_fltsrc0_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x74 + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_FLTSRC0 register."]
   #[inline] fn ch_fltsrc0_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x74 + (index * 64))
   }
#[doc="Read the CH_FLTSRC0 register."]
   #[inline] fn ch_fltsrc0<I: Into<bits::R4>>(&self, index: I) -> ChFltsrc0 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChFltsrc0(::core::ptr::read_volatile((self.base() + 0x74 + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_FLTSRC0 register."]
   #[inline] fn set_ch_fltsrc0<I: Into<bits::R4>, F: FnOnce(ChFltsrc0) -> ChFltsrc0>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChFltsrc0(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x74 + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_FLTSRC0 register."]
   #[inline] fn with_ch_fltsrc0<I: Into<bits::R4> + Copy, F: FnOnce(ChFltsrc0) -> ChFltsrc0>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChFltsrc0(::core::ptr::read_volatile((self.base() + 0x74 + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x74 + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_FLTSRC1 register."]
   #[inline] fn ch_fltsrc1_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x78 + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_FLTSRC1 register."]
   #[inline] fn ch_fltsrc1_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x78 + (index * 64))
   }
#[doc="Read the CH_FLTSRC1 register."]
   #[inline] fn ch_fltsrc1<I: Into<bits::R4>>(&self, index: I) -> ChFltsrc1 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChFltsrc1(::core::ptr::read_volatile((self.base() + 0x78 + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_FLTSRC1 register."]
   #[inline] fn set_ch_fltsrc1<I: Into<bits::R4>, F: FnOnce(ChFltsrc1) -> ChFltsrc1>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChFltsrc1(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x78 + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_FLTSRC1 register."]
   #[inline] fn with_ch_fltsrc1<I: Into<bits::R4> + Copy, F: FnOnce(ChFltsrc1) -> ChFltsrc1>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChFltsrc1(::core::ptr::read_volatile((self.base() + 0x78 + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x78 + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_MINFLTPER register."]
   #[inline] fn ch_minfltper_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x7c + (index * 64))
   }
#[doc="Get the *mut pointer for the CH_MINFLTPER register."]
   #[inline] fn ch_minfltper_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x7c + (index * 64))
   }
#[doc="Read the CH_MINFLTPER register."]
   #[inline] fn ch_minfltper<I: Into<bits::R4>>(&self, index: I) -> ChMinfltper { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChMinfltper(::core::ptr::read_volatile((self.base() + 0x7c + (index * 64)) as *const u32))
      }
   }
#[doc="Write the CH_MINFLTPER register."]
   #[inline] fn set_ch_minfltper<I: Into<bits::R4>, F: FnOnce(ChMinfltper) -> ChMinfltper>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChMinfltper(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x7c + (index * 64)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_MINFLTPER register."]
   #[inline] fn with_ch_minfltper<I: Into<bits::R4> + Copy, F: FnOnce(ChMinfltper) -> ChMinfltper>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChMinfltper(::core::ptr::read_volatile((self.base() + 0x7c + (index * 64)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x7c + (index * 64)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_FLTSEN register."]
   #[inline] fn ch_fltsen_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x800 + (index * 128))
   }
#[doc="Get the *mut pointer for the CH_FLTSEN register."]
   #[inline] fn ch_fltsen_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x800 + (index * 128))
   }
#[doc="Read the CH_FLTSEN register."]
   #[inline] fn ch_fltsen<I: Into<bits::R4>>(&self, index: I) -> ChFltsen { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChFltsen(::core::ptr::read_volatile((self.base() + 0x800 + (index * 128)) as *const u32))
      }
   }
#[doc="Write the CH_FLTSEN register."]
   #[inline] fn set_ch_fltsen<I: Into<bits::R4>, F: FnOnce(ChFltsen) -> ChFltsen>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let value = f(ChFltsen(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x800 + (index * 128)) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CH_FLTSEN register."]
   #[inline] fn with_ch_fltsen<I: Into<bits::R4> + Copy, F: FnOnce(ChFltsen) -> ChFltsen>(&self, index: I, f: F) -> &Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      let tmp = unsafe {
         ChFltsen(::core::ptr::read_volatile((self.base() + 0x800 + (index * 128)) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0x800 + (index * 128)) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CH_FLTSTAT0 register."]
   #[inline] fn ch_fltstat0_ptr<I: Into<bits::R4>>(&self, index: I) -> *const u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x804 + (index * 128))
   }
#[doc="Get the *mut pointer for the CH_FLTSTAT0 register."]
   #[inline] fn ch_fltstat0_mut<I: Into<bits::R4>>(&self, index: I) -> *mut u32 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      <Self as Base>::addr(&self, 0x804 + (index * 128))
   }
#[doc="Read the CH_FLTSTAT0 register."]
   #[inline] fn ch_fltstat0<I: Into<bits::R4>>(&self, index: I) -> ChFltstat0 { 
      let index: bits::R4 = index.into();
      let index: usize = index.value() as usize;
      unsafe {
         ChFltstat0(::core::ptr::read_volatile((self.base() + 0x804 + (index * 128)) as *const u32))
      }
   }

#[doc="Get the *const pointer for the CH_FLTSTAT1 register."]
   #[inline] fn ch_fltstat1_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0x808)
   }
#[doc="Get the *mut pointer for the CH_FLTSTAT1 register."]
   #[inline] fn ch_fltstat1_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0x808)
   }
#[doc="Read the CH_FLTSTAT1 register."]
   #[inline] fn ch_fltstat1(&self) -> ChFltstat1 { 
      unsafe {
         ChFltstat1(::core::ptr::read_volatile((self.base() + 0x808) as *const u32))
      }
   }

#[doc="Get the *const pointer for the PP register."]
   #[inline] fn pp_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xfc0)
   }
#[doc="Get the *mut pointer for the PP register."]
   #[inline] fn pp_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xfc0)
   }
#[doc="Read the PP register."]
   #[inline] fn pp(&self) -> Pp { 
      unsafe {
         Pp(::core::ptr::read_volatile((self.base() + 0xfc0) as *const u32))
      }
   }
#[doc="Write the PP register."]
   #[inline] fn set_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
      let value = f(Pp(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xfc0) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the PP register."]
   #[inline] fn with_pp<F: FnOnce(Pp) -> Pp>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Pp(::core::ptr::read_volatile((self.base() + 0xfc0) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xfc0) as *mut u32, value.0);
      }
      self
   }

#[doc="Get the *const pointer for the CC register."]
   #[inline] fn cc_ptr(&self) -> *const u32 { 
       <Self as Base>::addr(&self, 0xfc8)
   }
#[doc="Get the *mut pointer for the CC register."]
   #[inline] fn cc_mut(&self) -> *mut u32 { 
       <Self as Base>::addr(&self, 0xfc8)
   }
#[doc="Read the CC register."]
   #[inline] fn cc(&self) -> Cc { 
      unsafe {
         Cc(::core::ptr::read_volatile((self.base() + 0xfc8) as *const u32))
      }
   }
#[doc="Write the CC register."]
   #[inline] fn set_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
      let value = f(Cc(0));
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xfc8) as *mut u32, value.0);
      }
      self
   }
#[doc="Modify the CC register."]
   #[inline] fn with_cc<F: FnOnce(Cc) -> Cc>(&self, f: F) -> &Self {
      let tmp = unsafe {
         Cc(::core::ptr::read_volatile((self.base() + 0xfc8) as *const u32))
      };
      let value = f(tmp);
      unsafe {
         ::core::ptr::write_volatile((self.base() + 0xfc8) as *mut u32, value.0);
      }
      self
   }

}

#[doc="PWM Master Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ctl(pub u32);
impl Ctl {
#[doc="Update PWM Generator n"]
   #[inline] pub fn globalsync<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Update PWM Generator n"]
   #[inline] pub fn set_globalsync<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.globalsync(0) != 0 { try!(write!(f, " globalsync[0]"))}
      if self.globalsync(1) != 0 { try!(write!(f, " globalsync[1]"))}
      if self.globalsync(2) != 0 { try!(write!(f, " globalsync[2]"))}
      if self.globalsync(3) != 0 { try!(write!(f, " globalsync[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Time Base Sync"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sync(pub u32);
impl Sync {
#[doc="Reset Generator n Counter"]
   #[inline] pub fn sync<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Reset Generator n Counter"]
   #[inline] pub fn set_sync<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Sync {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Sync {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.sync(0) != 0 { try!(write!(f, " sync[0]"))}
      if self.sync(1) != 0 { try!(write!(f, " sync[1]"))}
      if self.sync(2) != 0 { try!(write!(f, " sync[2]"))}
      if self.sync(3) != 0 { try!(write!(f, " sync[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Output Enable"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Enable(pub u32);
impl Enable {
#[doc="MnPWMn Output Enable"]
   #[inline] pub fn pwmen<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="MnPWMn Output Enable"]
   #[inline] pub fn set_pwmen<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Enable {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Enable {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwmen(0) != 0 { try!(write!(f, " pwmen[0]"))}
      if self.pwmen(1) != 0 { try!(write!(f, " pwmen[1]"))}
      if self.pwmen(2) != 0 { try!(write!(f, " pwmen[2]"))}
      if self.pwmen(3) != 0 { try!(write!(f, " pwmen[3]"))}
      if self.pwmen(4) != 0 { try!(write!(f, " pwmen[4]"))}
      if self.pwmen(5) != 0 { try!(write!(f, " pwmen[5]"))}
      if self.pwmen(6) != 0 { try!(write!(f, " pwmen[6]"))}
      if self.pwmen(7) != 0 { try!(write!(f, " pwmen[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Output Inversion"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Invert(pub u32);
impl Invert {
#[doc="Invert MnPWMn Signal"]
   #[inline] pub fn pwminv<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Invert MnPWMn Signal"]
   #[inline] pub fn set_pwminv<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Invert {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Invert {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pwminv(0) != 0 { try!(write!(f, " pwminv[0]"))}
      if self.pwminv(1) != 0 { try!(write!(f, " pwminv[1]"))}
      if self.pwminv(2) != 0 { try!(write!(f, " pwminv[2]"))}
      if self.pwminv(3) != 0 { try!(write!(f, " pwminv[3]"))}
      if self.pwminv(4) != 0 { try!(write!(f, " pwminv[4]"))}
      if self.pwminv(5) != 0 { try!(write!(f, " pwminv[5]"))}
      if self.pwminv(6) != 0 { try!(write!(f, " pwminv[6]"))}
      if self.pwminv(7) != 0 { try!(write!(f, " pwminv[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Output Fault"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Fault(pub u32);
impl Fault {
#[doc="MnPWMn Fault"]
   #[inline] pub fn fault<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="MnPWMn Fault"]
   #[inline] pub fn set_fault<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Fault {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Fault {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fault(0) != 0 { try!(write!(f, " fault[0]"))}
      if self.fault(1) != 0 { try!(write!(f, " fault[1]"))}
      if self.fault(2) != 0 { try!(write!(f, " fault[2]"))}
      if self.fault(3) != 0 { try!(write!(f, " fault[3]"))}
      if self.fault(4) != 0 { try!(write!(f, " fault[4]"))}
      if self.fault(5) != 0 { try!(write!(f, " fault[5]"))}
      if self.fault(6) != 0 { try!(write!(f, " fault[6]"))}
      if self.fault(7) != 0 { try!(write!(f, " fault[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Interrupt Enable"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Inten(pub u32);
impl Inten {
#[doc="PWMn Interrupt Enable"]
   #[inline] pub fn intpwm<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="PWMn Interrupt Enable"]
   #[inline] pub fn set_intpwm<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Interrupt Fault 0"]
   #[inline] pub fn intfault<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 16 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
   }
#[doc="Interrupt Fault 0"]
   #[inline] pub fn set_intfault<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 16 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Inten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.intpwm(0) != 0 { try!(write!(f, " intpwm[0]"))}
      if self.intpwm(1) != 0 { try!(write!(f, " intpwm[1]"))}
      if self.intpwm(2) != 0 { try!(write!(f, " intpwm[2]"))}
      if self.intpwm(3) != 0 { try!(write!(f, " intpwm[3]"))}
      if self.intfault(0) != 0 { try!(write!(f, " intfault[0]"))}
      if self.intfault(1) != 0 { try!(write!(f, " intfault[1]"))}
      if self.intfault(2) != 0 { try!(write!(f, " intfault[2]"))}
      if self.intfault(3) != 0 { try!(write!(f, " intfault[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Raw Interrupt Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ris(pub u32);
impl Ris {
#[doc="PWM0 Interrupt Asserted"]
   #[inline] pub fn intpwm<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="PWM0 Interrupt Asserted"]
   #[inline] pub fn set_intpwm<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="Interrupt Fault PWM 0"]
   #[inline] pub fn intfault<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 16 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
   }
#[doc="Interrupt Fault PWM 0"]
   #[inline] pub fn set_intfault<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 16 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
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
      if self.intpwm(0) != 0 { try!(write!(f, " intpwm[0]"))}
      if self.intpwm(1) != 0 { try!(write!(f, " intpwm[1]"))}
      if self.intpwm(2) != 0 { try!(write!(f, " intpwm[2]"))}
      if self.intpwm(3) != 0 { try!(write!(f, " intpwm[3]"))}
      if self.intfault(0) != 0 { try!(write!(f, " intfault[0]"))}
      if self.intfault(1) != 0 { try!(write!(f, " intfault[1]"))}
      if self.intfault(2) != 0 { try!(write!(f, " intfault[2]"))}
      if self.intfault(3) != 0 { try!(write!(f, " intfault[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Interrupt Status and Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Isc(pub u32);
impl Isc {
#[doc="PWMn Interrupt Status"]
   #[inline] pub fn intpwm<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="PWMn Interrupt Status"]
   #[inline] pub fn set_intpwm<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

#[doc="FAULTn Interrupt Asserted"]
   #[inline] pub fn intfault<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 16 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [16]
   }
#[doc="FAULTn Interrupt Asserted"]
   #[inline] pub fn set_intfault<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 16 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Isc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.intpwm(0) != 0 { try!(write!(f, " intpwm[0]"))}
      if self.intpwm(1) != 0 { try!(write!(f, " intpwm[1]"))}
      if self.intpwm(2) != 0 { try!(write!(f, " intpwm[2]"))}
      if self.intpwm(3) != 0 { try!(write!(f, " intpwm[3]"))}
      if self.intfault(0) != 0 { try!(write!(f, " intfault[0]"))}
      if self.intfault(1) != 0 { try!(write!(f, " intfault[1]"))}
      if self.intfault(2) != 0 { try!(write!(f, " intfault[2]"))}
      if self.intfault(3) != 0 { try!(write!(f, " intfault[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
#[doc="Generator n Fault Status"]
   #[inline] pub fn fault<I: Into<bits::R4>>(&self, index: I) -> bits::U1 {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="Generator n Fault Status"]
   #[inline] pub fn set_fault<I: Into<bits::R4>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R4 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Status {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fault(0) != 0 { try!(write!(f, " fault[0]"))}
      if self.fault(1) != 0 { try!(write!(f, " fault[1]"))}
      if self.fault(2) != 0 { try!(write!(f, " fault[2]"))}
      if self.fault(3) != 0 { try!(write!(f, " fault[3]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Fault Condition Value"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Faultval(pub u32);
impl Faultval {
#[doc="MnPWM0 Fault Value"]
   #[inline] pub fn faultval<I: Into<bits::R8>>(&self, index: I) -> bits::U1 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
   }
#[doc="MnPWM0 Fault Value"]
   #[inline] pub fn set_faultval<I: Into<bits::R8>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x1 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Faultval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Faultval {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.faultval(0) != 0 { try!(write!(f, " faultval[0]"))}
      if self.faultval(1) != 0 { try!(write!(f, " faultval[1]"))}
      if self.faultval(2) != 0 { try!(write!(f, " faultval[2]"))}
      if self.faultval(3) != 0 { try!(write!(f, " faultval[3]"))}
      if self.faultval(4) != 0 { try!(write!(f, " faultval[4]"))}
      if self.faultval(5) != 0 { try!(write!(f, " faultval[5]"))}
      if self.faultval(6) != 0 { try!(write!(f, " faultval[6]"))}
      if self.faultval(7) != 0 { try!(write!(f, " faultval[7]"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Enable Update"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Enupd(pub u32);
impl Enupd {
#[doc="MnPWMn Enable Update Mode"]
   #[inline] pub fn enupd<I: Into<bits::R8>>(&self, index: I) -> bits::U2 {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let shift: usize = 0 + index;
      unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [1:0]
   }
#[doc="MnPWMn Enable Update Mode"]
   #[inline] pub fn set_enupd<I: Into<bits::R8>, V: Into<bits::U2>>(mut self, index: I, value: V) -> Self {
      let index: bits::R8 = index.into();
      let index: usize = index.value();
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      let shift: usize = 0 + index;
      self.0 &= !(0x3 << shift);
      self.0 |= value << shift;
      self
   }

}
impl ::core::fmt::Display for Enupd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Enupd {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enupd(0) != 0 { try!(write!(f, " enupd[0]=0x{:x}", self.enupd(0)))}
      if self.enupd(1) != 0 { try!(write!(f, " enupd[1]=0x{:x}", self.enupd(1)))}
      if self.enupd(2) != 0 { try!(write!(f, " enupd[2]=0x{:x}", self.enupd(2)))}
      if self.enupd(3) != 0 { try!(write!(f, " enupd[3]=0x{:x}", self.enupd(3)))}
      if self.enupd(4) != 0 { try!(write!(f, " enupd[4]=0x{:x}", self.enupd(4)))}
      if self.enupd(5) != 0 { try!(write!(f, " enupd[5]=0x{:x}", self.enupd(5)))}
      if self.enupd(6) != 0 { try!(write!(f, " enupd[6]=0x{:x}", self.enupd(6)))}
      if self.enupd(7) != 0 { try!(write!(f, " enupd[7]=0x{:x}", self.enupd(7)))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWMn Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChCtl(pub u32);
impl ChCtl {
#[doc="PWM Block Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="PWM Block Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Counter Mode"]
   #[inline] pub fn mode(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Counter Mode"]
   #[inline] pub fn set_mode<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Debug Mode"]
   #[inline] pub fn debug(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Debug Mode"]
   #[inline] pub fn set_debug<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Load Register Update Mode"]
   #[inline] pub fn loadupd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Load Register Update Mode"]
   #[inline] pub fn set_loadupd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Comparator A Update Mode"]
   #[inline] pub fn cmpaupd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Comparator A Update Mode"]
   #[inline] pub fn set_cmpaupd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Comparator B Update Mode"]
   #[inline] pub fn cmpbupd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Comparator B Update Mode"]
   #[inline] pub fn set_cmpbupd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="PWMnGENA Update Mode"]
   #[inline] pub fn genaupd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="PWMnGENA Update Mode"]
   #[inline] pub fn set_genaupd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="PWMnGENB Update Mode"]
   #[inline] pub fn genbupd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
   }
#[doc="PWMnGENB Update Mode"]
   #[inline] pub fn set_genbupd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="PWMnDBCTL Update Mode"]
   #[inline] pub fn dbctlupd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
   }
#[doc="PWMnDBCTL Update Mode"]
   #[inline] pub fn set_dbctlupd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="PWMnDBRISE Update Mode"]
   #[inline] pub fn dbriseupd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
   }
#[doc="PWMnDBRISE Update Mode"]
   #[inline] pub fn set_dbriseupd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="PWMnDBFALL Update Mode"]
   #[inline] pub fn dbfallupd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
   }
#[doc="PWMnDBFALL Update Mode"]
   #[inline] pub fn set_dbfallupd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 14);
      self.0 |= value << 14;
      self
   }

#[doc="Fault Condition Source"]
   #[inline] pub fn fltsrc(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
   }
#[doc="Fault Condition Source"]
   #[inline] pub fn set_fltsrc<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 16);
      self.0 |= value << 16;
      self
   }

#[doc="Minimum Fault Period"]
   #[inline] pub fn minfltper(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
   }
#[doc="Minimum Fault Period"]
   #[inline] pub fn set_minfltper<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 17);
      self.0 |= value << 17;
      self
   }

#[doc="Latch Fault Input"]
   #[inline] pub fn latch(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
   }
#[doc="Latch Fault Input"]
   #[inline] pub fn set_latch<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 18);
      self.0 |= value << 18;
      self
   }

}
impl ::core::fmt::Display for ChCtl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChCtl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.mode() != 0 { try!(write!(f, " mode"))}
      if self.debug() != 0 { try!(write!(f, " debug"))}
      if self.loadupd() != 0 { try!(write!(f, " loadupd"))}
      if self.cmpaupd() != 0 { try!(write!(f, " cmpaupd"))}
      if self.cmpbupd() != 0 { try!(write!(f, " cmpbupd"))}
      if self.genaupd() != 0 { try!(write!(f, " genaupd=0x{:x}", self.genaupd()))}
      if self.genbupd() != 0 { try!(write!(f, " genbupd=0x{:x}", self.genbupd()))}
      if self.dbctlupd() != 0 { try!(write!(f, " dbctlupd=0x{:x}", self.dbctlupd()))}
      if self.dbriseupd() != 0 { try!(write!(f, " dbriseupd=0x{:x}", self.dbriseupd()))}
      if self.dbfallupd() != 0 { try!(write!(f, " dbfallupd=0x{:x}", self.dbfallupd()))}
      if self.fltsrc() != 0 { try!(write!(f, " fltsrc"))}
      if self.minfltper() != 0 { try!(write!(f, " minfltper"))}
      if self.latch() != 0 { try!(write!(f, " latch"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Interrupt and Trigger Enable"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChInten(pub u32);
impl ChInten {
#[doc="Interrupt for Counter=0"]
   #[inline] pub fn intcntzero(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Interrupt for Counter=0"]
   #[inline] pub fn set_intcntzero<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Interrupt for Counter=PWMnLOAD"]
   #[inline] pub fn intcntload(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Interrupt for Counter=PWMnLOAD"]
   #[inline] pub fn set_intcntload<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Interrupt for Counter=PWMnCMPA Up"]
   #[inline] pub fn intcmpau(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Interrupt for Counter=PWMnCMPA Up"]
   #[inline] pub fn set_intcmpau<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Interrupt for Counter=PWMnCMPA Down"]
   #[inline] pub fn intcmpad(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Interrupt for Counter=PWMnCMPA Down"]
   #[inline] pub fn set_intcmpad<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Interrupt for Counter=PWMnCMPB Up"]
   #[inline] pub fn intcmpbu(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Interrupt for Counter=PWMnCMPB Up"]
   #[inline] pub fn set_intcmpbu<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Interrupt for Counter=PWMnCMPB Down"]
   #[inline] pub fn intcmpbd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Interrupt for Counter=PWMnCMPB Down"]
   #[inline] pub fn set_intcmpbd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Trigger for Counter=0"]
   #[inline] pub fn trcntzero(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Trigger for Counter=0"]
   #[inline] pub fn set_trcntzero<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Trigger for Counter=PWMnLOAD"]
   #[inline] pub fn trcntload(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Trigger for Counter=PWMnLOAD"]
   #[inline] pub fn set_trcntload<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="Trigger for Counter=PWMnCMPA Up"]
   #[inline] pub fn trcmpau(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="Trigger for Counter=PWMnCMPA Up"]
   #[inline] pub fn set_trcmpau<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
   }

#[doc="Trigger for Counter=PWMnCMPA Down"]
   #[inline] pub fn trcmpad(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
   }
#[doc="Trigger for Counter=PWMnCMPA Down"]
   #[inline] pub fn set_trcmpad<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 11);
      self.0 |= value << 11;
      self
   }

#[doc="Trigger for Counter=PWMnCMPB Up"]
   #[inline] pub fn trcmpbu(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
   }
#[doc="Trigger for Counter=PWMnCMPB Up"]
   #[inline] pub fn set_trcmpbu<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 12);
      self.0 |= value << 12;
      self
   }

#[doc="Trigger for Counter=PWMnCMPB Down"]
   #[inline] pub fn trcmpbd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
   }
#[doc="Trigger for Counter=PWMnCMPB Down"]
   #[inline] pub fn set_trcmpbd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 13);
      self.0 |= value << 13;
      self
   }

}
impl ::core::fmt::Display for ChInten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChInten {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.intcntzero() != 0 { try!(write!(f, " intcntzero"))}
      if self.intcntload() != 0 { try!(write!(f, " intcntload"))}
      if self.intcmpau() != 0 { try!(write!(f, " intcmpau"))}
      if self.intcmpad() != 0 { try!(write!(f, " intcmpad"))}
      if self.intcmpbu() != 0 { try!(write!(f, " intcmpbu"))}
      if self.intcmpbd() != 0 { try!(write!(f, " intcmpbd"))}
      if self.trcntzero() != 0 { try!(write!(f, " trcntzero"))}
      if self.trcntload() != 0 { try!(write!(f, " trcntload"))}
      if self.trcmpau() != 0 { try!(write!(f, " trcmpau"))}
      if self.trcmpad() != 0 { try!(write!(f, " trcmpad"))}
      if self.trcmpbu() != 0 { try!(write!(f, " trcmpbu"))}
      if self.trcmpbd() != 0 { try!(write!(f, " trcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Raw Interrupt Status"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChRis(pub u32);
impl ChRis {
#[doc="Counter=0 Interrupt Status"]
   #[inline] pub fn intcntzero(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter=0 Interrupt Status"]
   #[inline] pub fn set_intcntzero<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Counter=Load Interrupt Status"]
   #[inline] pub fn intcntload(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Counter=Load Interrupt Status"]
   #[inline] pub fn set_intcntload<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Comparator A Up Interrupt Status"]
   #[inline] pub fn intcmpau(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Comparator A Up Interrupt Status"]
   #[inline] pub fn set_intcmpau<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Comparator A Down Interrupt Status"]
   #[inline] pub fn intcmpad(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Comparator A Down Interrupt Status"]
   #[inline] pub fn set_intcmpad<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Comparator B Up Interrupt Status"]
   #[inline] pub fn intcmpbu(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Comparator B Up Interrupt Status"]
   #[inline] pub fn set_intcmpbu<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Comparator B Down Interrupt Status"]
   #[inline] pub fn intcmpbd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Comparator B Down Interrupt Status"]
   #[inline] pub fn set_intcmpbd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for ChRis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChRis {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.intcntzero() != 0 { try!(write!(f, " intcntzero"))}
      if self.intcntload() != 0 { try!(write!(f, " intcntload"))}
      if self.intcmpau() != 0 { try!(write!(f, " intcmpau"))}
      if self.intcmpad() != 0 { try!(write!(f, " intcmpad"))}
      if self.intcmpbu() != 0 { try!(write!(f, " intcmpbu"))}
      if self.intcmpbd() != 0 { try!(write!(f, " intcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Interrupt Status and Clear"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChIsc(pub u32);
impl ChIsc {
#[doc="Counter=0 Interrupt"]
   #[inline] pub fn intcntzero(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Counter=0 Interrupt"]
   #[inline] pub fn set_intcntzero<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Counter=Load Interrupt"]
   #[inline] pub fn intcntload(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Counter=Load Interrupt"]
   #[inline] pub fn set_intcntload<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Comparator A Up Interrupt"]
   #[inline] pub fn intcmpau(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Comparator A Up Interrupt"]
   #[inline] pub fn set_intcmpau<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Comparator A Down Interrupt"]
   #[inline] pub fn intcmpad(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Comparator A Down Interrupt"]
   #[inline] pub fn set_intcmpad<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Comparator B Up Interrupt"]
   #[inline] pub fn intcmpbu(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Comparator B Up Interrupt"]
   #[inline] pub fn set_intcmpbu<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Comparator B Down Interrupt"]
   #[inline] pub fn intcmpbd(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Comparator B Down Interrupt"]
   #[inline] pub fn set_intcmpbd<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

}
impl ::core::fmt::Display for ChIsc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChIsc {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.intcntzero() != 0 { try!(write!(f, " intcntzero"))}
      if self.intcntload() != 0 { try!(write!(f, " intcntload"))}
      if self.intcmpau() != 0 { try!(write!(f, " intcmpau"))}
      if self.intcmpad() != 0 { try!(write!(f, " intcmpad"))}
      if self.intcmpbu() != 0 { try!(write!(f, " intcmpbu"))}
      if self.intcmpbd() != 0 { try!(write!(f, " intcmpbd"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Load"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChLoad(pub u32);
impl ChLoad {
#[doc="Counter Load Value"]
   #[inline] pub fn load(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Counter Load Value"]
   #[inline] pub fn set_load<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for ChLoad {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChLoad {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.load() != 0 { try!(write!(f, " load=0x{:x}", self.load()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Counter"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChCount(pub u32);
impl ChCount {
#[doc="Counter Value"]
   #[inline] pub fn count(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Counter Value"]
   #[inline] pub fn set_count<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for ChCount {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChCount {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Compare A"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChCmpa(pub u32);
impl ChCmpa {
#[doc="Comparator A Value"]
   #[inline] pub fn cmpa(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Comparator A Value"]
   #[inline] pub fn set_cmpa<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for ChCmpa {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChCmpa {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cmpa() != 0 { try!(write!(f, " cmpa=0x{:x}", self.cmpa()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Compare B"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChCmpb(pub u32);
impl ChCmpb {
#[doc="Comparator B Value"]
   #[inline] pub fn cmpb(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Comparator B Value"]
   #[inline] pub fn set_cmpb<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for ChCmpb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChCmpb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.cmpb() != 0 { try!(write!(f, " cmpb=0x{:x}", self.cmpb()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Generator A Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChGena(pub u32);
impl ChGena {
#[doc="Action for Counter=0"]
   #[inline] pub fn actzero(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Action for Counter=0"]
   #[inline] pub fn set_actzero<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Action for Counter=LOAD"]
   #[inline] pub fn actload(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="Action for Counter=LOAD"]
   #[inline] pub fn set_actload<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Action for Comparator A Up"]
   #[inline] pub fn actcmpau(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
   }
#[doc="Action for Comparator A Up"]
   #[inline] pub fn set_actcmpau<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Action for Comparator A Down"]
   #[inline] pub fn actcmpad(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Action for Comparator A Down"]
   #[inline] pub fn set_actcmpad<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Action for Comparator B Up"]
   #[inline] pub fn actcmpbu(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
   }
#[doc="Action for Comparator B Up"]
   #[inline] pub fn set_actcmpbu<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Action for Comparator B Down"]
   #[inline] pub fn actcmpbd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
   }
#[doc="Action for Comparator B Down"]
   #[inline] pub fn set_actcmpbd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 10);
      self.0 |= value << 10;
      self
   }

}
impl ::core::fmt::Display for ChGena {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChGena {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.actzero() != 0 { try!(write!(f, " actzero=0x{:x}", self.actzero()))}
      if self.actload() != 0 { try!(write!(f, " actload=0x{:x}", self.actload()))}
      if self.actcmpau() != 0 { try!(write!(f, " actcmpau=0x{:x}", self.actcmpau()))}
      if self.actcmpad() != 0 { try!(write!(f, " actcmpad=0x{:x}", self.actcmpad()))}
      if self.actcmpbu() != 0 { try!(write!(f, " actcmpbu=0x{:x}", self.actcmpbu()))}
      if self.actcmpbd() != 0 { try!(write!(f, " actcmpbd=0x{:x}", self.actcmpbd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Generator B Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChGenb(pub u32);
impl ChGenb {
#[doc="Action for Counter=0"]
   #[inline] pub fn actzero(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
   }
#[doc="Action for Counter=0"]
   #[inline] pub fn set_actzero<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Action for Counter=LOAD"]
   #[inline] pub fn actload(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
   }
#[doc="Action for Counter=LOAD"]
   #[inline] pub fn set_actload<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Action for Comparator A Up"]
   #[inline] pub fn actcmpau(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
   }
#[doc="Action for Comparator A Up"]
   #[inline] pub fn set_actcmpau<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Action for Comparator A Down"]
   #[inline] pub fn actcmpad(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
   }
#[doc="Action for Comparator A Down"]
   #[inline] pub fn set_actcmpad<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Action for Comparator B Up"]
   #[inline] pub fn actcmpbu(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
   }
#[doc="Action for Comparator B Up"]
   #[inline] pub fn set_actcmpbu<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Action for Comparator B Down"]
   #[inline] pub fn actcmpbd(&self) -> bits::U2 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
   }
#[doc="Action for Comparator B Down"]
   #[inline] pub fn set_actcmpbd<V: Into<bits::U2>>(mut self, value: V) -> Self {
      let value: bits::U2 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x3 << 10);
      self.0 |= value << 10;
      self
   }

}
impl ::core::fmt::Display for ChGenb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChGenb {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.actzero() != 0 { try!(write!(f, " actzero=0x{:x}", self.actzero()))}
      if self.actload() != 0 { try!(write!(f, " actload=0x{:x}", self.actload()))}
      if self.actcmpau() != 0 { try!(write!(f, " actcmpau=0x{:x}", self.actcmpau()))}
      if self.actcmpad() != 0 { try!(write!(f, " actcmpad=0x{:x}", self.actcmpad()))}
      if self.actcmpbu() != 0 { try!(write!(f, " actcmpbu=0x{:x}", self.actcmpbu()))}
      if self.actcmpbd() != 0 { try!(write!(f, " actcmpbd=0x{:x}", self.actcmpbd()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Dead-Band Control"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChDbctl(pub u32);
impl ChDbctl {
#[doc="Dead-Band Generator Enable"]
   #[inline] pub fn enable(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Dead-Band Generator Enable"]
   #[inline] pub fn set_enable<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for ChDbctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChDbctl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Dead-Band Rising-Edge Delay"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChDbrise(pub u32);
impl ChDbrise {
#[doc="Dead-Band Rise Delay"]
   #[inline] pub fn delay(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="Dead-Band Rise Delay"]
   #[inline] pub fn set_delay<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for ChDbrise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChDbrise {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.delay() != 0 { try!(write!(f, " delay=0x{:x}", self.delay()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Dead-Band Falling-Edge-Delay"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChDbfall(pub u32);
impl ChDbfall {
#[doc="Dead-Band Fall Delay"]
   #[inline] pub fn delay(&self) -> bits::U12 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
   }
#[doc="Dead-Band Fall Delay"]
   #[inline] pub fn set_delay<V: Into<bits::U12>>(mut self, value: V) -> Self {
      let value: bits::U12 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xfff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for ChDbfall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChDbfall {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.delay() != 0 { try!(write!(f, " delay=0x{:x}", self.delay()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Fault Source 0"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChFltsrc0(pub u32);
impl ChFltsrc0 {
#[doc="Fault0 Input"]
   #[inline] pub fn fault0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Fault0 Input"]
   #[inline] pub fn set_fault0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Fault1 Input"]
   #[inline] pub fn fault1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Fault1 Input"]
   #[inline] pub fn set_fault1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Fault2 Input"]
   #[inline] pub fn fault2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Fault2 Input"]
   #[inline] pub fn set_fault2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Fault3 Input"]
   #[inline] pub fn fault3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Fault3 Input"]
   #[inline] pub fn set_fault3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for ChFltsrc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChFltsrc0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fault0() != 0 { try!(write!(f, " fault0"))}
      if self.fault1() != 0 { try!(write!(f, " fault1"))}
      if self.fault2() != 0 { try!(write!(f, " fault2"))}
      if self.fault3() != 0 { try!(write!(f, " fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Fault Source 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChFltsrc1(pub u32);
impl ChFltsrc1 {
#[doc="Digital Comparator 0"]
   #[inline] pub fn dcmp0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Digital Comparator 0"]
   #[inline] pub fn set_dcmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Digital Comparator 1"]
   #[inline] pub fn dcmp1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Digital Comparator 1"]
   #[inline] pub fn set_dcmp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Digital Comparator 2"]
   #[inline] pub fn dcmp2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Digital Comparator 2"]
   #[inline] pub fn set_dcmp2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Digital Comparator 3"]
   #[inline] pub fn dcmp3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Digital Comparator 3"]
   #[inline] pub fn set_dcmp3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Digital Comparator 4"]
   #[inline] pub fn dcmp4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Digital Comparator 4"]
   #[inline] pub fn set_dcmp4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Digital Comparator 5"]
   #[inline] pub fn dcmp5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Digital Comparator 5"]
   #[inline] pub fn set_dcmp5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Digital Comparator 6"]
   #[inline] pub fn dcmp6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Digital Comparator 6"]
   #[inline] pub fn set_dcmp6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Digital Comparator 7"]
   #[inline] pub fn dcmp7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Digital Comparator 7"]
   #[inline] pub fn set_dcmp7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for ChFltsrc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChFltsrc1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dcmp0() != 0 { try!(write!(f, " dcmp0"))}
      if self.dcmp1() != 0 { try!(write!(f, " dcmp1"))}
      if self.dcmp2() != 0 { try!(write!(f, " dcmp2"))}
      if self.dcmp3() != 0 { try!(write!(f, " dcmp3"))}
      if self.dcmp4() != 0 { try!(write!(f, " dcmp4"))}
      if self.dcmp5() != 0 { try!(write!(f, " dcmp5"))}
      if self.dcmp6() != 0 { try!(write!(f, " dcmp6"))}
      if self.dcmp7() != 0 { try!(write!(f, " dcmp7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Minimum Fault Period"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChMinfltper(pub u32);
impl ChMinfltper {
#[doc="Minimum Fault Period"]
   #[inline] pub fn minfltper(&self) -> bits::U16 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
   }
#[doc="Minimum Fault Period"]
   #[inline] pub fn set_minfltper<V: Into<bits::U16>>(mut self, value: V) -> Self {
      let value: bits::U16 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xffff << 0);
      self.0 |= value << 0;
      self
   }

}
impl ::core::fmt::Display for ChMinfltper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChMinfltper {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.minfltper() != 0 { try!(write!(f, " minfltper=0x{:x}", self.minfltper()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Fault Pin Logic Sense"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChFltsen(pub u32);
impl ChFltsen {
#[doc="Fault0 Sense"]
   #[inline] pub fn fault0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Fault0 Sense"]
   #[inline] pub fn set_fault0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Fault1 Sense"]
   #[inline] pub fn fault1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Fault1 Sense"]
   #[inline] pub fn set_fault1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Fault2 Sense"]
   #[inline] pub fn fault2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Fault2 Sense"]
   #[inline] pub fn set_fault2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Fault3 Sense"]
   #[inline] pub fn fault3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Fault3 Sense"]
   #[inline] pub fn set_fault3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for ChFltsen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChFltsen {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fault0() != 0 { try!(write!(f, " fault0"))}
      if self.fault1() != 0 { try!(write!(f, " fault1"))}
      if self.fault2() != 0 { try!(write!(f, " fault2"))}
      if self.fault3() != 0 { try!(write!(f, " fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Fault Status 0"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChFltstat0(pub u32);
impl ChFltstat0 {
#[doc="Fault Input 0"]
   #[inline] pub fn fault0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Fault Input 0"]
   #[inline] pub fn set_fault0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Fault Input 1"]
   #[inline] pub fn fault1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Fault Input 1"]
   #[inline] pub fn set_fault1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Fault Input 2"]
   #[inline] pub fn fault2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Fault Input 2"]
   #[inline] pub fn set_fault2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Fault Input 3"]
   #[inline] pub fn fault3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Fault Input 3"]
   #[inline] pub fn set_fault3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

}
impl ::core::fmt::Display for ChFltstat0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChFltstat0 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.fault0() != 0 { try!(write!(f, " fault0"))}
      if self.fault1() != 0 { try!(write!(f, " fault1"))}
      if self.fault2() != 0 { try!(write!(f, " fault2"))}
      if self.fault3() != 0 { try!(write!(f, " fault3"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Fault Status 1"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ChFltstat1(pub u32);
impl ChFltstat1 {
#[doc="Digital Comparator 0 Trigger"]
   #[inline] pub fn dcmp0(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
   }
#[doc="Digital Comparator 0 Trigger"]
   #[inline] pub fn set_dcmp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Digital Comparator 1 Trigger"]
   #[inline] pub fn dcmp1(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
   }
#[doc="Digital Comparator 1 Trigger"]
   #[inline] pub fn set_dcmp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 1);
      self.0 |= value << 1;
      self
   }

#[doc="Digital Comparator 2 Trigger"]
   #[inline] pub fn dcmp2(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
   }
#[doc="Digital Comparator 2 Trigger"]
   #[inline] pub fn set_dcmp2<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 2);
      self.0 |= value << 2;
      self
   }

#[doc="Digital Comparator 3 Trigger"]
   #[inline] pub fn dcmp3(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
   }
#[doc="Digital Comparator 3 Trigger"]
   #[inline] pub fn set_dcmp3<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 3);
      self.0 |= value << 3;
      self
   }

#[doc="Digital Comparator 4 Trigger"]
   #[inline] pub fn dcmp4(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
   }
#[doc="Digital Comparator 4 Trigger"]
   #[inline] pub fn set_dcmp4<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Digital Comparator 5 Trigger"]
   #[inline] pub fn dcmp5(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
   }
#[doc="Digital Comparator 5 Trigger"]
   #[inline] pub fn set_dcmp5<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 5);
      self.0 |= value << 5;
      self
   }

#[doc="Digital Comparator 6 Trigger"]
   #[inline] pub fn dcmp6(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
   }
#[doc="Digital Comparator 6 Trigger"]
   #[inline] pub fn set_dcmp6<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 6);
      self.0 |= value << 6;
      self
   }

#[doc="Digital Comparator 7 Trigger"]
   #[inline] pub fn dcmp7(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
   }
#[doc="Digital Comparator 7 Trigger"]
   #[inline] pub fn set_dcmp7<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 7);
      self.0 |= value << 7;
      self
   }

}
impl ::core::fmt::Display for ChFltstat1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for ChFltstat1 {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.dcmp0() != 0 { try!(write!(f, " dcmp0"))}
      if self.dcmp1() != 0 { try!(write!(f, " dcmp1"))}
      if self.dcmp2() != 0 { try!(write!(f, " dcmp2"))}
      if self.dcmp3() != 0 { try!(write!(f, " dcmp3"))}
      if self.dcmp4() != 0 { try!(write!(f, " dcmp4"))}
      if self.dcmp5() != 0 { try!(write!(f, " dcmp5"))}
      if self.dcmp6() != 0 { try!(write!(f, " dcmp6"))}
      if self.dcmp7() != 0 { try!(write!(f, " dcmp7"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Peripheral Properties"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pp(pub u32);
impl Pp {
#[doc="Generators"]
   #[inline] pub fn gcnt(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
   }
#[doc="Generators"]
   #[inline] pub fn set_gcnt<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Fault Inputs (per PWM unit)"]
   #[inline] pub fn fcnt(&self) -> bits::U4 {
      unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
   }
#[doc="Fault Inputs (per PWM unit)"]
   #[inline] pub fn set_fcnt<V: Into<bits::U4>>(mut self, value: V) -> Self {
      let value: bits::U4 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0xf << 4);
      self.0 |= value << 4;
      self
   }

#[doc="Extended Synchronization"]
   #[inline] pub fn esync(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Extended Synchronization"]
   #[inline] pub fn set_esync<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
   }

#[doc="Extended Fault"]
   #[inline] pub fn efault(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
   }
#[doc="Extended Fault"]
   #[inline] pub fn set_efault<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 9);
      self.0 |= value << 9;
      self
   }

#[doc="One-Shot Mode"]
   #[inline] pub fn one(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
   }
#[doc="One-Shot Mode"]
   #[inline] pub fn set_one<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 10);
      self.0 |= value << 10;
      self
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
      if self.gcnt() != 0 { try!(write!(f, " gcnt=0x{:x}", self.gcnt()))}
      if self.fcnt() != 0 { try!(write!(f, " fcnt=0x{:x}", self.fcnt()))}
      if self.esync() != 0 { try!(write!(f, " esync"))}
      if self.efault() != 0 { try!(write!(f, " efault"))}
      if self.one() != 0 { try!(write!(f, " one"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="PWM Clock Configuration"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cc(pub u32);
impl Cc {
#[doc="PWM Clock Divider"]
   #[inline] pub fn pwmdiv(&self) -> bits::U3 {
      unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
   }
#[doc="PWM Clock Divider"]
   #[inline] pub fn set_pwmdiv<V: Into<bits::U3>>(mut self, value: V) -> Self {
      let value: bits::U3 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x7 << 0);
      self.0 |= value << 0;
      self
   }

#[doc="Use PWM Clock Divisor"]
   #[inline] pub fn usepwm(&self) -> bits::U1 {
      unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
   }
#[doc="Use PWM Clock Divisor"]
   #[inline] pub fn set_usepwm<V: Into<bits::U1>>(mut self, value: V) -> Self {
      let value: bits::U1 = value.into();
      let value: u32 = value.into();
      self.0 &= !(0x1 << 8);
      self.0 |= value << 8;
      self
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
      if self.pwmdiv() != 0 { try!(write!(f, " pwmdiv=0x{:x}", self.pwmdiv()))}
      if self.usepwm() != 0 { try!(write!(f, " usepwm"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
pub trait Channel<T> {
   fn periph(&self) -> T;
   fn index(&self) -> usize;
}

channel!(PWM0_CH0, Pwm0Ch0, PWM0, Pwm0, 0);
channel!(PWM0_CH1, Pwm0Ch1, PWM0, Pwm0, 1);
channel!(PWM0_CH2, Pwm0Ch2, PWM0, Pwm0, 2);
channel!(PWM0_CH3, Pwm0Ch3, PWM0, Pwm0, 3);
