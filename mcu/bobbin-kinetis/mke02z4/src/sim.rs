::bobbin_mcu::periph!( SIM, Sim, SIM_PERIPH, SimPeriph, SIM_OWNED, SIM_REF_COUNT, 0x40048000, 0x00, 0x09);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="SIM Peripheral"]
pub struct SimPeriph(pub usize); 

impl SimPeriph {
    #[doc="Get the SRSID Register."]
    #[inline] pub fn srsid_reg(&self) -> ::bobbin_mcu::register::Register<Srsid> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Srsid, 0x0)
    }

    #[doc="Get the *mut pointer for the SRSID register."]
    #[inline] pub fn srsid_mut(&self) -> *mut Srsid { 
        self.srsid_reg().ptr()
    }

    #[doc="Get the *const pointer for the SRSID register."]
    #[inline] pub fn srsid_ptr(&self) -> *const Srsid { 
        self.srsid_reg().ptr()
    }

    #[doc="Read the SRSID register."]
    #[inline] pub fn srsid(&self) -> Srsid { 
        self.srsid_reg().read()
    }

    #[doc="Get the SOPT Register."]
    #[inline] pub fn sopt_reg(&self) -> ::bobbin_mcu::register::Register<Sopt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sopt, 0x4)
    }

    #[doc="Get the *mut pointer for the SOPT register."]
    #[inline] pub fn sopt_mut(&self) -> *mut Sopt { 
        self.sopt_reg().ptr()
    }

    #[doc="Get the *const pointer for the SOPT register."]
    #[inline] pub fn sopt_ptr(&self) -> *const Sopt { 
        self.sopt_reg().ptr()
    }

    #[doc="Read the SOPT register."]
    #[inline] pub fn sopt(&self) -> Sopt { 
        self.sopt_reg().read()
    }

    #[doc="Write the SOPT register."]
    #[inline] pub fn write_sopt(&self, value: Sopt) -> &Self { 
        self.sopt_reg().write(value);
        self
    }

    #[doc="Set the SOPT register."]
    #[inline] pub fn set_sopt<F: FnOnce(Sopt) -> Sopt>(&self, f: F) -> &Self {
        self.sopt_reg().set(f);
        self
    }

    #[doc="Modify the SOPT register."]
    #[inline] pub fn with_sopt<F: FnOnce(Sopt) -> Sopt>(&self, f: F) -> &Self {
        self.sopt_reg().with(f);
        self
    }

    #[doc="Get the PINSEL Register."]
    #[inline] pub fn pinsel_reg(&self) -> ::bobbin_mcu::register::Register<Pinsel> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pinsel, 0x8)
    }

    #[doc="Get the *mut pointer for the PINSEL register."]
    #[inline] pub fn pinsel_mut(&self) -> *mut Pinsel { 
        self.pinsel_reg().ptr()
    }

    #[doc="Get the *const pointer for the PINSEL register."]
    #[inline] pub fn pinsel_ptr(&self) -> *const Pinsel { 
        self.pinsel_reg().ptr()
    }

    #[doc="Read the PINSEL register."]
    #[inline] pub fn pinsel(&self) -> Pinsel { 
        self.pinsel_reg().read()
    }

    #[doc="Write the PINSEL register."]
    #[inline] pub fn write_pinsel(&self, value: Pinsel) -> &Self { 
        self.pinsel_reg().write(value);
        self
    }

    #[doc="Set the PINSEL register."]
    #[inline] pub fn set_pinsel<F: FnOnce(Pinsel) -> Pinsel>(&self, f: F) -> &Self {
        self.pinsel_reg().set(f);
        self
    }

    #[doc="Modify the PINSEL register."]
    #[inline] pub fn with_pinsel<F: FnOnce(Pinsel) -> Pinsel>(&self, f: F) -> &Self {
        self.pinsel_reg().with(f);
        self
    }

    #[doc="Get the SCGC Register."]
    #[inline] pub fn scgc_reg(&self) -> ::bobbin_mcu::register::Register<Scgc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Scgc, 0xc)
    }

    #[doc="Get the *mut pointer for the SCGC register."]
    #[inline] pub fn scgc_mut(&self) -> *mut Scgc { 
        self.scgc_reg().ptr()
    }

    #[doc="Get the *const pointer for the SCGC register."]
    #[inline] pub fn scgc_ptr(&self) -> *const Scgc { 
        self.scgc_reg().ptr()
    }

    #[doc="Read the SCGC register."]
    #[inline] pub fn scgc(&self) -> Scgc { 
        self.scgc_reg().read()
    }

    #[doc="Write the SCGC register."]
    #[inline] pub fn write_scgc(&self, value: Scgc) -> &Self { 
        self.scgc_reg().write(value);
        self
    }

    #[doc="Set the SCGC register."]
    #[inline] pub fn set_scgc<F: FnOnce(Scgc) -> Scgc>(&self, f: F) -> &Self {
        self.scgc_reg().set(f);
        self
    }

    #[doc="Modify the SCGC register."]
    #[inline] pub fn with_scgc<F: FnOnce(Scgc) -> Scgc>(&self, f: F) -> &Self {
        self.scgc_reg().with(f);
        self
    }

    #[doc="Get the UUIDL Register."]
    #[inline] pub fn uuidl_reg(&self) -> ::bobbin_mcu::register::Register<Uuidl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Uuidl, 0x10)
    }

    #[doc="Get the *mut pointer for the UUIDL register."]
    #[inline] pub fn uuidl_mut(&self) -> *mut Uuidl { 
        self.uuidl_reg().ptr()
    }

    #[doc="Get the *const pointer for the UUIDL register."]
    #[inline] pub fn uuidl_ptr(&self) -> *const Uuidl { 
        self.uuidl_reg().ptr()
    }

    #[doc="Read the UUIDL register."]
    #[inline] pub fn uuidl(&self) -> Uuidl { 
        self.uuidl_reg().read()
    }

    #[doc="Get the UUIDH Register."]
    #[inline] pub fn uuidh_reg(&self) -> ::bobbin_mcu::register::Register<Uuidh> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Uuidh, 0x14)
    }

    #[doc="Get the *mut pointer for the UUIDH register."]
    #[inline] pub fn uuidh_mut(&self) -> *mut Uuidh { 
        self.uuidh_reg().ptr()
    }

    #[doc="Get the *const pointer for the UUIDH register."]
    #[inline] pub fn uuidh_ptr(&self) -> *const Uuidh { 
        self.uuidh_reg().ptr()
    }

    #[doc="Read the UUIDH register."]
    #[inline] pub fn uuidh(&self) -> Uuidh { 
        self.uuidh_reg().read()
    }

    #[doc="Get the BUSDIV Register."]
    #[inline] pub fn busdiv_reg(&self) -> ::bobbin_mcu::register::Register<Busdiv> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Busdiv, 0x18)
    }

    #[doc="Get the *mut pointer for the BUSDIV register."]
    #[inline] pub fn busdiv_mut(&self) -> *mut Busdiv { 
        self.busdiv_reg().ptr()
    }

    #[doc="Get the *const pointer for the BUSDIV register."]
    #[inline] pub fn busdiv_ptr(&self) -> *const Busdiv { 
        self.busdiv_reg().ptr()
    }

    #[doc="Read the BUSDIV register."]
    #[inline] pub fn busdiv(&self) -> Busdiv { 
        self.busdiv_reg().read()
    }

    #[doc="Write the BUSDIV register."]
    #[inline] pub fn write_busdiv(&self, value: Busdiv) -> &Self { 
        self.busdiv_reg().write(value);
        self
    }

    #[doc="Set the BUSDIV register."]
    #[inline] pub fn set_busdiv<F: FnOnce(Busdiv) -> Busdiv>(&self, f: F) -> &Self {
        self.busdiv_reg().set(f);
        self
    }

    #[doc="Modify the BUSDIV register."]
    #[inline] pub fn with_busdiv<F: FnOnce(Busdiv) -> Busdiv>(&self, f: F) -> &Self {
        self.busdiv_reg().with(f);
        self
    }

}

#[doc="System Reset Status and ID Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Srsid(pub u32);
impl Srsid {
    #[doc="Low Voltage Detect"]
    #[inline] pub fn lvd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if LVD != 0"]
    #[inline] pub fn test_lvd(&self) -> bool {
        self.lvd() != 0
    }

    #[doc="Sets the LVD field."]
    #[inline] pub fn set_lvd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Internal Clock Source Module Reset"]
    #[inline] pub fn loc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if LOC != 0"]
    #[inline] pub fn test_loc(&self) -> bool {
        self.loc() != 0
    }

    #[doc="Sets the LOC field."]
    #[inline] pub fn set_loc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Watchdog (WDOG)"]
    #[inline] pub fn wdog(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if WDOG != 0"]
    #[inline] pub fn test_wdog(&self) -> bool {
        self.wdog() != 0
    }

    #[doc="Sets the WDOG field."]
    #[inline] pub fn set_wdog<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="External Reset Pin"]
    #[inline] pub fn pin(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PIN != 0"]
    #[inline] pub fn test_pin(&self) -> bool {
        self.pin() != 0
    }

    #[doc="Sets the PIN field."]
    #[inline] pub fn set_pin<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Power-On Reset"]
    #[inline] pub fn por(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if POR != 0"]
    #[inline] pub fn test_por(&self) -> bool {
        self.por() != 0
    }

    #[doc="Sets the POR field."]
    #[inline] pub fn set_por<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Core Lockup"]
    #[inline] pub fn lockup(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LOCKUP != 0"]
    #[inline] pub fn test_lockup(&self) -> bool {
        self.lockup() != 0
    }

    #[doc="Sets the LOCKUP field."]
    #[inline] pub fn set_lockup<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Software"]
    #[inline] pub fn sw(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SW != 0"]
    #[inline] pub fn test_sw(&self) -> bool {
        self.sw() != 0
    }

    #[doc="Sets the SW field."]
    #[inline] pub fn set_sw<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="MDM-AP System Reset Request"]
    #[inline] pub fn mdmap(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if MDMAP != 0"]
    #[inline] pub fn test_mdmap(&self) -> bool {
        self.mdmap() != 0
    }

    #[doc="Sets the MDMAP field."]
    #[inline] pub fn set_mdmap<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Stop Mode Acknowledge Error Reset"]
    #[inline] pub fn sackerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SACKERR != 0"]
    #[inline] pub fn test_sackerr(&self) -> bool {
        self.sackerr() != 0
    }

    #[doc="Sets the SACKERR field."]
    #[inline] pub fn set_sackerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Device Pin ID"]
    #[inline] pub fn pinid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if PINID != 0"]
    #[inline] pub fn test_pinid(&self) -> bool {
        self.pinid() != 0
    }

    #[doc="Sets the PINID field."]
    #[inline] pub fn set_pinid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Device Revision Number"]
    #[inline] pub fn revid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0xf) as u8) } // [23:20]
    }

    #[doc="Returns true if REVID != 0"]
    #[inline] pub fn test_revid(&self) -> bool {
        self.revid() != 0
    }

    #[doc="Sets the REVID field."]
    #[inline] pub fn set_revid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Kinetis sub-family ID"]
    #[inline] pub fn subfamid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if SUBFAMID != 0"]
    #[inline] pub fn test_subfamid(&self) -> bool {
        self.subfamid() != 0
    }

    #[doc="Sets the SUBFAMID field."]
    #[inline] pub fn set_subfamid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Kinetis family ID"]
    #[inline] pub fn famid(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0xf) as u8) } // [31:28]
    }

    #[doc="Returns true if FAMID != 0"]
    #[inline] pub fn test_famid(&self) -> bool {
        self.famid() != 0
    }

    #[doc="Sets the FAMID field."]
    #[inline] pub fn set_famid<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 28);
        self.0 |= value << 28;
        self
    }

}

impl From<u32> for Srsid {
    #[inline]
    fn from(other: u32) -> Self {
         Srsid(other)
    }
}

impl ::core::fmt::Display for Srsid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Srsid {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.lvd() != 0 { try!(write!(f, " lvd"))}
        if self.loc() != 0 { try!(write!(f, " loc"))}
        if self.wdog() != 0 { try!(write!(f, " wdog"))}
        if self.pin() != 0 { try!(write!(f, " pin"))}
        if self.por() != 0 { try!(write!(f, " por"))}
        if self.lockup() != 0 { try!(write!(f, " lockup"))}
        if self.sw() != 0 { try!(write!(f, " sw"))}
        if self.mdmap() != 0 { try!(write!(f, " mdmap"))}
        if self.sackerr() != 0 { try!(write!(f, " sackerr"))}
        if self.pinid() != 0 { try!(write!(f, " pinid=0x{:x}", self.pinid()))}
        if self.revid() != 0 { try!(write!(f, " revid=0x{:x}", self.revid()))}
        if self.subfamid() != 0 { try!(write!(f, " subfamid=0x{:x}", self.subfamid()))}
        if self.famid() != 0 { try!(write!(f, " famid=0x{:x}", self.famid()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Options Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sopt(pub u32);
impl Sopt {
    #[doc="NMI Pin Enable"]
    #[inline] pub fn nmie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if NMIE != 0"]
    #[inline] pub fn test_nmie(&self) -> bool {
        self.nmie() != 0
    }

    #[doc="Sets the NMIE field."]
    #[inline] pub fn set_nmie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="RESET Pin Enable"]
    #[inline] pub fn rstpe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RSTPE != 0"]
    #[inline] pub fn test_rstpe(&self) -> bool {
        self.rstpe() != 0
    }

    #[doc="Sets the RSTPE field."]
    #[inline] pub fn set_rstpe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Single Wire Debug Port Pin Enable"]
    #[inline] pub fn swde(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SWDE != 0"]
    #[inline] pub fn test_swde(&self) -> bool {
        self.swde() != 0
    }

    #[doc="Sets the SWDE field."]
    #[inline] pub fn set_swde<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ADC Hardware Trigger Source"]
    #[inline] pub fn adhwt(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if ADHWT != 0"]
    #[inline] pub fn test_adhwt(&self) -> bool {
        self.adhwt() != 0
    }

    #[doc="Sets the ADHWT field."]
    #[inline] pub fn set_adhwt<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Real-Time Counter Capture"]
    #[inline] pub fn rtcc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if RTCC != 0"]
    #[inline] pub fn test_rtcc(&self) -> bool {
        self.rtcc() != 0
    }

    #[doc="Sets the RTCC field."]
    #[inline] pub fn set_rtcc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Analog Comparator to Input Capture Enable"]
    #[inline] pub fn acic(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if ACIC != 0"]
    #[inline] pub fn test_acic(&self) -> bool {
        self.acic() != 0
    }

    #[doc="Sets the ACIC field."]
    #[inline] pub fn set_acic<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="UART0_RX Capture Select"]
    #[inline] pub fn rxdce(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if RXDCE != 0"]
    #[inline] pub fn test_rxdce(&self) -> bool {
        self.rxdce() != 0
    }

    #[doc="Sets the RXDCE field."]
    #[inline] pub fn set_rxdce<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="UART0_RX Filter Select"]
    #[inline] pub fn rxdfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if RXDFE != 0"]
    #[inline] pub fn test_rxdfe(&self) -> bool {
        self.rxdfe() != 0
    }

    #[doc="Sets the RXDFE field."]
    #[inline] pub fn set_rxdfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="FTM2 Synchronization Select"]
    #[inline] pub fn ftmsync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FTMSYNC != 0"]
    #[inline] pub fn test_ftmsync(&self) -> bool {
        self.ftmsync() != 0
    }

    #[doc="Sets the FTMSYNC field."]
    #[inline] pub fn set_ftmsync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="UART0_TX Modulation Select"]
    #[inline] pub fn txdme(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if TXDME != 0"]
    #[inline] pub fn test_txdme(&self) -> bool {
        self.txdme() != 0
    }

    #[doc="Sets the TXDME field."]
    #[inline] pub fn set_txdme<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="BUS Clock Output select"]
    #[inline] pub fn busref(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7) as u8) } // [18:16]
    }

    #[doc="Returns true if BUSREF != 0"]
    #[inline] pub fn test_busref(&self) -> bool {
        self.busref() != 0
    }

    #[doc="Sets the BUSREF field."]
    #[inline] pub fn set_busref<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Bus Clock Output Enable"]
    #[inline] pub fn clkoe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if CLKOE != 0"]
    #[inline] pub fn test_clkoe(&self) -> bool {
        self.clkoe() != 0
    }

    #[doc="Sets the CLKOE field."]
    #[inline] pub fn set_clkoe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="FTM2 Trigger Delay Active"]
    #[inline] pub fn dlyact(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if DLYACT != 0"]
    #[inline] pub fn test_dlyact(&self) -> bool {
        self.dlyact() != 0
    }

    #[doc="Sets the DLYACT field."]
    #[inline] pub fn set_dlyact<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="FTM2 Trigger Delay"]
    #[inline] pub fn delay(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DELAY != 0"]
    #[inline] pub fn test_delay(&self) -> bool {
        self.delay() != 0
    }

    #[doc="Sets the DELAY field."]
    #[inline] pub fn set_delay<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Sopt {
    #[inline]
    fn from(other: u32) -> Self {
         Sopt(other)
    }
}

impl ::core::fmt::Display for Sopt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sopt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nmie() != 0 { try!(write!(f, " nmie"))}
        if self.rstpe() != 0 { try!(write!(f, " rstpe"))}
        if self.swde() != 0 { try!(write!(f, " swde"))}
        if self.adhwt() != 0 { try!(write!(f, " adhwt=0x{:x}", self.adhwt()))}
        if self.rtcc() != 0 { try!(write!(f, " rtcc"))}
        if self.acic() != 0 { try!(write!(f, " acic"))}
        if self.rxdce() != 0 { try!(write!(f, " rxdce"))}
        if self.rxdfe() != 0 { try!(write!(f, " rxdfe"))}
        if self.ftmsync() != 0 { try!(write!(f, " ftmsync"))}
        if self.txdme() != 0 { try!(write!(f, " txdme"))}
        if self.busref() != 0 { try!(write!(f, " busref=0x{:x}", self.busref()))}
        if self.clkoe() != 0 { try!(write!(f, " clkoe"))}
        if self.dlyact() != 0 { try!(write!(f, " dlyact"))}
        if self.delay() != 0 { try!(write!(f, " delay=0x{:x}", self.delay()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pin Selection Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pinsel(pub u32);
impl Pinsel {
    #[doc="RTCO Pin Select"]
    #[inline] pub fn rtcps(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if RTCPS != 0"]
    #[inline] pub fn test_rtcps(&self) -> bool {
        self.rtcps() != 0
    }

    #[doc="Sets the RTCPS field."]
    #[inline] pub fn set_rtcps<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="I2C0 Port Pin Select"]
    #[inline] pub fn i2c0ps(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if I2C0PS != 0"]
    #[inline] pub fn test_i2c0ps(&self) -> bool {
        self.i2c0ps() != 0
    }

    #[doc="Sets the I2C0PS field."]
    #[inline] pub fn set_i2c0ps<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="SPI0 Pin Select"]
    #[inline] pub fn spi0ps(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if SPI0PS != 0"]
    #[inline] pub fn test_spi0ps(&self) -> bool {
        self.spi0ps() != 0
    }

    #[doc="Sets the SPI0PS field."]
    #[inline] pub fn set_spi0ps<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="UART0 Pin Select"]
    #[inline] pub fn uart0ps(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if UART0PS != 0"]
    #[inline] pub fn test_uart0ps(&self) -> bool {
        self.uart0ps() != 0
    }

    #[doc="Sets the UART0PS field."]
    #[inline] pub fn set_uart0ps<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="FTM0_CH0 Port Pin Select"]
    #[inline] pub fn ftm0ps0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if FTM0PS0 != 0"]
    #[inline] pub fn test_ftm0ps0(&self) -> bool {
        self.ftm0ps0() != 0
    }

    #[doc="Sets the FTM0PS0 field."]
    #[inline] pub fn set_ftm0ps0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FTM0_CH1 Port Pin Select"]
    #[inline] pub fn ftm0ps1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if FTM0PS1 != 0"]
    #[inline] pub fn test_ftm0ps1(&self) -> bool {
        self.ftm0ps1() != 0
    }

    #[doc="Sets the FTM0PS1 field."]
    #[inline] pub fn set_ftm0ps1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="FTM1_CH0 Port Pin Select"]
    #[inline] pub fn ftm1ps0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if FTM1PS0 != 0"]
    #[inline] pub fn test_ftm1ps0(&self) -> bool {
        self.ftm1ps0() != 0
    }

    #[doc="Sets the FTM1PS0 field."]
    #[inline] pub fn set_ftm1ps0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="FTM1_CH1 Port Pin Select"]
    #[inline] pub fn ftm1ps1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if FTM1PS1 != 0"]
    #[inline] pub fn test_ftm1ps1(&self) -> bool {
        self.ftm1ps1() != 0
    }

    #[doc="Sets the FTM1PS1 field."]
    #[inline] pub fn set_ftm1ps1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="FTM2_CH0 Port Pin Select"]
    #[inline] pub fn ftm2ps0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FTM2PS0 != 0"]
    #[inline] pub fn test_ftm2ps0(&self) -> bool {
        self.ftm2ps0() != 0
    }

    #[doc="Sets the FTM2PS0 field."]
    #[inline] pub fn set_ftm2ps0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="FTM2_CH1 Port Pin Select"]
    #[inline] pub fn ftm2ps1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if FTM2PS1 != 0"]
    #[inline] pub fn test_ftm2ps1(&self) -> bool {
        self.ftm2ps1() != 0
    }

    #[doc="Sets the FTM2PS1 field."]
    #[inline] pub fn set_ftm2ps1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="FTM2_CH2 Port Pin Select"]
    #[inline] pub fn ftm2ps2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FTM2PS2 != 0"]
    #[inline] pub fn test_ftm2ps2(&self) -> bool {
        self.ftm2ps2() != 0
    }

    #[doc="Sets the FTM2PS2 field."]
    #[inline] pub fn set_ftm2ps2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="FTM2_CH3 Port Pin Select"]
    #[inline] pub fn ftm2ps3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if FTM2PS3 != 0"]
    #[inline] pub fn test_ftm2ps3(&self) -> bool {
        self.ftm2ps3() != 0
    }

    #[doc="Sets the FTM2PS3 field."]
    #[inline] pub fn set_ftm2ps3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Pinsel {
    #[inline]
    fn from(other: u32) -> Self {
         Pinsel(other)
    }
}

impl ::core::fmt::Display for Pinsel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pinsel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rtcps() != 0 { try!(write!(f, " rtcps"))}
        if self.i2c0ps() != 0 { try!(write!(f, " i2c0ps"))}
        if self.spi0ps() != 0 { try!(write!(f, " spi0ps"))}
        if self.uart0ps() != 0 { try!(write!(f, " uart0ps"))}
        if self.ftm0ps0() != 0 { try!(write!(f, " ftm0ps0"))}
        if self.ftm0ps1() != 0 { try!(write!(f, " ftm0ps1"))}
        if self.ftm1ps0() != 0 { try!(write!(f, " ftm1ps0"))}
        if self.ftm1ps1() != 0 { try!(write!(f, " ftm1ps1"))}
        if self.ftm2ps0() != 0 { try!(write!(f, " ftm2ps0"))}
        if self.ftm2ps1() != 0 { try!(write!(f, " ftm2ps1"))}
        if self.ftm2ps2() != 0 { try!(write!(f, " ftm2ps2"))}
        if self.ftm2ps3() != 0 { try!(write!(f, " ftm2ps3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="System Clock Gating Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Scgc(pub u32);
impl Scgc {
    #[doc="RTC Clock Gate Control"]
    #[inline] pub fn rtc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RTC != 0"]
    #[inline] pub fn test_rtc(&self) -> bool {
        self.rtc() != 0
    }

    #[doc="Sets the RTC field."]
    #[inline] pub fn set_rtc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="PIT Clock Gate Control"]
    #[inline] pub fn pit(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PIT != 0"]
    #[inline] pub fn test_pit(&self) -> bool {
        self.pit() != 0
    }

    #[doc="Sets the PIT field."]
    #[inline] pub fn set_pit<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FTM0 Clock Gate Control"]
    #[inline] pub fn ftm0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FTM0 != 0"]
    #[inline] pub fn test_ftm0(&self) -> bool {
        self.ftm0() != 0
    }

    #[doc="Sets the FTM0 field."]
    #[inline] pub fn set_ftm0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="FTM1 Clock Gate Control"]
    #[inline] pub fn ftm1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FTM1 != 0"]
    #[inline] pub fn test_ftm1(&self) -> bool {
        self.ftm1() != 0
    }

    #[doc="Sets the FTM1 field."]
    #[inline] pub fn set_ftm1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="FTM2 Clock Gate Control"]
    #[inline] pub fn ftm2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FTM2 != 0"]
    #[inline] pub fn test_ftm2(&self) -> bool {
        self.ftm2() != 0
    }

    #[doc="Sets the FTM2 field."]
    #[inline] pub fn set_ftm2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="CRC Clock Gate Control"]
    #[inline] pub fn crc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CRC != 0"]
    #[inline] pub fn test_crc(&self) -> bool {
        self.crc() != 0
    }

    #[doc="Sets the CRC field."]
    #[inline] pub fn set_crc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Flash Clock Gate Control"]
    #[inline] pub fn flash(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if FLASH != 0"]
    #[inline] pub fn test_flash(&self) -> bool {
        self.flash() != 0
    }

    #[doc="Sets the FLASH field."]
    #[inline] pub fn set_flash<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="SWD (single wire debugger) Clock Gate Control"]
    #[inline] pub fn swd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SWD != 0"]
    #[inline] pub fn test_swd(&self) -> bool {
        self.swd() != 0
    }

    #[doc="Sets the SWD field."]
    #[inline] pub fn set_swd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="I2C Clock Gate Control"]
    #[inline] pub fn i2c(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if I2C != 0"]
    #[inline] pub fn test_i2c(&self) -> bool {
        self.i2c() != 0
    }

    #[doc="Sets the I2C field."]
    #[inline] pub fn set_i2c<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="SPI0 Clock Gate Control"]
    #[inline] pub fn spi0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if SPI0 != 0"]
    #[inline] pub fn test_spi0(&self) -> bool {
        self.spi0() != 0
    }

    #[doc="Sets the SPI0 field."]
    #[inline] pub fn set_spi0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="SPI1 Clock Gate Control"]
    #[inline] pub fn spi1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if SPI1 != 0"]
    #[inline] pub fn test_spi1(&self) -> bool {
        self.spi1() != 0
    }

    #[doc="Sets the SPI1 field."]
    #[inline] pub fn set_spi1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="UART0 Clock Gate Control"]
    #[inline] pub fn uart0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if UART0 != 0"]
    #[inline] pub fn test_uart0(&self) -> bool {
        self.uart0() != 0
    }

    #[doc="Sets the UART0 field."]
    #[inline] pub fn set_uart0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="UART1 Clock Gate Control"]
    #[inline] pub fn uart1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if UART1 != 0"]
    #[inline] pub fn test_uart1(&self) -> bool {
        self.uart1() != 0
    }

    #[doc="Sets the UART1 field."]
    #[inline] pub fn set_uart1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="UART2 Clock Gate Control"]
    #[inline] pub fn uart2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if UART2 != 0"]
    #[inline] pub fn test_uart2(&self) -> bool {
        self.uart2() != 0
    }

    #[doc="Sets the UART2 field."]
    #[inline] pub fn set_uart2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="KBI0 Clock Gate Control"]
    #[inline] pub fn kbi0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if KBI0 != 0"]
    #[inline] pub fn test_kbi0(&self) -> bool {
        self.kbi0() != 0
    }

    #[doc="Sets the KBI0 field."]
    #[inline] pub fn set_kbi0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="KBI1 Clock Gate Control"]
    #[inline] pub fn kbi1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if KBI1 != 0"]
    #[inline] pub fn test_kbi1(&self) -> bool {
        self.kbi1() != 0
    }

    #[doc="Sets the KBI1 field."]
    #[inline] pub fn set_kbi1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="IRQ Clock Gate Control"]
    #[inline] pub fn irq(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if IRQ != 0"]
    #[inline] pub fn test_irq(&self) -> bool {
        self.irq() != 0
    }

    #[doc="Sets the IRQ field."]
    #[inline] pub fn set_irq<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="ADC Clock Gate Control"]
    #[inline] pub fn adc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if ADC != 0"]
    #[inline] pub fn test_adc(&self) -> bool {
        self.adc() != 0
    }

    #[doc="Sets the ADC field."]
    #[inline] pub fn set_adc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="ACMP0 Clock Gate Control"]
    #[inline] pub fn acmp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if ACMP0 != 0"]
    #[inline] pub fn test_acmp0(&self) -> bool {
        self.acmp0() != 0
    }

    #[doc="Sets the ACMP0 field."]
    #[inline] pub fn set_acmp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="ACMP1 Clock Gate Control"]
    #[inline] pub fn acmp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if ACMP1 != 0"]
    #[inline] pub fn test_acmp1(&self) -> bool {
        self.acmp1() != 0
    }

    #[doc="Sets the ACMP1 field."]
    #[inline] pub fn set_acmp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Scgc {
    #[inline]
    fn from(other: u32) -> Self {
         Scgc(other)
    }
}

impl ::core::fmt::Display for Scgc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Scgc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rtc() != 0 { try!(write!(f, " rtc"))}
        if self.pit() != 0 { try!(write!(f, " pit"))}
        if self.ftm0() != 0 { try!(write!(f, " ftm0"))}
        if self.ftm1() != 0 { try!(write!(f, " ftm1"))}
        if self.ftm2() != 0 { try!(write!(f, " ftm2"))}
        if self.crc() != 0 { try!(write!(f, " crc"))}
        if self.flash() != 0 { try!(write!(f, " flash"))}
        if self.swd() != 0 { try!(write!(f, " swd"))}
        if self.i2c() != 0 { try!(write!(f, " i2c"))}
        if self.spi0() != 0 { try!(write!(f, " spi0"))}
        if self.spi1() != 0 { try!(write!(f, " spi1"))}
        if self.uart0() != 0 { try!(write!(f, " uart0"))}
        if self.uart1() != 0 { try!(write!(f, " uart1"))}
        if self.uart2() != 0 { try!(write!(f, " uart2"))}
        if self.kbi0() != 0 { try!(write!(f, " kbi0"))}
        if self.kbi1() != 0 { try!(write!(f, " kbi1"))}
        if self.irq() != 0 { try!(write!(f, " irq"))}
        if self.adc() != 0 { try!(write!(f, " adc"))}
        if self.acmp0() != 0 { try!(write!(f, " acmp0"))}
        if self.acmp1() != 0 { try!(write!(f, " acmp1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universally Unique Identifier Low Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uuidl(pub u32);
impl Uuidl {
    #[doc="Universally Unique Identifier"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uuidl {
    #[inline]
    fn from(other: u32) -> Self {
         Uuidl(other)
    }
}

impl ::core::fmt::Display for Uuidl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Uuidl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Universally Unique Identifier High Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Uuidh(pub u32);
impl Uuidh {
    #[doc="Universally Unique Identifier"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Uuidh {
    #[inline]
    fn from(other: u32) -> Self {
         Uuidh(other)
    }
}

impl ::core::fmt::Display for Uuidh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Uuidh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="BUS Clock Divider Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Busdiv(pub u32);
impl Busdiv {
    #[doc="BUS Clock Divider"]
    #[inline] pub fn busdiv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if BUSDIV != 0"]
    #[inline] pub fn test_busdiv(&self) -> bool {
        self.busdiv() != 0
    }

    #[doc="Sets the BUSDIV field."]
    #[inline] pub fn set_busdiv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Busdiv {
    #[inline]
    fn from(other: u32) -> Self {
         Busdiv(other)
    }
}

impl ::core::fmt::Display for Busdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Busdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.busdiv() != 0 { try!(write!(f, " busdiv"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

