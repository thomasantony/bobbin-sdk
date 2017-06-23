pub const WDT: Wdt = Wdt(0x40001000);

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Wdt(pub u32);
impl Wdt {
  #[inline] pub fn clear_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x8) as *const u8
  }
  #[inline] pub fn clear_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x8) as *mut u8
  }
  #[inline] pub fn set_clear(&self, value: Clear) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x8) as *mut u8, value.0);
     }
     self
  }

  #[inline] pub fn config_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x1) as *const u8
  }
  #[inline] pub fn config_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x1) as *mut u8
  }
  #[inline] pub fn config(&self) -> Config { 
     unsafe {
        Config(::core::ptr::read_volatile(((self.0 as usize) + 0x1) as *const u8))
     }
  }
  #[inline] pub fn set_config(&self, value: Config) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x1) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_config<F: FnOnce(Config) -> Config>(&self, f: F) -> &Self {
     let tmp = self.config();
     self.set_config(f(tmp))
  }

  #[inline] pub fn ctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x0) as *const u8
  }
  #[inline] pub fn ctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x0) as *mut u8
  }
  #[inline] pub fn ctrl(&self) -> Ctrl { 
     unsafe {
        Ctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x0) as *const u8))
     }
  }
  #[inline] pub fn set_ctrl(&self, value: Ctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x0) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
     let tmp = self.ctrl();
     self.set_ctrl(f(tmp))
  }

  #[inline] pub fn ewctrl_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x2) as *const u8
  }
  #[inline] pub fn ewctrl_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x2) as *mut u8
  }
  #[inline] pub fn ewctrl(&self) -> Ewctrl { 
     unsafe {
        Ewctrl(::core::ptr::read_volatile(((self.0 as usize) + 0x2) as *const u8))
     }
  }
  #[inline] pub fn set_ewctrl(&self, value: Ewctrl) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x2) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_ewctrl<F: FnOnce(Ewctrl) -> Ewctrl>(&self, f: F) -> &Self {
     let tmp = self.ewctrl();
     self.set_ewctrl(f(tmp))
  }

  #[inline] pub fn intenclr_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x4) as *const u8
  }
  #[inline] pub fn intenclr_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x4) as *mut u8
  }
  #[inline] pub fn intenclr(&self) -> Intenclr { 
     unsafe {
        Intenclr(::core::ptr::read_volatile(((self.0 as usize) + 0x4) as *const u8))
     }
  }
  #[inline] pub fn set_intenclr(&self, value: Intenclr) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x4) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_intenclr<F: FnOnce(Intenclr) -> Intenclr>(&self, f: F) -> &Self {
     let tmp = self.intenclr();
     self.set_intenclr(f(tmp))
  }

  #[inline] pub fn intenset_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x5) as *const u8
  }
  #[inline] pub fn intenset_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x5) as *mut u8
  }
  #[inline] pub fn intenset(&self) -> Intenset { 
     unsafe {
        Intenset(::core::ptr::read_volatile(((self.0 as usize) + 0x5) as *const u8))
     }
  }
  #[inline] pub fn set_intenset(&self, value: Intenset) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x5) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_intenset<F: FnOnce(Intenset) -> Intenset>(&self, f: F) -> &Self {
     let tmp = self.intenset();
     self.set_intenset(f(tmp))
  }

  #[inline] pub fn intflag_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x6) as *const u8
  }
  #[inline] pub fn intflag_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x6) as *mut u8
  }
  #[inline] pub fn intflag(&self) -> Intflag { 
     unsafe {
        Intflag(::core::ptr::read_volatile(((self.0 as usize) + 0x6) as *const u8))
     }
  }
  #[inline] pub fn set_intflag(&self, value: Intflag) -> &Self {
     unsafe {
        ::core::ptr::write_volatile(((self.0 as usize) + 0x6) as *mut u8, value.0);
     }
     self
  }
  #[inline] pub fn with_intflag<F: FnOnce(Intflag) -> Intflag>(&self, f: F) -> &Self {
     let tmp = self.intflag();
     self.set_intflag(f(tmp))
  }

  #[inline] pub fn status_ptr(&self) -> *const u8 { 
     ((self.0 as usize) + 0x7) as *const u8
  }
  #[inline] pub fn status_mut(&self) -> *mut u8 { 
     ((self.0 as usize) + 0x7) as *mut u8
  }
  #[inline] pub fn status(&self) -> Status { 
     unsafe {
        Status(::core::ptr::read_volatile(((self.0 as usize) + 0x7) as *const u8))
     }
  }

}

#[derive(PartialEq, Eq)]
pub struct Clear(pub u8);
impl Clear {
  #[inline] pub fn clear(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xff // [7:0]
  }
  #[inline] pub fn set_clear(mut self, value: u8) -> Self {
     assert!((value & !0xff) == 0);
     self.0 &= !(0xff << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Clear {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Clear {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.clear() != 0 { try!(write!(f, " clear=0x{:x}", self.clear()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Config(pub u8);
impl Config {
  #[inline] pub fn per(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_per(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

  #[inline] pub fn window(&self) -> u8 {
     ((self.0 as u8) >> 4) & 0xf // [7:4]
  }
  #[inline] pub fn set_window(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 4);
     self.0 |= value << 4;
     self
  }

}
impl ::core::fmt::Display for Config {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Config {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.per() != 0 { try!(write!(f, " per=0x{:x}", self.per()))}
      if self.window() != 0 { try!(write!(f, " window=0x{:x}", self.window()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ctrl(pub u8);
impl Ctrl {
  #[inline] pub fn enable(&self) -> u8 {
     ((self.0 as u8) >> 1) & 0x1 // [1]
  }
  #[inline] pub fn set_enable(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 1);
     self.0 |= value << 1;
     self
  }

  #[inline] pub fn wen(&self) -> u8 {
     ((self.0 as u8) >> 2) & 0x1 // [2]
  }
  #[inline] pub fn set_wen(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 2);
     self.0 |= value << 2;
     self
  }

  #[inline] pub fn alwayson(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_alwayson(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
     self
  }

}
impl ::core::fmt::Display for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.enable() != 0 { try!(write!(f, " enable"))}
      if self.wen() != 0 { try!(write!(f, " wen"))}
      if self.alwayson() != 0 { try!(write!(f, " alwayson"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Ewctrl(pub u8);
impl Ewctrl {
  #[inline] pub fn ewoffset(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0xf // [3:0]
  }
  #[inline] pub fn set_ewoffset(mut self, value: u8) -> Self {
     assert!((value & !0xf) == 0);
     self.0 &= !(0xf << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Ewctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Ewctrl {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ewoffset() != 0 { try!(write!(f, " ewoffset=0x{:x}", self.ewoffset()))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intenclr(pub u8);
impl Intenclr {
  #[inline] pub fn ew(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_ew(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenclr {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ew() != 0 { try!(write!(f, " ew"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intenset(pub u8);
impl Intenset {
  #[inline] pub fn ew(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_ew(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intenset {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ew() != 0 { try!(write!(f, " ew"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Intflag(pub u8);
impl Intflag {
  #[inline] pub fn ew(&self) -> u8 {
     ((self.0 as u8) >> 0) & 0x1 // [0]
  }
  #[inline] pub fn set_ew(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 0);
     self.0 |= value << 0;
     self
  }

}
impl ::core::fmt::Display for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
       self.0.fmt(f)
   }
}
impl ::core::fmt::Debug for Intflag {
   fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
      try!(write!(f, "[0x{:08x}", self.0));
      if self.ew() != 0 { try!(write!(f, " ew"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
#[derive(PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
  #[inline] pub fn syncbusy(&self) -> u8 {
     ((self.0 as u8) >> 7) & 0x1 // [7]
  }
  #[inline] pub fn set_syncbusy(mut self, value: u8) -> Self {
     assert!((value & !0x1) == 0);
     self.0 &= !(0x1 << 7);
     self.0 |= value << 7;
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
      if self.syncbusy() != 0 { try!(write!(f, " syncbusy"))}
      try!(write!(f, "]"));
      Ok(())
   }
}
