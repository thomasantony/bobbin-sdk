::bobbin_mcu::periph!( WDOG, Wdog, WDOG_PERIPH, WdogPeriph, WDOG_OWNED, WDOG_REF_COUNT, 0x40052000, 0x00, 0x0b);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="WDOG Peripheral"]
pub struct WdogPeriph(pub usize); 

impl WdogPeriph {
    #[doc="Get the CS1 Register."]
    #[inline] pub fn cs1_reg(&self) -> ::bobbin_mcu::register::Register<Cs1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cs1, 0x0)
    }

    #[doc="Get the *mut pointer for the CS1 register."]
    #[inline] pub fn cs1_mut(&self) -> *mut Cs1 { 
        self.cs1_reg().ptr()
    }

    #[doc="Get the *const pointer for the CS1 register."]
    #[inline] pub fn cs1_ptr(&self) -> *const Cs1 { 
        self.cs1_reg().ptr()
    }

    #[doc="Read the CS1 register."]
    #[inline] pub fn cs1(&self) -> Cs1 { 
        self.cs1_reg().read()
    }

    #[doc="Write the CS1 register."]
    #[inline] pub fn write_cs1(&self, value: Cs1) -> &Self { 
        self.cs1_reg().write(value);
        self
    }

    #[doc="Set the CS1 register."]
    #[inline] pub fn set_cs1<F: FnOnce(Cs1) -> Cs1>(&self, f: F) -> &Self {
        self.cs1_reg().set(f);
        self
    }

    #[doc="Modify the CS1 register."]
    #[inline] pub fn with_cs1<F: FnOnce(Cs1) -> Cs1>(&self, f: F) -> &Self {
        self.cs1_reg().with(f);
        self
    }

    #[doc="Get the CS2 Register."]
    #[inline] pub fn cs2_reg(&self) -> ::bobbin_mcu::register::Register<Cs2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cs2, 0x1)
    }

    #[doc="Get the *mut pointer for the CS2 register."]
    #[inline] pub fn cs2_mut(&self) -> *mut Cs2 { 
        self.cs2_reg().ptr()
    }

    #[doc="Get the *const pointer for the CS2 register."]
    #[inline] pub fn cs2_ptr(&self) -> *const Cs2 { 
        self.cs2_reg().ptr()
    }

    #[doc="Read the CS2 register."]
    #[inline] pub fn cs2(&self) -> Cs2 { 
        self.cs2_reg().read()
    }

    #[doc="Write the CS2 register."]
    #[inline] pub fn write_cs2(&self, value: Cs2) -> &Self { 
        self.cs2_reg().write(value);
        self
    }

    #[doc="Set the CS2 register."]
    #[inline] pub fn set_cs2<F: FnOnce(Cs2) -> Cs2>(&self, f: F) -> &Self {
        self.cs2_reg().set(f);
        self
    }

    #[doc="Modify the CS2 register."]
    #[inline] pub fn with_cs2<F: FnOnce(Cs2) -> Cs2>(&self, f: F) -> &Self {
        self.cs2_reg().with(f);
        self
    }

    #[doc="Get the CNTH Register."]
    #[inline] pub fn cnth_reg(&self) -> ::bobbin_mcu::register::Register<Cnth> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cnth, 0x2)
    }

    #[doc="Get the *mut pointer for the CNTH register."]
    #[inline] pub fn cnth_mut(&self) -> *mut Cnth { 
        self.cnth_reg().ptr()
    }

    #[doc="Get the *const pointer for the CNTH register."]
    #[inline] pub fn cnth_ptr(&self) -> *const Cnth { 
        self.cnth_reg().ptr()
    }

    #[doc="Read the CNTH register."]
    #[inline] pub fn cnth(&self) -> Cnth { 
        self.cnth_reg().read()
    }

    #[doc="Write the CNTH register."]
    #[inline] pub fn write_cnth(&self, value: Cnth) -> &Self { 
        self.cnth_reg().write(value);
        self
    }

    #[doc="Set the CNTH register."]
    #[inline] pub fn set_cnth<F: FnOnce(Cnth) -> Cnth>(&self, f: F) -> &Self {
        self.cnth_reg().set(f);
        self
    }

    #[doc="Modify the CNTH register."]
    #[inline] pub fn with_cnth<F: FnOnce(Cnth) -> Cnth>(&self, f: F) -> &Self {
        self.cnth_reg().with(f);
        self
    }

    #[doc="Get the CNTL Register."]
    #[inline] pub fn cntl_reg(&self) -> ::bobbin_mcu::register::Register<Cntl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cntl, 0x3)
    }

    #[doc="Get the *mut pointer for the CNTL register."]
    #[inline] pub fn cntl_mut(&self) -> *mut Cntl { 
        self.cntl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CNTL register."]
    #[inline] pub fn cntl_ptr(&self) -> *const Cntl { 
        self.cntl_reg().ptr()
    }

    #[doc="Read the CNTL register."]
    #[inline] pub fn cntl(&self) -> Cntl { 
        self.cntl_reg().read()
    }

    #[doc="Write the CNTL register."]
    #[inline] pub fn write_cntl(&self, value: Cntl) -> &Self { 
        self.cntl_reg().write(value);
        self
    }

    #[doc="Set the CNTL register."]
    #[inline] pub fn set_cntl<F: FnOnce(Cntl) -> Cntl>(&self, f: F) -> &Self {
        self.cntl_reg().set(f);
        self
    }

    #[doc="Modify the CNTL register."]
    #[inline] pub fn with_cntl<F: FnOnce(Cntl) -> Cntl>(&self, f: F) -> &Self {
        self.cntl_reg().with(f);
        self
    }

    #[doc="Get the TOVALH Register."]
    #[inline] pub fn tovalh_reg(&self) -> ::bobbin_mcu::register::Register<Tovalh> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tovalh, 0x4)
    }

    #[doc="Get the *mut pointer for the TOVALH register."]
    #[inline] pub fn tovalh_mut(&self) -> *mut Tovalh { 
        self.tovalh_reg().ptr()
    }

    #[doc="Get the *const pointer for the TOVALH register."]
    #[inline] pub fn tovalh_ptr(&self) -> *const Tovalh { 
        self.tovalh_reg().ptr()
    }

    #[doc="Read the TOVALH register."]
    #[inline] pub fn tovalh(&self) -> Tovalh { 
        self.tovalh_reg().read()
    }

    #[doc="Write the TOVALH register."]
    #[inline] pub fn write_tovalh(&self, value: Tovalh) -> &Self { 
        self.tovalh_reg().write(value);
        self
    }

    #[doc="Set the TOVALH register."]
    #[inline] pub fn set_tovalh<F: FnOnce(Tovalh) -> Tovalh>(&self, f: F) -> &Self {
        self.tovalh_reg().set(f);
        self
    }

    #[doc="Modify the TOVALH register."]
    #[inline] pub fn with_tovalh<F: FnOnce(Tovalh) -> Tovalh>(&self, f: F) -> &Self {
        self.tovalh_reg().with(f);
        self
    }

    #[doc="Get the TOVALL Register."]
    #[inline] pub fn tovall_reg(&self) -> ::bobbin_mcu::register::Register<Tovall> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Tovall, 0x5)
    }

    #[doc="Get the *mut pointer for the TOVALL register."]
    #[inline] pub fn tovall_mut(&self) -> *mut Tovall { 
        self.tovall_reg().ptr()
    }

    #[doc="Get the *const pointer for the TOVALL register."]
    #[inline] pub fn tovall_ptr(&self) -> *const Tovall { 
        self.tovall_reg().ptr()
    }

    #[doc="Read the TOVALL register."]
    #[inline] pub fn tovall(&self) -> Tovall { 
        self.tovall_reg().read()
    }

    #[doc="Write the TOVALL register."]
    #[inline] pub fn write_tovall(&self, value: Tovall) -> &Self { 
        self.tovall_reg().write(value);
        self
    }

    #[doc="Set the TOVALL register."]
    #[inline] pub fn set_tovall<F: FnOnce(Tovall) -> Tovall>(&self, f: F) -> &Self {
        self.tovall_reg().set(f);
        self
    }

    #[doc="Modify the TOVALL register."]
    #[inline] pub fn with_tovall<F: FnOnce(Tovall) -> Tovall>(&self, f: F) -> &Self {
        self.tovall_reg().with(f);
        self
    }

    #[doc="Get the WINH Register."]
    #[inline] pub fn winh_reg(&self) -> ::bobbin_mcu::register::Register<Winh> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Winh, 0x6)
    }

    #[doc="Get the *mut pointer for the WINH register."]
    #[inline] pub fn winh_mut(&self) -> *mut Winh { 
        self.winh_reg().ptr()
    }

    #[doc="Get the *const pointer for the WINH register."]
    #[inline] pub fn winh_ptr(&self) -> *const Winh { 
        self.winh_reg().ptr()
    }

    #[doc="Read the WINH register."]
    #[inline] pub fn winh(&self) -> Winh { 
        self.winh_reg().read()
    }

    #[doc="Write the WINH register."]
    #[inline] pub fn write_winh(&self, value: Winh) -> &Self { 
        self.winh_reg().write(value);
        self
    }

    #[doc="Set the WINH register."]
    #[inline] pub fn set_winh<F: FnOnce(Winh) -> Winh>(&self, f: F) -> &Self {
        self.winh_reg().set(f);
        self
    }

    #[doc="Modify the WINH register."]
    #[inline] pub fn with_winh<F: FnOnce(Winh) -> Winh>(&self, f: F) -> &Self {
        self.winh_reg().with(f);
        self
    }

    #[doc="Get the WINL Register."]
    #[inline] pub fn winl_reg(&self) -> ::bobbin_mcu::register::Register<Winl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Winl, 0x7)
    }

    #[doc="Get the *mut pointer for the WINL register."]
    #[inline] pub fn winl_mut(&self) -> *mut Winl { 
        self.winl_reg().ptr()
    }

    #[doc="Get the *const pointer for the WINL register."]
    #[inline] pub fn winl_ptr(&self) -> *const Winl { 
        self.winl_reg().ptr()
    }

    #[doc="Read the WINL register."]
    #[inline] pub fn winl(&self) -> Winl { 
        self.winl_reg().read()
    }

    #[doc="Write the WINL register."]
    #[inline] pub fn write_winl(&self, value: Winl) -> &Self { 
        self.winl_reg().write(value);
        self
    }

    #[doc="Set the WINL register."]
    #[inline] pub fn set_winl<F: FnOnce(Winl) -> Winl>(&self, f: F) -> &Self {
        self.winl_reg().set(f);
        self
    }

    #[doc="Modify the WINL register."]
    #[inline] pub fn with_winl<F: FnOnce(Winl) -> Winl>(&self, f: F) -> &Self {
        self.winl_reg().with(f);
        self
    }

}

#[doc="Watchdog Control and Status Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cs1(pub u8);
impl Cs1 {
    #[doc="Stop Enable"]
    #[inline] pub fn stop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if STOP != 0"]
    #[inline] pub fn test_stop(&self) -> bool {
        self.stop() != 0
    }

    #[doc="Sets the STOP field."]
    #[inline] pub fn set_stop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Wait Enable"]
    #[inline] pub fn wait(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WAIT != 0"]
    #[inline] pub fn test_wait(&self) -> bool {
        self.wait() != 0
    }

    #[doc="Sets the WAIT field."]
    #[inline] pub fn set_wait<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Debug Enable"]
    #[inline] pub fn dbg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DBG != 0"]
    #[inline] pub fn test_dbg(&self) -> bool {
        self.dbg() != 0
    }

    #[doc="Sets the DBG field."]
    #[inline] pub fn set_dbg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Watchdog Test"]
    #[inline] pub fn tst(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if TST != 0"]
    #[inline] pub fn test_tst(&self) -> bool {
        self.tst() != 0
    }

    #[doc="Sets the TST field."]
    #[inline] pub fn set_tst<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Allow updates"]
    #[inline] pub fn update(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if UPDATE != 0"]
    #[inline] pub fn test_update(&self) -> bool {
        self.update() != 0
    }

    #[doc="Sets the UPDATE field."]
    #[inline] pub fn set_update<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Watchdog Interrupt"]
    #[inline] pub fn int(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if INT != 0"]
    #[inline] pub fn test_int(&self) -> bool {
        self.int() != 0
    }

    #[doc="Sets the INT field."]
    #[inline] pub fn set_int<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Watchdog Enable"]
    #[inline] pub fn en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if EN != 0"]
    #[inline] pub fn test_en(&self) -> bool {
        self.en() != 0
    }

    #[doc="Sets the EN field."]
    #[inline] pub fn set_en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Cs1 {
    #[inline]
    fn from(other: u8) -> Self {
         Cs1(other)
    }
}

impl ::core::fmt::Display for Cs1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cs1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stop() != 0 { try!(write!(f, " stop"))}
        if self.wait() != 0 { try!(write!(f, " wait"))}
        if self.dbg() != 0 { try!(write!(f, " dbg"))}
        if self.tst() != 0 { try!(write!(f, " tst=0x{:x}", self.tst()))}
        if self.update() != 0 { try!(write!(f, " update"))}
        if self.int() != 0 { try!(write!(f, " int"))}
        if self.en() != 0 { try!(write!(f, " en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Control and Status Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cs2(pub u8);
impl Cs2 {
    #[doc="Watchdog Clock"]
    #[inline] pub fn clk(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if CLK != 0"]
    #[inline] pub fn test_clk(&self) -> bool {
        self.clk() != 0
    }

    #[doc="Sets the CLK field."]
    #[inline] pub fn set_clk<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Watchdog Prescalar"]
    #[inline] pub fn pres(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PRES != 0"]
    #[inline] pub fn test_pres(&self) -> bool {
        self.pres() != 0
    }

    #[doc="Sets the PRES field."]
    #[inline] pub fn set_pres<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Watchdog Interrupt Flag"]
    #[inline] pub fn flg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FLG != 0"]
    #[inline] pub fn test_flg(&self) -> bool {
        self.flg() != 0
    }

    #[doc="Sets the FLG field."]
    #[inline] pub fn set_flg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Watchdog Window"]
    #[inline] pub fn win(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if WIN != 0"]
    #[inline] pub fn test_win(&self) -> bool {
        self.win() != 0
    }

    #[doc="Sets the WIN field."]
    #[inline] pub fn set_win<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Cs2 {
    #[inline]
    fn from(other: u8) -> Self {
         Cs2(other)
    }
}

impl ::core::fmt::Display for Cs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cs2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.clk() != 0 { try!(write!(f, " clk=0x{:x}", self.clk()))}
        if self.pres() != 0 { try!(write!(f, " pres"))}
        if self.flg() != 0 { try!(write!(f, " flg"))}
        if self.win() != 0 { try!(write!(f, " win"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Counter Register: High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnth(pub u8);
impl Cnth {
    #[doc="High byte of the Watchdog Counter"]
    #[inline] pub fn cnthigh(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CNTHIGH != 0"]
    #[inline] pub fn test_cnthigh(&self) -> bool {
        self.cnthigh() != 0
    }

    #[doc="Sets the CNTHIGH field."]
    #[inline] pub fn set_cnthigh<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Cnth {
    #[inline]
    fn from(other: u8) -> Self {
         Cnth(other)
    }
}

impl ::core::fmt::Display for Cnth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cnth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cnthigh() != 0 { try!(write!(f, " cnthigh=0x{:x}", self.cnthigh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Counter Register: Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cntl(pub u8);
impl Cntl {
    #[doc="Low byte of the Watchdog Counter"]
    #[inline] pub fn cntlow(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CNTLOW != 0"]
    #[inline] pub fn test_cntlow(&self) -> bool {
        self.cntlow() != 0
    }

    #[doc="Sets the CNTLOW field."]
    #[inline] pub fn set_cntlow<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Cntl {
    #[inline]
    fn from(other: u8) -> Self {
         Cntl(other)
    }
}

impl ::core::fmt::Display for Cntl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cntl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cntlow() != 0 { try!(write!(f, " cntlow=0x{:x}", self.cntlow()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Timeout Value Register: High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tovalh(pub u8);
impl Tovalh {
    #[doc="High byte of the timeout value"]
    #[inline] pub fn tovalhigh(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TOVALHIGH != 0"]
    #[inline] pub fn test_tovalhigh(&self) -> bool {
        self.tovalhigh() != 0
    }

    #[doc="Sets the TOVALHIGH field."]
    #[inline] pub fn set_tovalhigh<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Tovalh {
    #[inline]
    fn from(other: u8) -> Self {
         Tovalh(other)
    }
}

impl ::core::fmt::Display for Tovalh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tovalh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tovalhigh() != 0 { try!(write!(f, " tovalhigh=0x{:x}", self.tovalhigh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Timeout Value Register: Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tovall(pub u8);
impl Tovall {
    #[doc="Low byte of the timeout value"]
    #[inline] pub fn tovallow(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TOVALLOW != 0"]
    #[inline] pub fn test_tovallow(&self) -> bool {
        self.tovallow() != 0
    }

    #[doc="Sets the TOVALLOW field."]
    #[inline] pub fn set_tovallow<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Tovall {
    #[inline]
    fn from(other: u8) -> Self {
         Tovall(other)
    }
}

impl ::core::fmt::Display for Tovall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tovall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tovallow() != 0 { try!(write!(f, " tovallow=0x{:x}", self.tovallow()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Window Register: High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Winh(pub u8);
impl Winh {
    #[doc="High byte of Watchdog Window"]
    #[inline] pub fn winhigh(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if WINHIGH != 0"]
    #[inline] pub fn test_winhigh(&self) -> bool {
        self.winhigh() != 0
    }

    #[doc="Sets the WINHIGH field."]
    #[inline] pub fn set_winhigh<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Winh {
    #[inline]
    fn from(other: u8) -> Self {
         Winh(other)
    }
}

impl ::core::fmt::Display for Winh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Winh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.winhigh() != 0 { try!(write!(f, " winhigh=0x{:x}", self.winhigh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Watchdog Window Register: Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Winl(pub u8);
impl Winl {
    #[doc="Low byte of Watchdog Window"]
    #[inline] pub fn winlow(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if WINLOW != 0"]
    #[inline] pub fn test_winlow(&self) -> bool {
        self.winlow() != 0
    }

    #[doc="Sets the WINLOW field."]
    #[inline] pub fn set_winlow<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Winl {
    #[inline]
    fn from(other: u8) -> Self {
         Winl(other)
    }
}

impl ::core::fmt::Display for Winl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Winl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.winlow() != 0 { try!(write!(f, " winlow=0x{:x}", self.winlow()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

