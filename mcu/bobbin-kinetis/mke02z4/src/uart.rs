::bobbin_mcu::periph!( UART0, Uart0, UART0_PERIPH, UartPeriph, UART0_OWNED, UART0_REF_COUNT, 0x4006a000, 0x00, 0x0f);
::bobbin_mcu::periph!( UART1, Uart1, UART1_PERIPH, UartPeriph, UART1_OWNED, UART1_REF_COUNT, 0x4006b000, 0x01, 0x10);
::bobbin_mcu::periph!( UART2, Uart2, UART2_PERIPH, UartPeriph, UART2_OWNED, UART2_REF_COUNT, 0x4006c000, 0x02, 0x11);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="UART Peripheral"]
pub struct UartPeriph(pub usize); 

impl UartPeriph {
    #[doc="Get the BDH Register."]
    #[inline] pub fn bdh_reg(&self) -> ::bobbin_mcu::register::Register<Bdh> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bdh, 0x0)
    }

    #[doc="Get the *mut pointer for the BDH register."]
    #[inline] pub fn bdh_mut(&self) -> *mut Bdh { 
        self.bdh_reg().ptr()
    }

    #[doc="Get the *const pointer for the BDH register."]
    #[inline] pub fn bdh_ptr(&self) -> *const Bdh { 
        self.bdh_reg().ptr()
    }

    #[doc="Read the BDH register."]
    #[inline] pub fn bdh(&self) -> Bdh { 
        self.bdh_reg().read()
    }

    #[doc="Write the BDH register."]
    #[inline] pub fn write_bdh(&self, value: Bdh) -> &Self { 
        self.bdh_reg().write(value);
        self
    }

    #[doc="Set the BDH register."]
    #[inline] pub fn set_bdh<F: FnOnce(Bdh) -> Bdh>(&self, f: F) -> &Self {
        self.bdh_reg().set(f);
        self
    }

    #[doc="Modify the BDH register."]
    #[inline] pub fn with_bdh<F: FnOnce(Bdh) -> Bdh>(&self, f: F) -> &Self {
        self.bdh_reg().with(f);
        self
    }

    #[doc="Get the BDL Register."]
    #[inline] pub fn bdl_reg(&self) -> ::bobbin_mcu::register::Register<Bdl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Bdl, 0x1)
    }

    #[doc="Get the *mut pointer for the BDL register."]
    #[inline] pub fn bdl_mut(&self) -> *mut Bdl { 
        self.bdl_reg().ptr()
    }

    #[doc="Get the *const pointer for the BDL register."]
    #[inline] pub fn bdl_ptr(&self) -> *const Bdl { 
        self.bdl_reg().ptr()
    }

    #[doc="Read the BDL register."]
    #[inline] pub fn bdl(&self) -> Bdl { 
        self.bdl_reg().read()
    }

    #[doc="Write the BDL register."]
    #[inline] pub fn write_bdl(&self, value: Bdl) -> &Self { 
        self.bdl_reg().write(value);
        self
    }

    #[doc="Set the BDL register."]
    #[inline] pub fn set_bdl<F: FnOnce(Bdl) -> Bdl>(&self, f: F) -> &Self {
        self.bdl_reg().set(f);
        self
    }

    #[doc="Modify the BDL register."]
    #[inline] pub fn with_bdl<F: FnOnce(Bdl) -> Bdl>(&self, f: F) -> &Self {
        self.bdl_reg().with(f);
        self
    }

    #[doc="Get the C1 Register."]
    #[inline] pub fn c1_reg(&self) -> ::bobbin_mcu::register::Register<C1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C1, 0x2)
    }

    #[doc="Get the *mut pointer for the C1 register."]
    #[inline] pub fn c1_mut(&self) -> *mut C1 { 
        self.c1_reg().ptr()
    }

    #[doc="Get the *const pointer for the C1 register."]
    #[inline] pub fn c1_ptr(&self) -> *const C1 { 
        self.c1_reg().ptr()
    }

    #[doc="Read the C1 register."]
    #[inline] pub fn c1(&self) -> C1 { 
        self.c1_reg().read()
    }

    #[doc="Write the C1 register."]
    #[inline] pub fn write_c1(&self, value: C1) -> &Self { 
        self.c1_reg().write(value);
        self
    }

    #[doc="Set the C1 register."]
    #[inline] pub fn set_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
        self.c1_reg().set(f);
        self
    }

    #[doc="Modify the C1 register."]
    #[inline] pub fn with_c1<F: FnOnce(C1) -> C1>(&self, f: F) -> &Self {
        self.c1_reg().with(f);
        self
    }

    #[doc="Get the C2 Register."]
    #[inline] pub fn c2_reg(&self) -> ::bobbin_mcu::register::Register<C2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C2, 0x3)
    }

    #[doc="Get the *mut pointer for the C2 register."]
    #[inline] pub fn c2_mut(&self) -> *mut C2 { 
        self.c2_reg().ptr()
    }

    #[doc="Get the *const pointer for the C2 register."]
    #[inline] pub fn c2_ptr(&self) -> *const C2 { 
        self.c2_reg().ptr()
    }

    #[doc="Read the C2 register."]
    #[inline] pub fn c2(&self) -> C2 { 
        self.c2_reg().read()
    }

    #[doc="Write the C2 register."]
    #[inline] pub fn write_c2(&self, value: C2) -> &Self { 
        self.c2_reg().write(value);
        self
    }

    #[doc="Set the C2 register."]
    #[inline] pub fn set_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
        self.c2_reg().set(f);
        self
    }

    #[doc="Modify the C2 register."]
    #[inline] pub fn with_c2<F: FnOnce(C2) -> C2>(&self, f: F) -> &Self {
        self.c2_reg().with(f);
        self
    }

    #[doc="Get the S1 Register."]
    #[inline] pub fn s1_reg(&self) -> ::bobbin_mcu::register::Register<S1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut S1, 0x4)
    }

    #[doc="Get the *mut pointer for the S1 register."]
    #[inline] pub fn s1_mut(&self) -> *mut S1 { 
        self.s1_reg().ptr()
    }

    #[doc="Get the *const pointer for the S1 register."]
    #[inline] pub fn s1_ptr(&self) -> *const S1 { 
        self.s1_reg().ptr()
    }

    #[doc="Read the S1 register."]
    #[inline] pub fn s1(&self) -> S1 { 
        self.s1_reg().read()
    }

    #[doc="Get the S2 Register."]
    #[inline] pub fn s2_reg(&self) -> ::bobbin_mcu::register::Register<S2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut S2, 0x5)
    }

    #[doc="Get the *mut pointer for the S2 register."]
    #[inline] pub fn s2_mut(&self) -> *mut S2 { 
        self.s2_reg().ptr()
    }

    #[doc="Get the *const pointer for the S2 register."]
    #[inline] pub fn s2_ptr(&self) -> *const S2 { 
        self.s2_reg().ptr()
    }

    #[doc="Read the S2 register."]
    #[inline] pub fn s2(&self) -> S2 { 
        self.s2_reg().read()
    }

    #[doc="Write the S2 register."]
    #[inline] pub fn write_s2(&self, value: S2) -> &Self { 
        self.s2_reg().write(value);
        self
    }

    #[doc="Set the S2 register."]
    #[inline] pub fn set_s2<F: FnOnce(S2) -> S2>(&self, f: F) -> &Self {
        self.s2_reg().set(f);
        self
    }

    #[doc="Modify the S2 register."]
    #[inline] pub fn with_s2<F: FnOnce(S2) -> S2>(&self, f: F) -> &Self {
        self.s2_reg().with(f);
        self
    }

    #[doc="Get the C3 Register."]
    #[inline] pub fn c3_reg(&self) -> ::bobbin_mcu::register::Register<C3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut C3, 0x6)
    }

    #[doc="Get the *mut pointer for the C3 register."]
    #[inline] pub fn c3_mut(&self) -> *mut C3 { 
        self.c3_reg().ptr()
    }

    #[doc="Get the *const pointer for the C3 register."]
    #[inline] pub fn c3_ptr(&self) -> *const C3 { 
        self.c3_reg().ptr()
    }

    #[doc="Read the C3 register."]
    #[inline] pub fn c3(&self) -> C3 { 
        self.c3_reg().read()
    }

    #[doc="Write the C3 register."]
    #[inline] pub fn write_c3(&self, value: C3) -> &Self { 
        self.c3_reg().write(value);
        self
    }

    #[doc="Set the C3 register."]
    #[inline] pub fn set_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
        self.c3_reg().set(f);
        self
    }

    #[doc="Modify the C3 register."]
    #[inline] pub fn with_c3<F: FnOnce(C3) -> C3>(&self, f: F) -> &Self {
        self.c3_reg().with(f);
        self
    }

    #[doc="Get the D Register."]
    #[inline] pub fn d_reg(&self) -> ::bobbin_mcu::register::Register<D> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut D, 0x7)
    }

    #[doc="Get the *mut pointer for the D register."]
    #[inline] pub fn d_mut(&self) -> *mut D { 
        self.d_reg().ptr()
    }

    #[doc="Get the *const pointer for the D register."]
    #[inline] pub fn d_ptr(&self) -> *const D { 
        self.d_reg().ptr()
    }

    #[doc="Read the D register."]
    #[inline] pub fn d(&self) -> D { 
        self.d_reg().read()
    }

    #[doc="Write the D register."]
    #[inline] pub fn write_d(&self, value: D) -> &Self { 
        self.d_reg().write(value);
        self
    }

    #[doc="Set the D register."]
    #[inline] pub fn set_d<F: FnOnce(D) -> D>(&self, f: F) -> &Self {
        self.d_reg().set(f);
        self
    }

    #[doc="Modify the D register."]
    #[inline] pub fn with_d<F: FnOnce(D) -> D>(&self, f: F) -> &Self {
        self.d_reg().with(f);
        self
    }

}

#[doc="UART Baud Rate Register: High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdh(pub u8);
impl Bdh {
    #[doc="Baud Rate Modulo Divisor."]
    #[inline] pub fn sbr(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if SBR != 0"]
    #[inline] pub fn test_sbr(&self) -> bool {
        self.sbr() != 0
    }

    #[doc="Sets the SBR field."]
    #[inline] pub fn set_sbr<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Stop Bit Number Select"]
    #[inline] pub fn sbns(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SBNS != 0"]
    #[inline] pub fn test_sbns(&self) -> bool {
        self.sbns() != 0
    }

    #[doc="Sets the SBNS field."]
    #[inline] pub fn set_sbns<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="RxD Input Active Edge Interrupt Enable (for RXEDGIF)"]
    #[inline] pub fn rxedgie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXEDGIE != 0"]
    #[inline] pub fn test_rxedgie(&self) -> bool {
        self.rxedgie() != 0
    }

    #[doc="Sets the RXEDGIE field."]
    #[inline] pub fn set_rxedgie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="LIN Break Detect Interrupt Enable (for LBKDIF)"]
    #[inline] pub fn lbkdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LBKDIE != 0"]
    #[inline] pub fn test_lbkdie(&self) -> bool {
        self.lbkdie() != 0
    }

    #[doc="Sets the LBKDIE field."]
    #[inline] pub fn set_lbkdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Bdh {
    #[inline]
    fn from(other: u8) -> Self {
         Bdh(other)
    }
}

impl ::core::fmt::Display for Bdh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sbr() != 0 { try!(write!(f, " sbr=0x{:x}", self.sbr()))}
        if self.sbns() != 0 { try!(write!(f, " sbns"))}
        if self.rxedgie() != 0 { try!(write!(f, " rxedgie"))}
        if self.lbkdie() != 0 { try!(write!(f, " lbkdie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Baud Rate Register: Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Bdl(pub u8);
impl Bdl {
    #[doc="Baud Rate Modulo Divisor"]
    #[inline] pub fn sbr(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if SBR != 0"]
    #[inline] pub fn test_sbr(&self) -> bool {
        self.sbr() != 0
    }

    #[doc="Sets the SBR field."]
    #[inline] pub fn set_sbr<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Bdl {
    #[inline]
    fn from(other: u8) -> Self {
         Bdl(other)
    }
}

impl ::core::fmt::Display for Bdl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Bdl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sbr() != 0 { try!(write!(f, " sbr=0x{:x}", self.sbr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Control Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C1(pub u8);
impl C1 {
    #[doc="Parity Type"]
    #[inline] pub fn pt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PT != 0"]
    #[inline] pub fn test_pt(&self) -> bool {
        self.pt() != 0
    }

    #[doc="Sets the PT field."]
    #[inline] pub fn set_pt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Parity Enable"]
    #[inline] pub fn pe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PE != 0"]
    #[inline] pub fn test_pe(&self) -> bool {
        self.pe() != 0
    }

    #[doc="Sets the PE field."]
    #[inline] pub fn set_pe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Idle Line Type Select"]
    #[inline] pub fn ilt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ILT != 0"]
    #[inline] pub fn test_ilt(&self) -> bool {
        self.ilt() != 0
    }

    #[doc="Sets the ILT field."]
    #[inline] pub fn set_ilt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receiver Wakeup Method Select"]
    #[inline] pub fn wake(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if WAKE != 0"]
    #[inline] pub fn test_wake(&self) -> bool {
        self.wake() != 0
    }

    #[doc="Sets the WAKE field."]
    #[inline] pub fn set_wake<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="9-Bit or 8-Bit Mode Select"]
    #[inline] pub fn m(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if M != 0"]
    #[inline] pub fn test_m(&self) -> bool {
        self.m() != 0
    }

    #[doc="Sets the M field."]
    #[inline] pub fn set_m<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receiver Source Select"]
    #[inline] pub fn rsrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RSRC != 0"]
    #[inline] pub fn test_rsrc(&self) -> bool {
        self.rsrc() != 0
    }

    #[doc="Sets the RSRC field."]
    #[inline] pub fn set_rsrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="UART Stops in Wait Mode"]
    #[inline] pub fn uartswai(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if UARTSWAI != 0"]
    #[inline] pub fn test_uartswai(&self) -> bool {
        self.uartswai() != 0
    }

    #[doc="Sets the UARTSWAI field."]
    #[inline] pub fn set_uartswai<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Loop Mode Select"]
    #[inline] pub fn loops(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LOOPS != 0"]
    #[inline] pub fn test_loops(&self) -> bool {
        self.loops() != 0
    }

    #[doc="Sets the LOOPS field."]
    #[inline] pub fn set_loops<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.pt() != 0 { try!(write!(f, " pt"))}
        if self.pe() != 0 { try!(write!(f, " pe"))}
        if self.ilt() != 0 { try!(write!(f, " ilt"))}
        if self.wake() != 0 { try!(write!(f, " wake"))}
        if self.m() != 0 { try!(write!(f, " m"))}
        if self.rsrc() != 0 { try!(write!(f, " rsrc"))}
        if self.uartswai() != 0 { try!(write!(f, " uartswai"))}
        if self.loops() != 0 { try!(write!(f, " loops"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Control Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C2(pub u8);
impl C2 {
    #[doc="Send Break"]
    #[inline] pub fn sbk(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SBK != 0"]
    #[inline] pub fn test_sbk(&self) -> bool {
        self.sbk() != 0
    }

    #[doc="Sets the SBK field."]
    #[inline] pub fn set_sbk<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Receiver Wakeup Control"]
    #[inline] pub fn rwu(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RWU != 0"]
    #[inline] pub fn test_rwu(&self) -> bool {
        self.rwu() != 0
    }

    #[doc="Sets the RWU field."]
    #[inline] pub fn set_rwu<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Receiver Enable"]
    #[inline] pub fn re(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RE != 0"]
    #[inline] pub fn test_re(&self) -> bool {
        self.re() != 0
    }

    #[doc="Sets the RE field."]
    #[inline] pub fn set_re<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Transmitter Enable"]
    #[inline] pub fn te(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TE != 0"]
    #[inline] pub fn test_te(&self) -> bool {
        self.te() != 0
    }

    #[doc="Sets the TE field."]
    #[inline] pub fn set_te<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Idle Line Interrupt Enable for IDLE"]
    #[inline] pub fn ilie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ILIE != 0"]
    #[inline] pub fn test_ilie(&self) -> bool {
        self.ilie() != 0
    }

    #[doc="Sets the ILIE field."]
    #[inline] pub fn set_ilie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receiver Interrupt Enable for RDRF"]
    #[inline] pub fn rie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RIE != 0"]
    #[inline] pub fn test_rie(&self) -> bool {
        self.rie() != 0
    }

    #[doc="Sets the RIE field."]
    #[inline] pub fn set_rie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transmission Complete Interrupt Enable for TC"]
    #[inline] pub fn tcie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TCIE != 0"]
    #[inline] pub fn test_tcie(&self) -> bool {
        self.tcie() != 0
    }

    #[doc="Sets the TCIE field."]
    #[inline] pub fn set_tcie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit Interrupt Enable for TDRE"]
    #[inline] pub fn tie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TIE != 0"]
    #[inline] pub fn test_tie(&self) -> bool {
        self.tie() != 0
    }

    #[doc="Sets the TIE field."]
    #[inline] pub fn set_tie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
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
        if self.sbk() != 0 { try!(write!(f, " sbk"))}
        if self.rwu() != 0 { try!(write!(f, " rwu"))}
        if self.re() != 0 { try!(write!(f, " re"))}
        if self.te() != 0 { try!(write!(f, " te"))}
        if self.ilie() != 0 { try!(write!(f, " ilie"))}
        if self.rie() != 0 { try!(write!(f, " rie"))}
        if self.tcie() != 0 { try!(write!(f, " tcie"))}
        if self.tie() != 0 { try!(write!(f, " tie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Status Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct S1(pub u8);
impl S1 {
    #[doc="Parity Error Flag"]
    #[inline] pub fn pf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PF != 0"]
    #[inline] pub fn test_pf(&self) -> bool {
        self.pf() != 0
    }

    #[doc="Sets the PF field."]
    #[inline] pub fn set_pf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Framing Error Flag"]
    #[inline] pub fn fe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FE != 0"]
    #[inline] pub fn test_fe(&self) -> bool {
        self.fe() != 0
    }

    #[doc="Sets the FE field."]
    #[inline] pub fn set_fe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Noise Flag"]
    #[inline] pub fn nf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NF != 0"]
    #[inline] pub fn test_nf(&self) -> bool {
        self.nf() != 0
    }

    #[doc="Sets the NF field."]
    #[inline] pub fn set_nf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receiver Overrun Flag"]
    #[inline] pub fn or(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if OR != 0"]
    #[inline] pub fn test_or(&self) -> bool {
        self.or() != 0
    }

    #[doc="Sets the OR field."]
    #[inline] pub fn set_or<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Idle Line Flag"]
    #[inline] pub fn idle(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IDLE != 0"]
    #[inline] pub fn test_idle(&self) -> bool {
        self.idle() != 0
    }

    #[doc="Sets the IDLE field."]
    #[inline] pub fn set_idle<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Receive Data Register Full Flag"]
    #[inline] pub fn rdrf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RDRF != 0"]
    #[inline] pub fn test_rdrf(&self) -> bool {
        self.rdrf() != 0
    }

    #[doc="Sets the RDRF field."]
    #[inline] pub fn set_rdrf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Transmission Complete Flag"]
    #[inline] pub fn tc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TC != 0"]
    #[inline] pub fn test_tc(&self) -> bool {
        self.tc() != 0
    }

    #[doc="Sets the TC field."]
    #[inline] pub fn set_tc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Transmit Data Register Empty Flag"]
    #[inline] pub fn tdre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TDRE != 0"]
    #[inline] pub fn test_tdre(&self) -> bool {
        self.tdre() != 0
    }

    #[doc="Sets the TDRE field."]
    #[inline] pub fn set_tdre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for S1 {
    #[inline]
    fn from(other: u8) -> Self {
         S1(other)
    }
}

impl ::core::fmt::Display for S1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for S1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pf() != 0 { try!(write!(f, " pf"))}
        if self.fe() != 0 { try!(write!(f, " fe"))}
        if self.nf() != 0 { try!(write!(f, " nf"))}
        if self.or() != 0 { try!(write!(f, " or"))}
        if self.idle() != 0 { try!(write!(f, " idle"))}
        if self.rdrf() != 0 { try!(write!(f, " rdrf"))}
        if self.tc() != 0 { try!(write!(f, " tc"))}
        if self.tdre() != 0 { try!(write!(f, " tdre"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Status Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct S2(pub u8);
impl S2 {
    #[doc="Receiver Active Flag"]
    #[inline] pub fn raf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RAF != 0"]
    #[inline] pub fn test_raf(&self) -> bool {
        self.raf() != 0
    }

    #[doc="Sets the RAF field."]
    #[inline] pub fn set_raf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="LIN Break Detection Enable"]
    #[inline] pub fn lbkde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LBKDE != 0"]
    #[inline] pub fn test_lbkde(&self) -> bool {
        self.lbkde() != 0
    }

    #[doc="Sets the LBKDE field."]
    #[inline] pub fn set_lbkde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Break Character Generation Length"]
    #[inline] pub fn brk13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BRK13 != 0"]
    #[inline] pub fn test_brk13(&self) -> bool {
        self.brk13() != 0
    }

    #[doc="Sets the BRK13 field."]
    #[inline] pub fn set_brk13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Receive Wake Up Idle Detect"]
    #[inline] pub fn rwuid(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RWUID != 0"]
    #[inline] pub fn test_rwuid(&self) -> bool {
        self.rwuid() != 0
    }

    #[doc="Sets the RWUID field."]
    #[inline] pub fn set_rwuid<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Receive Data Inversion"]
    #[inline] pub fn rxinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RXINV != 0"]
    #[inline] pub fn test_rxinv(&self) -> bool {
        self.rxinv() != 0
    }

    #[doc="Sets the RXINV field."]
    #[inline] pub fn set_rxinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="RxD Pin Active Edge Interrupt Flag"]
    #[inline] pub fn rxedgif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXEDGIF != 0"]
    #[inline] pub fn test_rxedgif(&self) -> bool {
        self.rxedgif() != 0
    }

    #[doc="Sets the RXEDGIF field."]
    #[inline] pub fn set_rxedgif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="LIN Break Detect Interrupt Flag"]
    #[inline] pub fn lbkdif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if LBKDIF != 0"]
    #[inline] pub fn test_lbkdif(&self) -> bool {
        self.lbkdif() != 0
    }

    #[doc="Sets the LBKDIF field."]
    #[inline] pub fn set_lbkdif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for S2 {
    #[inline]
    fn from(other: u8) -> Self {
         S2(other)
    }
}

impl ::core::fmt::Display for S2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for S2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.raf() != 0 { try!(write!(f, " raf"))}
        if self.lbkde() != 0 { try!(write!(f, " lbkde"))}
        if self.brk13() != 0 { try!(write!(f, " brk13"))}
        if self.rwuid() != 0 { try!(write!(f, " rwuid"))}
        if self.rxinv() != 0 { try!(write!(f, " rxinv"))}
        if self.rxedgif() != 0 { try!(write!(f, " rxedgif"))}
        if self.lbkdif() != 0 { try!(write!(f, " lbkdif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Control Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct C3(pub u8);
impl C3 {
    #[doc="Parity Error Interrupt Enable"]
    #[inline] pub fn peie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PEIE != 0"]
    #[inline] pub fn test_peie(&self) -> bool {
        self.peie() != 0
    }

    #[doc="Sets the PEIE field."]
    #[inline] pub fn set_peie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Framing Error Interrupt Enable"]
    #[inline] pub fn feie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FEIE != 0"]
    #[inline] pub fn test_feie(&self) -> bool {
        self.feie() != 0
    }

    #[doc="Sets the FEIE field."]
    #[inline] pub fn set_feie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Noise Error Interrupt Enable"]
    #[inline] pub fn neie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if NEIE != 0"]
    #[inline] pub fn test_neie(&self) -> bool {
        self.neie() != 0
    }

    #[doc="Sets the NEIE field."]
    #[inline] pub fn set_neie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Overrun Interrupt Enable"]
    #[inline] pub fn orie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ORIE != 0"]
    #[inline] pub fn test_orie(&self) -> bool {
        self.orie() != 0
    }

    #[doc="Sets the ORIE field."]
    #[inline] pub fn set_orie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Transmit Data Inversion"]
    #[inline] pub fn txinv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TXINV != 0"]
    #[inline] pub fn test_txinv(&self) -> bool {
        self.txinv() != 0
    }

    #[doc="Sets the TXINV field."]
    #[inline] pub fn set_txinv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="TxD Pin Direction in Single-Wire Mode"]
    #[inline] pub fn txdir(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TXDIR != 0"]
    #[inline] pub fn test_txdir(&self) -> bool {
        self.txdir() != 0
    }

    #[doc="Sets the TXDIR field."]
    #[inline] pub fn set_txdir<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Ninth Data Bit for Transmitter"]
    #[inline] pub fn t8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if T8 != 0"]
    #[inline] pub fn test_t8(&self) -> bool {
        self.t8() != 0
    }

    #[doc="Sets the T8 field."]
    #[inline] pub fn set_t8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Ninth Data Bit for Receiver"]
    #[inline] pub fn r8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if R8 != 0"]
    #[inline] pub fn test_r8(&self) -> bool {
        self.r8() != 0
    }

    #[doc="Sets the R8 field."]
    #[inline] pub fn set_r8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.peie() != 0 { try!(write!(f, " peie"))}
        if self.feie() != 0 { try!(write!(f, " feie"))}
        if self.neie() != 0 { try!(write!(f, " neie"))}
        if self.orie() != 0 { try!(write!(f, " orie"))}
        if self.txinv() != 0 { try!(write!(f, " txinv"))}
        if self.txdir() != 0 { try!(write!(f, " txdir"))}
        if self.t8() != 0 { try!(write!(f, " t8"))}
        if self.r8() != 0 { try!(write!(f, " r8"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="UART Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct D(pub u8);
impl D {
    #[doc="Read receive data buffer 0 or write transmit data buffer 0."]
    #[inline] pub fn r0t0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if R0T0 != 0"]
    #[inline] pub fn test_r0t0(&self) -> bool {
        self.r0t0() != 0
    }

    #[doc="Sets the R0T0 field."]
    #[inline] pub fn set_r0t0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Read receive data buffer 1 or write transmit data buffer 1."]
    #[inline] pub fn r1t1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if R1T1 != 0"]
    #[inline] pub fn test_r1t1(&self) -> bool {
        self.r1t1() != 0
    }

    #[doc="Sets the R1T1 field."]
    #[inline] pub fn set_r1t1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Read receive data buffer 2 or write transmit data buffer 2."]
    #[inline] pub fn r2t2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if R2T2 != 0"]
    #[inline] pub fn test_r2t2(&self) -> bool {
        self.r2t2() != 0
    }

    #[doc="Sets the R2T2 field."]
    #[inline] pub fn set_r2t2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Read receive data buffer 3 or write transmit data buffer 3."]
    #[inline] pub fn r3t3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if R3T3 != 0"]
    #[inline] pub fn test_r3t3(&self) -> bool {
        self.r3t3() != 0
    }

    #[doc="Sets the R3T3 field."]
    #[inline] pub fn set_r3t3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Read receive data buffer 4 or write transmit data buffer 4."]
    #[inline] pub fn r4t4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if R4T4 != 0"]
    #[inline] pub fn test_r4t4(&self) -> bool {
        self.r4t4() != 0
    }

    #[doc="Sets the R4T4 field."]
    #[inline] pub fn set_r4t4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Read receive data buffer 5 or write transmit data buffer 5."]
    #[inline] pub fn r5t5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if R5T5 != 0"]
    #[inline] pub fn test_r5t5(&self) -> bool {
        self.r5t5() != 0
    }

    #[doc="Sets the R5T5 field."]
    #[inline] pub fn set_r5t5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Read receive data buffer 6 or write transmit data buffer 6."]
    #[inline] pub fn r6t6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if R6T6 != 0"]
    #[inline] pub fn test_r6t6(&self) -> bool {
        self.r6t6() != 0
    }

    #[doc="Sets the R6T6 field."]
    #[inline] pub fn set_r6t6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Read receive data buffer 7 or write transmit data buffer 7."]
    #[inline] pub fn r7t7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if R7T7 != 0"]
    #[inline] pub fn test_r7t7(&self) -> bool {
        self.r7t7() != 0
    }

    #[doc="Sets the R7T7 field."]
    #[inline] pub fn set_r7t7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for D {
    #[inline]
    fn from(other: u8) -> Self {
         D(other)
    }
}

impl ::core::fmt::Display for D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.r0t0() != 0 { try!(write!(f, " r0t0"))}
        if self.r1t1() != 0 { try!(write!(f, " r1t1"))}
        if self.r2t2() != 0 { try!(write!(f, " r2t2"))}
        if self.r3t3() != 0 { try!(write!(f, " r3t3"))}
        if self.r4t4() != 0 { try!(write!(f, " r4t4"))}
        if self.r5t5() != 0 { try!(write!(f, " r5t5"))}
        if self.r6t6() != 0 { try!(write!(f, " r6t6"))}
        if self.r7t7() != 0 { try!(write!(f, " r7t7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

