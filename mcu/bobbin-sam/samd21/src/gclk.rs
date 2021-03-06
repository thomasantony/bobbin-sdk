
::bobbin_mcu::periph!( GCLK, Gclk, GCLK_PERIPH, GclkPeriph, GCLK_OWNED, GCLK_REF_COUNT, 0x40000c00, 0x00, 0x00);


// Gate { name: None, gate_type: Some("EN"), periph: Some("PM"), register: Some("APBAMASK"), field: Some("GCLK"), description: None }
impl ::bobbin_mcu::gate::GateEn for Gclk {
    #[inline]
    fn gate_en(&self) -> ::bobbin_bits::U1 { ::pm::PM.apbamask().gclk() }
    #[inline]
    fn set_gate_en<V: Into<::bobbin_bits::U1>>(&self, value: V) -> &Self {
        ::pm::PM.with_apbamask(|r| r.set_gclk(value));
        self
    }
}

#[doc="Generic Clock Generator"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GclkPeriph(pub usize);
impl GclkPeriph {
    #[doc="Get the CLKCTRL Register."]
    #[inline] pub fn clkctrl_reg(&self) -> ::bobbin_mcu::register::Register<Clkctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Clkctrl, 0x2)
    }

    #[doc="Get the *mut pointer for the CLKCTRL register."]
    #[inline] pub fn clkctrl_mut(&self) -> *mut Clkctrl { 
        self.clkctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLKCTRL register."]
    #[inline] pub fn clkctrl_ptr(&self) -> *const Clkctrl { 
        self.clkctrl_reg().ptr()
    }

    #[doc="Read the CLKCTRL register."]
    #[inline] pub fn clkctrl(&self) -> Clkctrl { 
        self.clkctrl_reg().read()
    }

    #[doc="Write the CLKCTRL register."]
    #[inline] pub fn write_clkctrl(&self, value: Clkctrl) -> &Self { 
        self.clkctrl_reg().write(value);
        self
    }

    #[doc="Set the CLKCTRL register."]
    #[inline] pub fn set_clkctrl<F: FnOnce(Clkctrl) -> Clkctrl>(&self, f: F) -> &Self {
        self.clkctrl_reg().set(f);
        self
    }

    #[doc="Modify the CLKCTRL register."]
    #[inline] pub fn with_clkctrl<F: FnOnce(Clkctrl) -> Clkctrl>(&self, f: F) -> &Self {
        self.clkctrl_reg().with(f);
        self
    }

    #[doc="Get the CLKCTRL_ID Register."]
    #[inline] pub fn clkctrl_id_reg(&self) -> ::bobbin_mcu::register::Register<ClkctrlId> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut ClkctrlId, 0x2)
    }

    #[doc="Get the *mut pointer for the CLKCTRL_ID register."]
    #[inline] pub fn clkctrl_id_mut(&self) -> *mut ClkctrlId { 
        self.clkctrl_id_reg().ptr()
    }

    #[doc="Get the *const pointer for the CLKCTRL_ID register."]
    #[inline] pub fn clkctrl_id_ptr(&self) -> *const ClkctrlId { 
        self.clkctrl_id_reg().ptr()
    }

    #[doc="Read the CLKCTRL_ID register."]
    #[inline] pub fn clkctrl_id(&self) -> ClkctrlId { 
        self.clkctrl_id_reg().read()
    }

    #[doc="Write the CLKCTRL_ID register."]
    #[inline] pub fn write_clkctrl_id(&self, value: ClkctrlId) -> &Self { 
        self.clkctrl_id_reg().write(value);
        self
    }

    #[doc="Set the CLKCTRL_ID register."]
    #[inline] pub fn set_clkctrl_id<F: FnOnce(ClkctrlId) -> ClkctrlId>(&self, f: F) -> &Self {
        self.clkctrl_id_reg().set(f);
        self
    }

    #[doc="Modify the CLKCTRL_ID register."]
    #[inline] pub fn with_clkctrl_id<F: FnOnce(ClkctrlId) -> ClkctrlId>(&self, f: F) -> &Self {
        self.clkctrl_id_reg().with(f);
        self
    }

    #[doc="Get the CTRL Register."]
    #[inline] pub fn ctrl_reg(&self) -> ::bobbin_mcu::register::Register<Ctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrl, 0x0)
    }

    #[doc="Get the *mut pointer for the CTRL register."]
    #[inline] pub fn ctrl_mut(&self) -> *mut Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRL register."]
    #[inline] pub fn ctrl_ptr(&self) -> *const Ctrl { 
        self.ctrl_reg().ptr()
    }

    #[doc="Read the CTRL register."]
    #[inline] pub fn ctrl(&self) -> Ctrl { 
        self.ctrl_reg().read()
    }

    #[doc="Write the CTRL register."]
    #[inline] pub fn write_ctrl(&self, value: Ctrl) -> &Self { 
        self.ctrl_reg().write(value);
        self
    }

    #[doc="Set the CTRL register."]
    #[inline] pub fn set_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().set(f);
        self
    }

    #[doc="Modify the CTRL register."]
    #[inline] pub fn with_ctrl<F: FnOnce(Ctrl) -> Ctrl>(&self, f: F) -> &Self {
        self.ctrl_reg().with(f);
        self
    }

    #[doc="Get the GENCTRL Register."]
    #[inline] pub fn genctrl_reg(&self) -> ::bobbin_mcu::register::Register<Genctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Genctrl, 0x4)
    }

    #[doc="Get the *mut pointer for the GENCTRL register."]
    #[inline] pub fn genctrl_mut(&self) -> *mut Genctrl { 
        self.genctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the GENCTRL register."]
    #[inline] pub fn genctrl_ptr(&self) -> *const Genctrl { 
        self.genctrl_reg().ptr()
    }

    #[doc="Read the GENCTRL register."]
    #[inline] pub fn genctrl(&self) -> Genctrl { 
        self.genctrl_reg().read()
    }

    #[doc="Write the GENCTRL register."]
    #[inline] pub fn write_genctrl(&self, value: Genctrl) -> &Self { 
        self.genctrl_reg().write(value);
        self
    }

    #[doc="Set the GENCTRL register."]
    #[inline] pub fn set_genctrl<F: FnOnce(Genctrl) -> Genctrl>(&self, f: F) -> &Self {
        self.genctrl_reg().set(f);
        self
    }

    #[doc="Modify the GENCTRL register."]
    #[inline] pub fn with_genctrl<F: FnOnce(Genctrl) -> Genctrl>(&self, f: F) -> &Self {
        self.genctrl_reg().with(f);
        self
    }

    #[doc="Get the GENCTRL_ID Register."]
    #[inline] pub fn genctrl_id_reg(&self) -> ::bobbin_mcu::register::Register<GenctrlId> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut GenctrlId, 0x4)
    }

    #[doc="Get the *mut pointer for the GENCTRL_ID register."]
    #[inline] pub fn genctrl_id_mut(&self) -> *mut GenctrlId { 
        self.genctrl_id_reg().ptr()
    }

    #[doc="Get the *const pointer for the GENCTRL_ID register."]
    #[inline] pub fn genctrl_id_ptr(&self) -> *const GenctrlId { 
        self.genctrl_id_reg().ptr()
    }

    #[doc="Read the GENCTRL_ID register."]
    #[inline] pub fn genctrl_id(&self) -> GenctrlId { 
        self.genctrl_id_reg().read()
    }

    #[doc="Write the GENCTRL_ID register."]
    #[inline] pub fn write_genctrl_id(&self, value: GenctrlId) -> &Self { 
        self.genctrl_id_reg().write(value);
        self
    }

    #[doc="Set the GENCTRL_ID register."]
    #[inline] pub fn set_genctrl_id<F: FnOnce(GenctrlId) -> GenctrlId>(&self, f: F) -> &Self {
        self.genctrl_id_reg().set(f);
        self
    }

    #[doc="Modify the GENCTRL_ID register."]
    #[inline] pub fn with_genctrl_id<F: FnOnce(GenctrlId) -> GenctrlId>(&self, f: F) -> &Self {
        self.genctrl_id_reg().with(f);
        self
    }

    #[doc="Get the GENDIV Register."]
    #[inline] pub fn gendiv_reg(&self) -> ::bobbin_mcu::register::Register<Gendiv> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gendiv, 0x8)
    }

    #[doc="Get the *mut pointer for the GENDIV register."]
    #[inline] pub fn gendiv_mut(&self) -> *mut Gendiv { 
        self.gendiv_reg().ptr()
    }

    #[doc="Get the *const pointer for the GENDIV register."]
    #[inline] pub fn gendiv_ptr(&self) -> *const Gendiv { 
        self.gendiv_reg().ptr()
    }

    #[doc="Read the GENDIV register."]
    #[inline] pub fn gendiv(&self) -> Gendiv { 
        self.gendiv_reg().read()
    }

    #[doc="Write the GENDIV register."]
    #[inline] pub fn write_gendiv(&self, value: Gendiv) -> &Self { 
        self.gendiv_reg().write(value);
        self
    }

    #[doc="Set the GENDIV register."]
    #[inline] pub fn set_gendiv<F: FnOnce(Gendiv) -> Gendiv>(&self, f: F) -> &Self {
        self.gendiv_reg().set(f);
        self
    }

    #[doc="Modify the GENDIV register."]
    #[inline] pub fn with_gendiv<F: FnOnce(Gendiv) -> Gendiv>(&self, f: F) -> &Self {
        self.gendiv_reg().with(f);
        self
    }

    #[doc="Get the GENDIV_ID Register."]
    #[inline] pub fn gendiv_id_reg(&self) -> ::bobbin_mcu::register::Register<GendivId> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut GendivId, 0x8)
    }

    #[doc="Get the *mut pointer for the GENDIV_ID register."]
    #[inline] pub fn gendiv_id_mut(&self) -> *mut GendivId { 
        self.gendiv_id_reg().ptr()
    }

    #[doc="Get the *const pointer for the GENDIV_ID register."]
    #[inline] pub fn gendiv_id_ptr(&self) -> *const GendivId { 
        self.gendiv_id_reg().ptr()
    }

    #[doc="Read the GENDIV_ID register."]
    #[inline] pub fn gendiv_id(&self) -> GendivId { 
        self.gendiv_id_reg().read()
    }

    #[doc="Write the GENDIV_ID register."]
    #[inline] pub fn write_gendiv_id(&self, value: GendivId) -> &Self { 
        self.gendiv_id_reg().write(value);
        self
    }

    #[doc="Set the GENDIV_ID register."]
    #[inline] pub fn set_gendiv_id<F: FnOnce(GendivId) -> GendivId>(&self, f: F) -> &Self {
        self.gendiv_id_reg().set(f);
        self
    }

    #[doc="Modify the GENDIV_ID register."]
    #[inline] pub fn with_gendiv_id<F: FnOnce(GendivId) -> GendivId>(&self, f: F) -> &Self {
        self.gendiv_id_reg().with(f);
        self
    }

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x1)
    }

    #[doc="Get the *mut pointer for the STATUS register."]
    #[inline] pub fn status_mut(&self) -> *mut Status { 
        self.status_reg().ptr()
    }

    #[doc="Get the *const pointer for the STATUS register."]
    #[inline] pub fn status_ptr(&self) -> *const Status { 
        self.status_reg().ptr()
    }

    #[doc="Read the STATUS register."]
    #[inline] pub fn status(&self) -> Status { 
        self.status_reg().read()
    }

}

#[doc="Generic Clock Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Clkctrl(pub u16);
impl Clkctrl {
    #[doc="Generic Clock Selection ID"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Generic Clock Generator"]
    #[inline] pub fn gen(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if GEN != 0"]
    #[inline] pub fn test_gen(&self) -> bool {
        self.gen() != 0
    }

    #[doc="Sets the GEN field."]
    #[inline] pub fn set_gen<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Clock Enable"]
    #[inline] pub fn clken(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CLKEN != 0"]
    #[inline] pub fn test_clken(&self) -> bool {
        self.clken() != 0
    }

    #[doc="Sets the CLKEN field."]
    #[inline] pub fn set_clken<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Write Lock"]
    #[inline] pub fn wrtlock(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if WRTLOCK != 0"]
    #[inline] pub fn test_wrtlock(&self) -> bool {
        self.wrtlock() != 0
    }

    #[doc="Sets the WRTLOCK field."]
    #[inline] pub fn set_wrtlock<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u16> for Clkctrl {
    #[inline]
    fn from(other: u16) -> Self {
         Clkctrl(other)
    }
}

impl ::core::fmt::Display for Clkctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Clkctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
        if self.gen() != 0 { try!(write!(f, " gen=0x{:x}", self.gen()))}
        if self.clken() != 0 { try!(write!(f, " clken"))}
        if self.wrtlock() != 0 { try!(write!(f, " wrtlock"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Generic Clock Control - ID Field"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct ClkctrlId(pub u8);
impl ClkctrlId {
    #[doc="Generic Clock Selection ID"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for ClkctrlId {
    #[inline]
    fn from(other: u8) -> Self {
         ClkctrlId(other)
    }
}

impl ::core::fmt::Display for ClkctrlId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for ClkctrlId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u8);
impl Ctrl {
    #[doc="Software Reset"]
    #[inline] pub fn swrst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWRST != 0"]
    #[inline] pub fn test_swrst(&self) -> bool {
        self.swrst() != 0
    }

    #[doc="Sets the SWRST field."]
    #[inline] pub fn set_swrst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Ctrl {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrl(other)
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
        if self.swrst() != 0 { try!(write!(f, " swrst"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Generic Clock Generator Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Genctrl(pub u32);
impl Genctrl {
    #[doc="Generic Clock Generator Selection"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Source Select"]
    #[inline] pub fn src(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1f) as u8) } // [12:8]
    }

    #[doc="Returns true if SRC != 0"]
    #[inline] pub fn test_src(&self) -> bool {
        self.src() != 0
    }

    #[doc="Sets the SRC field."]
    #[inline] pub fn set_src<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Generic Clock Generator Enable"]
    #[inline] pub fn genen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if GENEN != 0"]
    #[inline] pub fn test_genen(&self) -> bool {
        self.genen() != 0
    }

    #[doc="Sets the GENEN field."]
    #[inline] pub fn set_genen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Improve Duty Cycle"]
    #[inline] pub fn idc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if IDC != 0"]
    #[inline] pub fn test_idc(&self) -> bool {
        self.idc() != 0
    }

    #[doc="Sets the IDC field."]
    #[inline] pub fn set_idc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Output Off Value"]
    #[inline] pub fn oov(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if OOV != 0"]
    #[inline] pub fn test_oov(&self) -> bool {
        self.oov() != 0
    }

    #[doc="Sets the OOV field."]
    #[inline] pub fn set_oov<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Output Enable"]
    #[inline] pub fn oe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if OE != 0"]
    #[inline] pub fn test_oe(&self) -> bool {
        self.oe() != 0
    }

    #[doc="Sets the OE field."]
    #[inline] pub fn set_oe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Divide Selection"]
    #[inline] pub fn divsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if DIVSEL != 0"]
    #[inline] pub fn test_divsel(&self) -> bool {
        self.divsel() != 0
    }

    #[doc="Sets the DIVSEL field."]
    #[inline] pub fn set_divsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Run in Standby"]
    #[inline] pub fn runstdby(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if RUNSTDBY != 0"]
    #[inline] pub fn test_runstdby(&self) -> bool {
        self.runstdby() != 0
    }

    #[doc="Sets the RUNSTDBY field."]
    #[inline] pub fn set_runstdby<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

}

impl From<u32> for Genctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Genctrl(other)
    }
}

impl ::core::fmt::Display for Genctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Genctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
        if self.src() != 0 { try!(write!(f, " src=0x{:x}", self.src()))}
        if self.genen() != 0 { try!(write!(f, " genen"))}
        if self.idc() != 0 { try!(write!(f, " idc"))}
        if self.oov() != 0 { try!(write!(f, " oov"))}
        if self.oe() != 0 { try!(write!(f, " oe"))}
        if self.divsel() != 0 { try!(write!(f, " divsel"))}
        if self.runstdby() != 0 { try!(write!(f, " runstdby"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Generic Clock Generator Control - ID Only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct GenctrlId(pub u8);
impl GenctrlId {
    #[doc="Generic Clock Generator Selection"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for GenctrlId {
    #[inline]
    fn from(other: u8) -> Self {
         GenctrlId(other)
    }
}

impl ::core::fmt::Display for GenctrlId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for GenctrlId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Generic Clock Generator Division"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gendiv(pub u32);
impl Gendiv {
    #[doc="Generic Clock Generator Selection"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Division Factor"]
    #[inline] pub fn div(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xffff) as u16) } // [23:8]
    }

    #[doc="Returns true if DIV != 0"]
    #[inline] pub fn test_div(&self) -> bool {
        self.div() != 0
    }

    #[doc="Sets the DIV field."]
    #[inline] pub fn set_div<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Gendiv {
    #[inline]
    fn from(other: u32) -> Self {
         Gendiv(other)
    }
}

impl ::core::fmt::Display for Gendiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gendiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
        if self.div() != 0 { try!(write!(f, " div=0x{:x}", self.div()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Generic Clock Generator Division - ID Only"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct GendivId(pub u8);
impl GendivId {
    #[doc="Generic Clock Generator Selection"]
    #[inline] pub fn id(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if ID != 0"]
    #[inline] pub fn test_id(&self) -> bool {
        self.id() != 0
    }

    #[doc="Sets the ID field."]
    #[inline] pub fn set_id<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for GendivId {
    #[inline]
    fn from(other: u8) -> Self {
         GendivId(other)
    }
}

impl ::core::fmt::Display for GendivId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for GendivId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.id() != 0 { try!(write!(f, " id=0x{:x}", self.id()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u8);
impl Status {
    #[doc="Synchronization Busy Status"]
    #[inline] pub fn syncbusy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SYNCBUSY != 0"]
    #[inline] pub fn test_syncbusy(&self) -> bool {
        self.syncbusy() != 0
    }

    #[doc="Sets the SYNCBUSY field."]
    #[inline] pub fn set_syncbusy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Status {
    #[inline]
    fn from(other: u8) -> Self {
         Status(other)
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

