//! Real-time clock
pub const RTC: Rtc = Rtc(0x40002800);

#[doc="Real-time clock"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rtc(pub u32);
impl Rtc {
#[doc="Get the *const pointer for the TR register."]
  #[inline] pub fn tr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x0) as *const u32
  }
#[doc="Get the *mut pointer for the TR register."]
  #[inline] pub fn tr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x0) as *mut u32
  }
#[doc="Read the TR register."]
  #[inline] pub fn tr(&self) -> Tr { 
     unsafe {
        Tr(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u32))
     }
  }
#[doc="Write the TR register."]
  #[inline] pub fn set_tr(&self, value: Tr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TR register."]
  #[inline] pub fn with_tr<F: FnOnce(Tr) -> Tr>(&self, f: F) -> &Self {
     let tmp = self.tr();
     self.set_tr(f(tmp))
  }

#[doc="Get the *const pointer for the DR register."]
  #[inline] pub fn dr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x4) as *const u32
  }
#[doc="Get the *mut pointer for the DR register."]
  #[inline] pub fn dr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x4) as *mut u32
  }
#[doc="Read the DR register."]
  #[inline] pub fn dr(&self) -> Dr { 
     unsafe {
        Dr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u32))
     }
  }
#[doc="Write the DR register."]
  #[inline] pub fn set_dr(&self, value: Dr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the DR register."]
  #[inline] pub fn with_dr<F: FnOnce(Dr) -> Dr>(&self, f: F) -> &Self {
     let tmp = self.dr();
     self.set_dr(f(tmp))
  }

#[doc="Get the *const pointer for the CR register."]
  #[inline] pub fn cr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x8) as *const u32
  }
#[doc="Get the *mut pointer for the CR register."]
  #[inline] pub fn cr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x8) as *mut u32
  }
#[doc="Read the CR register."]
  #[inline] pub fn cr(&self) -> Cr { 
     unsafe {
        Cr(::core::ptr::read_volatile(((self.0 as usize) + 0x8) as *const u32))
     }
  }
#[doc="Write the CR register."]
  #[inline] pub fn set_cr(&self, value: Cr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CR register."]
  #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
     let tmp = self.cr();
     self.set_cr(f(tmp))
  }

#[doc="Get the *const pointer for the ISR register."]
  #[inline] pub fn isr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0xc) as *const u32
  }
#[doc="Get the *mut pointer for the ISR register."]
  #[inline] pub fn isr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0xc) as *mut u32
  }
#[doc="Read the ISR register."]
  #[inline] pub fn isr(&self) -> Isr { 
     unsafe {
        Isr(::core::ptr::read_volatile(((self.0 as usize) + 0xc) as *const u32))
     }
  }
#[doc="Write the ISR register."]
  #[inline] pub fn set_isr(&self, value: Isr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0xc) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ISR register."]
  #[inline] pub fn with_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
     let tmp = self.isr();
     self.set_isr(f(tmp))
  }

#[doc="Get the *const pointer for the PRER register."]
  #[inline] pub fn prer_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x10) as *const u32
  }
#[doc="Get the *mut pointer for the PRER register."]
  #[inline] pub fn prer_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x10) as *mut u32
  }
#[doc="Read the PRER register."]
  #[inline] pub fn prer(&self) -> Prer { 
     unsafe {
        Prer(::core::ptr::read_volatile(((self.0 as usize) + 0x10) as *const u32))
     }
  }
#[doc="Write the PRER register."]
  #[inline] pub fn set_prer(&self, value: Prer) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x10) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the PRER register."]
  #[inline] pub fn with_prer<F: FnOnce(Prer) -> Prer>(&self, f: F) -> &Self {
     let tmp = self.prer();
     self.set_prer(f(tmp))
  }

#[doc="Get the *const pointer for the WUTR register."]
  #[inline] pub fn wutr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x14) as *const u32
  }
#[doc="Get the *mut pointer for the WUTR register."]
  #[inline] pub fn wutr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x14) as *mut u32
  }
#[doc="Read the WUTR register."]
  #[inline] pub fn wutr(&self) -> Wutr { 
     unsafe {
        Wutr(::core::ptr::read_volatile(((self.0 as usize) + 0x14) as *const u32))
     }
  }
#[doc="Write the WUTR register."]
  #[inline] pub fn set_wutr(&self, value: Wutr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x14) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the WUTR register."]
  #[inline] pub fn with_wutr<F: FnOnce(Wutr) -> Wutr>(&self, f: F) -> &Self {
     let tmp = self.wutr();
     self.set_wutr(f(tmp))
  }

#[doc="Get the *const pointer for the ALRMAR register."]
  #[inline] pub fn alrmar_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x1c) as *const u32
  }
#[doc="Get the *mut pointer for the ALRMAR register."]
  #[inline] pub fn alrmar_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x1c) as *mut u32
  }
#[doc="Read the ALRMAR register."]
  #[inline] pub fn alrmar(&self) -> Alrmar { 
     unsafe {
        Alrmar(::core::ptr::read_volatile(((self.0 as usize) + 0x1c) as *const u32))
     }
  }
#[doc="Write the ALRMAR register."]
  #[inline] pub fn set_alrmar(&self, value: Alrmar) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ALRMAR register."]
  #[inline] pub fn with_alrmar<F: FnOnce(Alrmar) -> Alrmar>(&self, f: F) -> &Self {
     let tmp = self.alrmar();
     self.set_alrmar(f(tmp))
  }

#[doc="Get the *const pointer for the ALRMBR register."]
  #[inline] pub fn alrmbr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x20) as *const u32
  }
#[doc="Get the *mut pointer for the ALRMBR register."]
  #[inline] pub fn alrmbr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x20) as *mut u32
  }
#[doc="Read the ALRMBR register."]
  #[inline] pub fn alrmbr(&self) -> Alrmbr { 
     unsafe {
        Alrmbr(::core::ptr::read_volatile(((self.0 as usize) + 0x20) as *const u32))
     }
  }
#[doc="Write the ALRMBR register."]
  #[inline] pub fn set_alrmbr(&self, value: Alrmbr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x20) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ALRMBR register."]
  #[inline] pub fn with_alrmbr<F: FnOnce(Alrmbr) -> Alrmbr>(&self, f: F) -> &Self {
     let tmp = self.alrmbr();
     self.set_alrmbr(f(tmp))
  }

#[doc="Get the *const pointer for the WPR register."]
  #[inline] pub fn wpr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x24) as *const u32
  }
#[doc="Get the *mut pointer for the WPR register."]
  #[inline] pub fn wpr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x24) as *mut u32
  }
#[doc="Write the WPR register."]
  #[inline] pub fn set_wpr(&self, value: Wpr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x24) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the SSR register."]
  #[inline] pub fn ssr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x28) as *const u32
  }
#[doc="Get the *mut pointer for the SSR register."]
  #[inline] pub fn ssr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x28) as *mut u32
  }
#[doc="Read the SSR register."]
  #[inline] pub fn ssr(&self) -> Ssr { 
     unsafe {
        Ssr(::core::ptr::read_volatile(((self.0 as usize) + 0x28) as *const u32))
     }
  }

#[doc="Get the *const pointer for the SHIFTR register."]
  #[inline] pub fn shiftr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x2c) as *const u32
  }
#[doc="Get the *mut pointer for the SHIFTR register."]
  #[inline] pub fn shiftr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x2c) as *mut u32
  }
#[doc="Write the SHIFTR register."]
  #[inline] pub fn set_shiftr(&self, value: Shiftr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2c) as *mut u32, value.0);
     }
     self
  }

#[doc="Get the *const pointer for the TSTR register."]
  #[inline] pub fn tstr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x30) as *const u32
  }
#[doc="Get the *mut pointer for the TSTR register."]
  #[inline] pub fn tstr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x30) as *mut u32
  }
#[doc="Read the TSTR register."]
  #[inline] pub fn tstr(&self) -> Tstr { 
     unsafe {
        Tstr(::core::ptr::read_volatile(((self.0 as usize) + 0x30) as *const u32))
     }
  }

#[doc="Get the *const pointer for the TSDR register."]
  #[inline] pub fn tsdr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x34) as *const u32
  }
#[doc="Get the *mut pointer for the TSDR register."]
  #[inline] pub fn tsdr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x34) as *mut u32
  }
#[doc="Read the TSDR register."]
  #[inline] pub fn tsdr(&self) -> Tsdr { 
     unsafe {
        Tsdr(::core::ptr::read_volatile(((self.0 as usize) + 0x34) as *const u32))
     }
  }

#[doc="Get the *const pointer for the TSSSR register."]
  #[inline] pub fn tsssr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x38) as *const u32
  }
#[doc="Get the *mut pointer for the TSSSR register."]
  #[inline] pub fn tsssr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x38) as *mut u32
  }
#[doc="Read the TSSSR register."]
  #[inline] pub fn tsssr(&self) -> Tsssr { 
     unsafe {
        Tsssr(::core::ptr::read_volatile(((self.0 as usize) + 0x38) as *const u32))
     }
  }

#[doc="Get the *const pointer for the CALR register."]
  #[inline] pub fn calr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x3c) as *const u32
  }
#[doc="Get the *mut pointer for the CALR register."]
  #[inline] pub fn calr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x3c) as *mut u32
  }
#[doc="Read the CALR register."]
  #[inline] pub fn calr(&self) -> Calr { 
     unsafe {
        Calr(::core::ptr::read_volatile(((self.0 as usize) + 0x3c) as *const u32))
     }
  }
#[doc="Write the CALR register."]
  #[inline] pub fn set_calr(&self, value: Calr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x3c) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the CALR register."]
  #[inline] pub fn with_calr<F: FnOnce(Calr) -> Calr>(&self, f: F) -> &Self {
     let tmp = self.calr();
     self.set_calr(f(tmp))
  }

#[doc="Get the *const pointer for the TAFCR register."]
  #[inline] pub fn tafcr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x40) as *const u32
  }
#[doc="Get the *mut pointer for the TAFCR register."]
  #[inline] pub fn tafcr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x40) as *mut u32
  }
#[doc="Read the TAFCR register."]
  #[inline] pub fn tafcr(&self) -> Tafcr { 
     unsafe {
        Tafcr(::core::ptr::read_volatile(((self.0 as usize) + 0x40) as *const u32))
     }
  }
#[doc="Write the TAFCR register."]
  #[inline] pub fn set_tafcr(&self, value: Tafcr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x40) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the TAFCR register."]
  #[inline] pub fn with_tafcr<F: FnOnce(Tafcr) -> Tafcr>(&self, f: F) -> &Self {
     let tmp = self.tafcr();
     self.set_tafcr(f(tmp))
  }

#[doc="Get the *const pointer for the ALRMASSR register."]
  #[inline] pub fn alrmassr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x44) as *const u32
  }
#[doc="Get the *mut pointer for the ALRMASSR register."]
  #[inline] pub fn alrmassr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x44) as *mut u32
  }
#[doc="Read the ALRMASSR register."]
  #[inline] pub fn alrmassr(&self) -> Alrmassr { 
     unsafe {
        Alrmassr(::core::ptr::read_volatile(((self.0 as usize) + 0x44) as *const u32))
     }
  }
#[doc="Write the ALRMASSR register."]
  #[inline] pub fn set_alrmassr(&self, value: Alrmassr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x44) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ALRMASSR register."]
  #[inline] pub fn with_alrmassr<F: FnOnce(Alrmassr) -> Alrmassr>(&self, f: F) -> &Self {
     let tmp = self.alrmassr();
     self.set_alrmassr(f(tmp))
  }

#[doc="Get the *const pointer for the ALRMBSSR register."]
  #[inline] pub fn alrmbssr_ptr(&self) -> *const u32 { 
     ((self.0 as usize) + 0x48) as *const u32
  }
#[doc="Get the *mut pointer for the ALRMBSSR register."]
  #[inline] pub fn alrmbssr_mut(&self) -> *mut u32 { 
     ((self.0 as usize) + 0x48) as *mut u32
  }
#[doc="Read the ALRMBSSR register."]
  #[inline] pub fn alrmbssr(&self) -> Alrmbssr { 
     unsafe {
        Alrmbssr(::core::ptr::read_volatile(((self.0 as usize) + 0x48) as *const u32))
     }
  }
#[doc="Write the ALRMBSSR register."]
  #[inline] pub fn set_alrmbssr(&self, value: Alrmbssr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x48) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the ALRMBSSR register."]
  #[inline] pub fn with_alrmbssr<F: FnOnce(Alrmbssr) -> Alrmbssr>(&self, f: F) -> &Self {
     let tmp = self.alrmbssr();
     self.set_alrmbssr(f(tmp))
  }

#[doc="Get the *const pointer for the BKPR register."]
  #[inline] pub fn bkpr_ptr(&self, index: usize) -> *const u32 { 
     assert!(index < 32);
     ((self.0 as usize) + 0x50 + (index << 2)) as *const u32
  }
#[doc="Get the *mut pointer for the BKPR register."]
  #[inline] pub fn bkpr_mut(&self, index: usize) -> *mut u32 { 
     assert!(index < 32);
     ((self.0 as usize) + 0x50 + (index << 2)) as *mut u32
  }
#[doc="Read the BKPR register."]
  #[inline] pub fn bkpr(&self, index: usize) -> Bkpr { 
     assert!(index < 32);
     unsafe {
        Bkpr(::core::ptr::read_volatile(((self.0 as usize) + 0x50 + (index << 2)) as *const u32))
     }
  }
#[doc="Write the BKPR register."]
  #[inline] pub fn set_bkpr(&self, index: usize, value: Bkpr) -> &Self {
     assert!(index < 32);
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x50 + (index << 2)) as *mut u32, value.0);
     }
     self
  }
#[doc="Modify the BKPR register."]
  #[inline] pub fn with_bkpr<F: FnOnce(Bkpr) -> Bkpr>(&self, index: usize, f: F) -> &Self {
     let tmp = self.bkpr(index);
     self.set_bkpr(index, f(tmp))
  }

}

#[doc="time register"]
#[derive(PartialEq, Eq)]
pub struct Tr(pub u32);
impl Tr {
#[doc="AM/PM notation"]
  #[inline] pub fn pm(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="AM/PM notation"]
  #[inline] pub fn set_pm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Hour tens in BCD format"]
  #[inline] pub fn ht(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
#[doc="Hour tens in BCD format"]
  #[inline] pub fn set_ht(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Hour units in BCD format"]
  #[inline] pub fn hu(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
#[doc="Hour units in BCD format"]
  #[inline] pub fn set_hu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Minute tens in BCD format"]
  #[inline] pub fn mnt(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
#[doc="Minute tens in BCD format"]
  #[inline] pub fn set_mnt(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Minute units in BCD format"]
  #[inline] pub fn mnu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
#[doc="Minute units in BCD format"]
  #[inline] pub fn set_mnu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Second tens in BCD format"]
  #[inline] pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
#[doc="Second tens in BCD format"]
  #[inline] pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Second units in BCD format"]
  #[inline] pub fn su(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="Second units in BCD format"]
  #[inline] pub fn set_su(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
      if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="date register"]
#[derive(PartialEq, Eq)]
pub struct Dr(pub u32);
impl Dr {
#[doc="Year tens in BCD format"]
  #[inline] pub fn yt(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0xf // [23:20]
  }
#[doc="Year tens in BCD format"]
  #[inline] pub fn set_yt(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Year units in BCD format"]
  #[inline] pub fn yu(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
#[doc="Year units in BCD format"]
  #[inline] pub fn set_yu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Week day units"]
  #[inline] pub fn wdu(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
#[doc="Week day units"]
  #[inline] pub fn set_wdu(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Month tens in BCD format"]
  #[inline] pub fn mt(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Month tens in BCD format"]
  #[inline] pub fn set_mt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Month units in BCD format"]
  #[inline] pub fn mu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
#[doc="Month units in BCD format"]
  #[inline] pub fn set_mu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Date tens in BCD format"]
  #[inline] pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
#[doc="Date tens in BCD format"]
  #[inline] pub fn set_dt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Date units in BCD format"]
  #[inline] pub fn du(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="Date units in BCD format"]
  #[inline] pub fn set_du(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
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
      if self.yt() != 0 { try!(write!(f, " yt=0x{:x}", self.yt()))}
      if self.yu() != 0 { try!(write!(f, " yu=0x{:x}", self.yu()))}
      if self.wdu() != 0 { try!(write!(f, " wdu=0x{:x}", self.wdu()))}
      if self.mt() != 0 { try!(write!(f, " mt"))}
      if self.mu() != 0 { try!(write!(f, " mu=0x{:x}", self.mu()))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="control register"]
#[derive(PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
#[doc="Wakeup clock selection"]
  #[inline] pub fn wcksel(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7 // [2:0]
  }
#[doc="Wakeup clock selection"]
  #[inline] pub fn set_wcksel(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Time-stamp event active edge"]
  #[inline] pub fn tsedge(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Time-stamp event active edge"]
  #[inline] pub fn set_tsedge(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Reference clock detection enable (50 or 60 Hz)"]
  #[inline] pub fn refckon(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Reference clock detection enable (50 or 60 Hz)"]
  #[inline] pub fn set_refckon(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Bypass the shadow registers"]
  #[inline] pub fn bypshad(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Bypass the shadow registers"]
  #[inline] pub fn set_bypshad(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Hour format"]
  #[inline] pub fn fmt(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Hour format"]
  #[inline] pub fn set_fmt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Alarm A enable"]
  #[inline] pub fn alrae(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Alarm A enable"]
  #[inline] pub fn set_alrae(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Alarm B enable"]
  #[inline] pub fn alrbe(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Alarm B enable"]
  #[inline] pub fn set_alrbe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Wakeup timer enable"]
  #[inline] pub fn wute(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Wakeup timer enable"]
  #[inline] pub fn set_wute(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Time stamp enable"]
  #[inline] pub fn tse(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Time stamp enable"]
  #[inline] pub fn set_tse(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Alarm A interrupt enable"]
  #[inline] pub fn alraie(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Alarm A interrupt enable"]
  #[inline] pub fn set_alraie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Alarm B interrupt enable"]
  #[inline] pub fn alrbie(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Alarm B interrupt enable"]
  #[inline] pub fn set_alrbie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Wakeup timer interrupt enable"]
  #[inline] pub fn wutie(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="Wakeup timer interrupt enable"]
  #[inline] pub fn set_wutie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Time-stamp interrupt enable"]
  #[inline] pub fn tsie(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Time-stamp interrupt enable"]
  #[inline] pub fn set_tsie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Add 1 hour (summer time change)"]
  #[inline] pub fn add1h(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Add 1 hour (summer time change)"]
  #[inline] pub fn set_add1h(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Subtract 1 hour (winter time change)"]
  #[inline] pub fn sub1h(&self) -> u32 {
     ((self.0 as u32) >> 17) & 0x1 // [17]
  }
#[doc="Subtract 1 hour (winter time change)"]
  #[inline] pub fn set_sub1h(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 17);
     self.0 |= value << 17;
     self
  }

#[doc="Backup"]
  #[inline] pub fn bkp(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="Backup"]
  #[inline] pub fn set_bkp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="Calibration output selection"]
  #[inline] pub fn cosel(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="Calibration output selection"]
  #[inline] pub fn set_cosel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="Output polarity"]
  #[inline] pub fn pol(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="Output polarity"]
  #[inline] pub fn set_pol(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Output selection"]
  #[inline] pub fn osel(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x3 // [22:21]
  }
#[doc="Output selection"]
  #[inline] pub fn set_osel(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="Calibration output enable"]
  #[inline] pub fn coe(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="Calibration output enable"]
  #[inline] pub fn set_coe(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

}
impl ::core::fmt::Display for Cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Cr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wcksel() != 0 { try!(write!(f, " wcksel=0x{:x}", self.wcksel()))}
      if self.tsedge() != 0 { try!(write!(f, " tsedge"))}
      if self.refckon() != 0 { try!(write!(f, " refckon"))}
      if self.bypshad() != 0 { try!(write!(f, " bypshad"))}
      if self.fmt() != 0 { try!(write!(f, " fmt"))}
      if self.alrae() != 0 { try!(write!(f, " alrae"))}
      if self.alrbe() != 0 { try!(write!(f, " alrbe"))}
      if self.wute() != 0 { try!(write!(f, " wute"))}
      if self.tse() != 0 { try!(write!(f, " tse"))}
      if self.alraie() != 0 { try!(write!(f, " alraie"))}
      if self.alrbie() != 0 { try!(write!(f, " alrbie"))}
      if self.wutie() != 0 { try!(write!(f, " wutie"))}
      if self.tsie() != 0 { try!(write!(f, " tsie"))}
      if self.add1h() != 0 { try!(write!(f, " add1h"))}
      if self.sub1h() != 0 { try!(write!(f, " sub1h"))}
      if self.bkp() != 0 { try!(write!(f, " bkp"))}
      if self.cosel() != 0 { try!(write!(f, " cosel"))}
      if self.pol() != 0 { try!(write!(f, " pol"))}
      if self.osel() != 0 { try!(write!(f, " osel=0x{:x}", self.osel()))}
      if self.coe() != 0 { try!(write!(f, " coe"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="initialization and status register"]
#[derive(PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
#[doc="Alarm A write flag"]
  #[inline] pub fn alrawf(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Alarm A write flag"]
  #[inline] pub fn set_alrawf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Alarm B write flag"]
  #[inline] pub fn alrbwf(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Alarm B write flag"]
  #[inline] pub fn set_alrbwf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Wakeup timer write flag"]
  #[inline] pub fn wutwf(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Wakeup timer write flag"]
  #[inline] pub fn set_wutwf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Shift operation pending"]
  #[inline] pub fn shpf(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Shift operation pending"]
  #[inline] pub fn set_shpf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Initialization status flag"]
  #[inline] pub fn inits(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Initialization status flag"]
  #[inline] pub fn set_inits(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Registers synchronization flag"]
  #[inline] pub fn rsf(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Registers synchronization flag"]
  #[inline] pub fn set_rsf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Initialization flag"]
  #[inline] pub fn initf(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Initialization flag"]
  #[inline] pub fn set_initf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Initialization mode"]
  #[inline] pub fn init(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Initialization mode"]
  #[inline] pub fn set_init(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Alarm A flag"]
  #[inline] pub fn alraf(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x1 // [8]
  }
#[doc="Alarm A flag"]
  #[inline] pub fn set_alraf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Alarm B flag"]
  #[inline] pub fn alrbf(&self) -> u32 {
     ((self.0 as u32) >> 9) & 0x1 // [9]
  }
#[doc="Alarm B flag"]
  #[inline] pub fn set_alrbf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 9);
     self.0 |= value << 9;
     self
  }

#[doc="Wakeup timer flag"]
  #[inline] pub fn wutf(&self) -> u32 {
     ((self.0 as u32) >> 10) & 0x1 // [10]
  }
#[doc="Wakeup timer flag"]
  #[inline] pub fn set_wutf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 10);
     self.0 |= value << 10;
     self
  }

#[doc="Time-stamp flag"]
  #[inline] pub fn tsf(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x1 // [11]
  }
#[doc="Time-stamp flag"]
  #[inline] pub fn set_tsf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Time-stamp overflow flag"]
  #[inline] pub fn tsovf(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Time-stamp overflow flag"]
  #[inline] pub fn set_tsovf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Tamper detection flag"]
  #[inline] pub fn tamp1f(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Tamper detection flag"]
  #[inline] pub fn set_tamp1f(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="RTC_TAMP2 detection flag"]
  #[inline] pub fn tamp2f(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="RTC_TAMP2 detection flag"]
  #[inline] pub fn set_tamp2f(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="RTC_TAMP3 detection flag"]
  #[inline] pub fn tamp3f(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="RTC_TAMP3 detection flag"]
  #[inline] pub fn set_tamp3f(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Recalibration pending Flag"]
  #[inline] pub fn recalpf(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x1 // [16]
  }
#[doc="Recalibration pending Flag"]
  #[inline] pub fn set_recalpf(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 16);
     self.0 |= value << 16;
     self
  }

}
impl ::core::fmt::Display for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Isr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.alrawf() != 0 { try!(write!(f, " alrawf"))}
      if self.alrbwf() != 0 { try!(write!(f, " alrbwf"))}
      if self.wutwf() != 0 { try!(write!(f, " wutwf"))}
      if self.shpf() != 0 { try!(write!(f, " shpf"))}
      if self.inits() != 0 { try!(write!(f, " inits"))}
      if self.rsf() != 0 { try!(write!(f, " rsf"))}
      if self.initf() != 0 { try!(write!(f, " initf"))}
      if self.init() != 0 { try!(write!(f, " init"))}
      if self.alraf() != 0 { try!(write!(f, " alraf"))}
      if self.alrbf() != 0 { try!(write!(f, " alrbf"))}
      if self.wutf() != 0 { try!(write!(f, " wutf"))}
      if self.tsf() != 0 { try!(write!(f, " tsf"))}
      if self.tsovf() != 0 { try!(write!(f, " tsovf"))}
      if self.tamp1f() != 0 { try!(write!(f, " tamp1f"))}
      if self.tamp2f() != 0 { try!(write!(f, " tamp2f"))}
      if self.tamp3f() != 0 { try!(write!(f, " tamp3f"))}
      if self.recalpf() != 0 { try!(write!(f, " recalpf"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="prescaler register"]
#[derive(PartialEq, Eq)]
pub struct Prer(pub u32);
impl Prer {
#[doc="Asynchronous prescaler factor"]
  #[inline] pub fn prediv_a(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0x7f // [22:16]
  }
#[doc="Asynchronous prescaler factor"]
  #[inline] pub fn set_prediv_a(mut self, value: u32) -> Self {
     assert!((value & !0x7f) == 0);
     self.0 &= !(0x7f << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Synchronous prescaler factor"]
  #[inline] pub fn prediv_s(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fff // [14:0]
  }
#[doc="Synchronous prescaler factor"]
  #[inline] pub fn set_prediv_s(mut self, value: u32) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Prer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Prer {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.prediv_a() != 0 { try!(write!(f, " prediv_a=0x{:x}", self.prediv_a()))}
      if self.prediv_s() != 0 { try!(write!(f, " prediv_s=0x{:x}", self.prediv_s()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="wakeup timer register"]
#[derive(PartialEq, Eq)]
pub struct Wutr(pub u32);
impl Wutr {
#[doc="Wakeup auto-reload value bits"]
  #[inline] pub fn wut(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Wakeup auto-reload value bits"]
  #[inline] pub fn set_wut(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Wutr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wutr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wut() != 0 { try!(write!(f, " wut=0x{:x}", self.wut()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="alarm A register"]
#[derive(PartialEq, Eq)]
pub struct Alrmar(pub u32);
impl Alrmar {
#[doc="Alarm A date mask"]
  #[inline] pub fn msk4(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Alarm A date mask"]
  #[inline] pub fn set_msk4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

#[doc="Week day selection"]
  #[inline] pub fn wdsel(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Week day selection"]
  #[inline] pub fn set_wdsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Date tens in BCD format"]
  #[inline] pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
#[doc="Date tens in BCD format"]
  #[inline] pub fn set_dt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Date units or day in BCD format"]
  #[inline] pub fn du(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
#[doc="Date units or day in BCD format"]
  #[inline] pub fn set_du(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Alarm A hours mask"]
  #[inline] pub fn msk3(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="Alarm A hours mask"]
  #[inline] pub fn set_msk3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="AM/PM notation"]
  #[inline] pub fn pm(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="AM/PM notation"]
  #[inline] pub fn set_pm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Hour tens in BCD format"]
  #[inline] pub fn ht(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
#[doc="Hour tens in BCD format"]
  #[inline] pub fn set_ht(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Hour units in BCD format"]
  #[inline] pub fn hu(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
#[doc="Hour units in BCD format"]
  #[inline] pub fn set_hu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Alarm A minutes mask"]
  #[inline] pub fn msk2(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Alarm A minutes mask"]
  #[inline] pub fn set_msk2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Minute tens in BCD format"]
  #[inline] pub fn mnt(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
#[doc="Minute tens in BCD format"]
  #[inline] pub fn set_mnt(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Minute units in BCD format"]
  #[inline] pub fn mnu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
#[doc="Minute units in BCD format"]
  #[inline] pub fn set_mnu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Alarm A seconds mask"]
  #[inline] pub fn msk1(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Alarm A seconds mask"]
  #[inline] pub fn set_msk1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Second tens in BCD format"]
  #[inline] pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
#[doc="Second tens in BCD format"]
  #[inline] pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Second units in BCD format"]
  #[inline] pub fn su(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="Second units in BCD format"]
  #[inline] pub fn set_su(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Alrmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Alrmar {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.msk4() != 0 { try!(write!(f, " msk4"))}
      if self.wdsel() != 0 { try!(write!(f, " wdsel"))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
      if self.msk3() != 0 { try!(write!(f, " msk3"))}
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      if self.msk2() != 0 { try!(write!(f, " msk2"))}
      if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
      if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
      if self.msk1() != 0 { try!(write!(f, " msk1"))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="alarm B register"]
#[derive(PartialEq, Eq)]
pub struct Alrmbr(pub u32);
impl Alrmbr {
#[doc="Alarm B date mask"]
  #[inline] pub fn msk4(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Alarm B date mask"]
  #[inline] pub fn set_msk4(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

#[doc="Week day selection"]
  #[inline] pub fn wdsel(&self) -> u32 {
     ((self.0 as u32) >> 30) & 0x1 // [30]
  }
#[doc="Week day selection"]
  #[inline] pub fn set_wdsel(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 30);
     self.0 |= value << 30;
     self
  }

#[doc="Date tens in BCD format"]
  #[inline] pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 28) & 0x3 // [29:28]
  }
#[doc="Date tens in BCD format"]
  #[inline] pub fn set_dt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 28);
     self.0 |= value << 28;
     self
  }

#[doc="Date units or day in BCD format"]
  #[inline] pub fn du(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
#[doc="Date units or day in BCD format"]
  #[inline] pub fn set_du(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Alarm B hours mask"]
  #[inline] pub fn msk3(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="Alarm B hours mask"]
  #[inline] pub fn set_msk3(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

#[doc="AM/PM notation"]
  #[inline] pub fn pm(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="AM/PM notation"]
  #[inline] pub fn set_pm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="Hour tens in BCD format"]
  #[inline] pub fn ht(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
#[doc="Hour tens in BCD format"]
  #[inline] pub fn set_ht(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="Hour units in BCD format"]
  #[inline] pub fn hu(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
#[doc="Hour units in BCD format"]
  #[inline] pub fn set_hu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Alarm B minutes mask"]
  #[inline] pub fn msk2(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Alarm B minutes mask"]
  #[inline] pub fn set_msk2(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Minute tens in BCD format"]
  #[inline] pub fn mnt(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
#[doc="Minute tens in BCD format"]
  #[inline] pub fn set_mnt(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Minute units in BCD format"]
  #[inline] pub fn mnu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
#[doc="Minute units in BCD format"]
  #[inline] pub fn set_mnu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Alarm B seconds mask"]
  #[inline] pub fn msk1(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Alarm B seconds mask"]
  #[inline] pub fn set_msk1(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Second tens in BCD format"]
  #[inline] pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
#[doc="Second tens in BCD format"]
  #[inline] pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Second units in BCD format"]
  #[inline] pub fn su(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="Second units in BCD format"]
  #[inline] pub fn set_su(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Alrmbr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Alrmbr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.msk4() != 0 { try!(write!(f, " msk4"))}
      if self.wdsel() != 0 { try!(write!(f, " wdsel"))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
      if self.msk3() != 0 { try!(write!(f, " msk3"))}
      if self.pm() != 0 { try!(write!(f, " pm"))}
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      if self.msk2() != 0 { try!(write!(f, " msk2"))}
      if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
      if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
      if self.msk1() != 0 { try!(write!(f, " msk1"))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="write protection register"]
#[derive(PartialEq, Eq)]
pub struct Wpr(pub u32);
impl Wpr {
#[doc="Write protection key"]
  #[inline] pub fn key(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xff // [7:0]
  }
#[doc="Write protection key"]
  #[inline] pub fn set_key(mut self, value: u32) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Wpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Wpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.key() != 0 { try!(write!(f, " key=0x{:x}", self.key()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="sub second register"]
#[derive(PartialEq, Eq)]
pub struct Ssr(pub u32);
impl Ssr {
#[doc="Sub second value"]
  #[inline] pub fn ss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Sub second value"]
  #[inline] pub fn set_ss(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="shift control register"]
#[derive(PartialEq, Eq)]
pub struct Shiftr(pub u32);
impl Shiftr {
#[doc="Add one second"]
  #[inline] pub fn add1s(&self) -> u32 {
     ((self.0 as u32) >> 31) & 0x1 // [31]
  }
#[doc="Add one second"]
  #[inline] pub fn set_add1s(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 31);
     self.0 |= value << 31;
     self
  }

#[doc="Subtract a fraction of a second"]
  #[inline] pub fn subfs(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fff // [14:0]
  }
#[doc="Subtract a fraction of a second"]
  #[inline] pub fn set_subfs(mut self, value: u32) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Shiftr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Shiftr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.add1s() != 0 { try!(write!(f, " add1s"))}
      if self.subfs() != 0 { try!(write!(f, " subfs=0x{:x}", self.subfs()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="time stamp time register"]
#[derive(PartialEq, Eq)]
pub struct Tstr(pub u32);
impl Tstr {
#[doc="Second units in BCD format"]
  #[inline] pub fn su(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="Second units in BCD format"]
  #[inline] pub fn set_su(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Second tens in BCD format"]
  #[inline] pub fn st(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x7 // [6:4]
  }
#[doc="Second tens in BCD format"]
  #[inline] pub fn set_st(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Minute units in BCD format"]
  #[inline] pub fn mnu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
#[doc="Minute units in BCD format"]
  #[inline] pub fn set_mnu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Minute tens in BCD format"]
  #[inline] pub fn mnt(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x7 // [14:12]
  }
#[doc="Minute tens in BCD format"]
  #[inline] pub fn set_mnt(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Hour units in BCD format"]
  #[inline] pub fn hu(&self) -> u32 {
     ((self.0 as u32) >> 16) & 0xf // [19:16]
  }
#[doc="Hour units in BCD format"]
  #[inline] pub fn set_hu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 16);
     self.0 |= value << 16;
     self
  }

#[doc="Hour tens in BCD format"]
  #[inline] pub fn ht(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x3 // [21:20]
  }
#[doc="Hour tens in BCD format"]
  #[inline] pub fn set_ht(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="AM/PM notation"]
  #[inline] pub fn pm(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="AM/PM notation"]
  #[inline] pub fn set_pm(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

}
impl ::core::fmt::Display for Tstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tstr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.su() != 0 { try!(write!(f, " su=0x{:x}", self.su()))}
      if self.st() != 0 { try!(write!(f, " st=0x{:x}", self.st()))}
      if self.mnu() != 0 { try!(write!(f, " mnu=0x{:x}", self.mnu()))}
      if self.mnt() != 0 { try!(write!(f, " mnt=0x{:x}", self.mnt()))}
      if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
      if self.ht() != 0 { try!(write!(f, " ht=0x{:x}", self.ht()))}
      if self.pm() != 0 { try!(write!(f, " pm"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="time stamp date register"]
#[derive(PartialEq, Eq)]
pub struct Tsdr(pub u32);
impl Tsdr {
#[doc="Week day units"]
  #[inline] pub fn wdu(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x7 // [15:13]
  }
#[doc="Week day units"]
  #[inline] pub fn set_wdu(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Month tens in BCD format"]
  #[inline] pub fn mt(&self) -> u32 {
     ((self.0 as u32) >> 12) & 0x1 // [12]
  }
#[doc="Month tens in BCD format"]
  #[inline] pub fn set_mt(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 12);
     self.0 |= value << 12;
     self
  }

#[doc="Month units in BCD format"]
  #[inline] pub fn mu(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0xf // [11:8]
  }
#[doc="Month units in BCD format"]
  #[inline] pub fn set_mu(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Date tens in BCD format"]
  #[inline] pub fn dt(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x3 // [5:4]
  }
#[doc="Date tens in BCD format"]
  #[inline] pub fn set_dt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Date units in BCD format"]
  #[inline] pub fn du(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xf // [3:0]
  }
#[doc="Date units in BCD format"]
  #[inline] pub fn set_du(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tsdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tsdr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.wdu() != 0 { try!(write!(f, " wdu=0x{:x}", self.wdu()))}
      if self.mt() != 0 { try!(write!(f, " mt"))}
      if self.mu() != 0 { try!(write!(f, " mu=0x{:x}", self.mu()))}
      if self.dt() != 0 { try!(write!(f, " dt=0x{:x}", self.dt()))}
      if self.du() != 0 { try!(write!(f, " du=0x{:x}", self.du()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="timestamp sub second register"]
#[derive(PartialEq, Eq)]
pub struct Tsssr(pub u32);
impl Tsssr {
#[doc="Sub second value"]
  #[inline] pub fn ss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffff // [15:0]
  }
#[doc="Sub second value"]
  #[inline] pub fn set_ss(mut self, value: u32) -> Self {
     assert!((value & !0xffff) == 0);
     self.0 &= !(0xffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Tsssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tsssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="calibration register"]
#[derive(PartialEq, Eq)]
pub struct Calr(pub u32);
impl Calr {
#[doc="Increase frequency of RTC by 488.5 ppm"]
  #[inline] pub fn calp(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="Increase frequency of RTC by 488.5 ppm"]
  #[inline] pub fn set_calp(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="Use an 8-second calibration cycle period"]
  #[inline] pub fn calw8(&self) -> u32 {
     ((self.0 as u32) >> 14) & 0x1 // [14]
  }
#[doc="Use an 8-second calibration cycle period"]
  #[inline] pub fn set_calw8(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 14);
     self.0 |= value << 14;
     self
  }

#[doc="Use a 16-second calibration cycle period"]
  #[inline] pub fn calw16(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x1 // [13]
  }
#[doc="Use a 16-second calibration cycle period"]
  #[inline] pub fn set_calw16(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="Calibration minus"]
  #[inline] pub fn calm(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1ff // [8:0]
  }
#[doc="Calibration minus"]
  #[inline] pub fn set_calm(mut self, value: u32) -> Self {
     assert!((value & !0x1ff) == 0);
     self.0 &= !(0x1ff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Calr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Calr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.calp() != 0 { try!(write!(f, " calp"))}
      if self.calw8() != 0 { try!(write!(f, " calw8"))}
      if self.calw16() != 0 { try!(write!(f, " calw16"))}
      if self.calm() != 0 { try!(write!(f, " calm=0x{:x}", self.calm()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="tamper and alternate function configuration register"]
#[derive(PartialEq, Eq)]
pub struct Tafcr(pub u32);
impl Tafcr {
#[doc="Tamper 1 detection enable"]
  #[inline] pub fn tamp1e(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x1 // [0]
  }
#[doc="Tamper 1 detection enable"]
  #[inline] pub fn set_tamp1e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

#[doc="Active level for tamper 1"]
  #[inline] pub fn tamp1trg(&self) -> u32 {
     ((self.0 as u32) >> 1) & 0x1 // [1]
  }
#[doc="Active level for tamper 1"]
  #[inline] pub fn set_tamp1trg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

#[doc="Tamper interrupt enable"]
  #[inline] pub fn tampie(&self) -> u32 {
     ((self.0 as u32) >> 2) & 0x1 // [2]
  }
#[doc="Tamper interrupt enable"]
  #[inline] pub fn set_tampie(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

#[doc="Tamper 2 detection enable"]
  #[inline] pub fn tamp2e(&self) -> u32 {
     ((self.0 as u32) >> 3) & 0x1 // [3]
  }
#[doc="Tamper 2 detection enable"]
  #[inline] pub fn set_tamp2e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 3);
     self.0 |= value << 3;
     self
  }

#[doc="Active level for tamper 2"]
  #[inline] pub fn tamp2trg(&self) -> u32 {
     ((self.0 as u32) >> 4) & 0x1 // [4]
  }
#[doc="Active level for tamper 2"]
  #[inline] pub fn set_tamp2trg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 4);
     self.0 |= value << 4;
     self
  }

#[doc="Tamper 3 detection enable"]
  #[inline] pub fn tamp3e(&self) -> u32 {
     ((self.0 as u32) >> 5) & 0x1 // [5]
  }
#[doc="Tamper 3 detection enable"]
  #[inline] pub fn set_tamp3e(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 5);
     self.0 |= value << 5;
     self
  }

#[doc="Active level for tamper 3"]
  #[inline] pub fn tamp3trg(&self) -> u32 {
     ((self.0 as u32) >> 6) & 0x1 // [6]
  }
#[doc="Active level for tamper 3"]
  #[inline] pub fn set_tamp3trg(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 6);
     self.0 |= value << 6;
     self
  }

#[doc="Activate timestamp on tamper detection event"]
  #[inline] pub fn tampts(&self) -> u32 {
     ((self.0 as u32) >> 7) & 0x1 // [7]
  }
#[doc="Activate timestamp on tamper detection event"]
  #[inline] pub fn set_tampts(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

#[doc="Tamper sampling frequency"]
  #[inline] pub fn tampfreq(&self) -> u32 {
     ((self.0 as u32) >> 8) & 0x7 // [10:8]
  }
#[doc="Tamper sampling frequency"]
  #[inline] pub fn set_tampfreq(mut self, value: u32) -> Self {
     assert!((value & !0x7) == 0);
     self.0 &= !(0x7 << 8);
     self.0 |= value << 8;
     self
  }

#[doc="Tamper filter count"]
  #[inline] pub fn tampflt(&self) -> u32 {
     ((self.0 as u32) >> 11) & 0x3 // [12:11]
  }
#[doc="Tamper filter count"]
  #[inline] pub fn set_tampflt(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 11);
     self.0 |= value << 11;
     self
  }

#[doc="Tamper precharge duration"]
  #[inline] pub fn tampprch(&self) -> u32 {
     ((self.0 as u32) >> 13) & 0x3 // [14:13]
  }
#[doc="Tamper precharge duration"]
  #[inline] pub fn set_tampprch(mut self, value: u32) -> Self {
     assert!((value & !0x3) == 0);
     self.0 &= !(0x3 << 13);
     self.0 |= value << 13;
     self
  }

#[doc="TAMPER pull-up disable"]
  #[inline] pub fn tamppudis(&self) -> u32 {
     ((self.0 as u32) >> 15) & 0x1 // [15]
  }
#[doc="TAMPER pull-up disable"]
  #[inline] pub fn set_tamppudis(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 15);
     self.0 |= value << 15;
     self
  }

#[doc="PC13 value"]
  #[inline] pub fn pc13value(&self) -> u32 {
     ((self.0 as u32) >> 18) & 0x1 // [18]
  }
#[doc="PC13 value"]
  #[inline] pub fn set_pc13value(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 18);
     self.0 |= value << 18;
     self
  }

#[doc="PC13 mode"]
  #[inline] pub fn pc13mode(&self) -> u32 {
     ((self.0 as u32) >> 19) & 0x1 // [19]
  }
#[doc="PC13 mode"]
  #[inline] pub fn set_pc13mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 19);
     self.0 |= value << 19;
     self
  }

#[doc="PC14 value"]
  #[inline] pub fn pc14value(&self) -> u32 {
     ((self.0 as u32) >> 20) & 0x1 // [20]
  }
#[doc="PC14 value"]
  #[inline] pub fn set_pc14value(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 20);
     self.0 |= value << 20;
     self
  }

#[doc="PC 14 mode"]
  #[inline] pub fn pc14mode(&self) -> u32 {
     ((self.0 as u32) >> 21) & 0x1 // [21]
  }
#[doc="PC 14 mode"]
  #[inline] pub fn set_pc14mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 21);
     self.0 |= value << 21;
     self
  }

#[doc="PC15 value"]
  #[inline] pub fn pc15value(&self) -> u32 {
     ((self.0 as u32) >> 22) & 0x1 // [22]
  }
#[doc="PC15 value"]
  #[inline] pub fn set_pc15value(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 22);
     self.0 |= value << 22;
     self
  }

#[doc="PC15 mode"]
  #[inline] pub fn pc15mode(&self) -> u32 {
     ((self.0 as u32) >> 23) & 0x1 // [23]
  }
#[doc="PC15 mode"]
  #[inline] pub fn set_pc15mode(mut self, value: u32) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 23);
     self.0 |= value << 23;
     self
  }

}
impl ::core::fmt::Display for Tafcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Tafcr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.tamp1e() != 0 { try!(write!(f, " tamp1e"))}
      if self.tamp1trg() != 0 { try!(write!(f, " tamp1trg"))}
      if self.tampie() != 0 { try!(write!(f, " tampie"))}
      if self.tamp2e() != 0 { try!(write!(f, " tamp2e"))}
      if self.tamp2trg() != 0 { try!(write!(f, " tamp2trg"))}
      if self.tamp3e() != 0 { try!(write!(f, " tamp3e"))}
      if self.tamp3trg() != 0 { try!(write!(f, " tamp3trg"))}
      if self.tampts() != 0 { try!(write!(f, " tampts"))}
      if self.tampfreq() != 0 { try!(write!(f, " tampfreq=0x{:x}", self.tampfreq()))}
      if self.tampflt() != 0 { try!(write!(f, " tampflt=0x{:x}", self.tampflt()))}
      if self.tampprch() != 0 { try!(write!(f, " tampprch=0x{:x}", self.tampprch()))}
      if self.tamppudis() != 0 { try!(write!(f, " tamppudis"))}
      if self.pc13value() != 0 { try!(write!(f, " pc13value"))}
      if self.pc13mode() != 0 { try!(write!(f, " pc13mode"))}
      if self.pc14value() != 0 { try!(write!(f, " pc14value"))}
      if self.pc14mode() != 0 { try!(write!(f, " pc14mode"))}
      if self.pc15value() != 0 { try!(write!(f, " pc15value"))}
      if self.pc15mode() != 0 { try!(write!(f, " pc15mode"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="alarm A sub second register"]
#[derive(PartialEq, Eq)]
pub struct Alrmassr(pub u32);
impl Alrmassr {
#[doc="Mask the most-significant bits starting at this bit"]
  #[inline] pub fn maskss(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
#[doc="Mask the most-significant bits starting at this bit"]
  #[inline] pub fn set_maskss(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Sub seconds value"]
  #[inline] pub fn ss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fff // [14:0]
  }
#[doc="Sub seconds value"]
  #[inline] pub fn set_ss(mut self, value: u32) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Alrmassr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Alrmassr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maskss() != 0 { try!(write!(f, " maskss=0x{:x}", self.maskss()))}
      if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="alarm B sub second register"]
#[derive(PartialEq, Eq)]
pub struct Alrmbssr(pub u32);
impl Alrmbssr {
#[doc="Mask the most-significant bits starting at this bit"]
  #[inline] pub fn maskss(&self) -> u32 {
     ((self.0 as u32) >> 24) & 0xf // [27:24]
  }
#[doc="Mask the most-significant bits starting at this bit"]
  #[inline] pub fn set_maskss(mut self, value: u32) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 24);
     self.0 |= value << 24;
     self
  }

#[doc="Sub seconds value"]
  #[inline] pub fn ss(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0x7fff // [14:0]
  }
#[doc="Sub seconds value"]
  #[inline] pub fn set_ss(mut self, value: u32) -> Self {
     assert!((value & !0x7fff) == 0);
     self.0 &= !(0x7fff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Alrmbssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Alrmbssr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.maskss() != 0 { try!(write!(f, " maskss=0x{:x}", self.maskss()))}
      if self.ss() != 0 { try!(write!(f, " ss=0x{:x}", self.ss()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[doc="backup register n"]
#[derive(PartialEq, Eq)]
pub struct Bkpr(pub u32);
impl Bkpr {
#[doc="BKP"]
  #[inline] pub fn bkp(&self) -> u32 {
     ((self.0 as u32) >> 0) & 0xffffffff // [31:0]
  }
#[doc="BKP"]
  #[inline] pub fn set_bkp(mut self, value: u32) -> Self {
     assert!((value & !0xffffffff) == 0);
     self.0 &= !(0xffffffff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Bkpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Bkpr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      try!(write!(f, "]"));
      Ok(())
   }
}

