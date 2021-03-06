
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="TPM Peripheral"]
pub struct TpmPeriph(pub usize); 

pub struct TpmCh { pub periph: TpmPeriph, pub index: usize }

impl TpmPeriph {
    #[doc="Get the SC Register."]
    #[inline] pub fn sc_reg(&self) -> ::bobbin_mcu::register::Register<Sc> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sc, 0x0)
    }

    #[doc="Get the *mut pointer for the SC register."]
    #[inline] pub fn sc_mut(&self) -> *mut Sc { 
        self.sc_reg().ptr()
    }

    #[doc="Get the *const pointer for the SC register."]
    #[inline] pub fn sc_ptr(&self) -> *const Sc { 
        self.sc_reg().ptr()
    }

    #[doc="Read the SC register."]
    #[inline] pub fn sc(&self) -> Sc { 
        self.sc_reg().read()
    }

    #[doc="Write the SC register."]
    #[inline] pub fn write_sc(&self, value: Sc) -> &Self { 
        self.sc_reg().write(value);
        self
    }

    #[doc="Set the SC register."]
    #[inline] pub fn set_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
        self.sc_reg().set(f);
        self
    }

    #[doc="Modify the SC register."]
    #[inline] pub fn with_sc<F: FnOnce(Sc) -> Sc>(&self, f: F) -> &Self {
        self.sc_reg().with(f);
        self
    }

    #[doc="Get the CNT Register."]
    #[inline] pub fn cnt_reg(&self) -> ::bobbin_mcu::register::Register<Cnt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cnt, 0x4)
    }

    #[doc="Get the *mut pointer for the CNT register."]
    #[inline] pub fn cnt_mut(&self) -> *mut Cnt { 
        self.cnt_reg().ptr()
    }

    #[doc="Get the *const pointer for the CNT register."]
    #[inline] pub fn cnt_ptr(&self) -> *const Cnt { 
        self.cnt_reg().ptr()
    }

    #[doc="Read the CNT register."]
    #[inline] pub fn cnt(&self) -> Cnt { 
        self.cnt_reg().read()
    }

    #[doc="Write the CNT register."]
    #[inline] pub fn write_cnt(&self, value: Cnt) -> &Self { 
        self.cnt_reg().write(value);
        self
    }

    #[doc="Set the CNT register."]
    #[inline] pub fn set_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
        self.cnt_reg().set(f);
        self
    }

    #[doc="Modify the CNT register."]
    #[inline] pub fn with_cnt<F: FnOnce(Cnt) -> Cnt>(&self, f: F) -> &Self {
        self.cnt_reg().with(f);
        self
    }

    #[doc="Get the MOD Register."]
    #[inline] pub fn mod_reg(&self) -> ::bobbin_mcu::register::Register<Mod> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mod, 0x8)
    }

    #[doc="Get the *mut pointer for the MOD register."]
    #[inline] pub fn mod_mut(&self) -> *mut Mod { 
        self.mod_reg().ptr()
    }

    #[doc="Get the *const pointer for the MOD register."]
    #[inline] pub fn mod_ptr(&self) -> *const Mod { 
        self.mod_reg().ptr()
    }

    #[doc="Read the MOD register."]
    #[inline] pub fn _mod(&self) -> Mod { 
        self.mod_reg().read()
    }

    #[doc="Write the MOD register."]
    #[inline] pub fn write_mod(&self, value: Mod) -> &Self { 
        self.mod_reg().write(value);
        self
    }

    #[doc="Set the MOD register."]
    #[inline] pub fn set_mod<F: FnOnce(Mod) -> Mod>(&self, f: F) -> &Self {
        self.mod_reg().set(f);
        self
    }

    #[doc="Modify the MOD register."]
    #[inline] pub fn with_mod<F: FnOnce(Mod) -> Mod>(&self, f: F) -> &Self {
        self.mod_reg().with(f);
        self
    }

    #[doc="Get the CSC Register."]
    #[inline] pub fn csc_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Csc, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Csc, 0xc, 0x8)
    }

    #[doc="Get the *mut pointer for the CSC register."]
    #[inline] pub fn csc_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut Csc { 
        self.csc_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CSC register."]
    #[inline] pub fn csc_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const Csc { 
        self.csc_reg().ptr(index.into())
    }

    #[doc="Read the CSC register."]
    #[inline] pub fn csc<I: Into<::bobbin_bits::R6>>(&self, index: I) -> Csc { 
        self.csc_reg().read(index.into())
    }

    #[doc="Write the CSC register."]
    #[inline] pub fn write_csc<I: Into<::bobbin_bits::R6>>(&self, index: I, value: Csc) -> &Self {
        self.csc_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CSC register."]
    #[inline] pub fn set_csc<I: Into<::bobbin_bits::R6>, F: FnOnce(Csc) -> Csc>(&self, index: I, f: F) -> &Self {
        self.csc_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CSC register."]
    #[inline] pub fn with_csc<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(Csc) -> Csc>(&self, index: I, f: F) -> &Self {
        self.csc_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CV Register."]
    #[inline] pub fn cv_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Cv, ::bobbin_bits::R6> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Cv, 0x10, 0x8)
    }

    #[doc="Get the *mut pointer for the CV register."]
    #[inline] pub fn cv_mut<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *mut Cv { 
        self.cv_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CV register."]
    #[inline] pub fn cv_ptr<I: Into<::bobbin_bits::R6>>(&self, index: I) -> *const Cv { 
        self.cv_reg().ptr(index.into())
    }

    #[doc="Read the CV register."]
    #[inline] pub fn cv<I: Into<::bobbin_bits::R6>>(&self, index: I) -> Cv { 
        self.cv_reg().read(index.into())
    }

    #[doc="Write the CV register."]
    #[inline] pub fn write_cv<I: Into<::bobbin_bits::R6>>(&self, index: I, value: Cv) -> &Self {
        self.cv_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CV register."]
    #[inline] pub fn set_cv<I: Into<::bobbin_bits::R6>, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
        self.cv_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CV register."]
    #[inline] pub fn with_cv<I: Into<::bobbin_bits::R6> + Copy, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
        self.cv_reg().with(index.into(), f);
        self
    }

    #[doc="Get the STATUS Register."]
    #[inline] pub fn status_reg(&self) -> ::bobbin_mcu::register::Register<Status> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Status, 0x50)
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

    #[doc="Write the STATUS register."]
    #[inline] pub fn write_status(&self, value: Status) -> &Self { 
        self.status_reg().write(value);
        self
    }

    #[doc="Set the STATUS register."]
    #[inline] pub fn set_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        self.status_reg().set(f);
        self
    }

    #[doc="Modify the STATUS register."]
    #[inline] pub fn with_status<F: FnOnce(Status) -> Status>(&self, f: F) -> &Self {
        self.status_reg().with(f);
        self
    }

    #[doc="Get the CONF Register."]
    #[inline] pub fn conf_reg(&self) -> ::bobbin_mcu::register::Register<Conf> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Conf, 0x84)
    }

    #[doc="Get the *mut pointer for the CONF register."]
    #[inline] pub fn conf_mut(&self) -> *mut Conf { 
        self.conf_reg().ptr()
    }

    #[doc="Get the *const pointer for the CONF register."]
    #[inline] pub fn conf_ptr(&self) -> *const Conf { 
        self.conf_reg().ptr()
    }

    #[doc="Read the CONF register."]
    #[inline] pub fn conf(&self) -> Conf { 
        self.conf_reg().read()
    }

    #[doc="Write the CONF register."]
    #[inline] pub fn write_conf(&self, value: Conf) -> &Self { 
        self.conf_reg().write(value);
        self
    }

    #[doc="Set the CONF register."]
    #[inline] pub fn set_conf<F: FnOnce(Conf) -> Conf>(&self, f: F) -> &Self {
        self.conf_reg().set(f);
        self
    }

    #[doc="Modify the CONF register."]
    #[inline] pub fn with_conf<F: FnOnce(Conf) -> Conf>(&self, f: F) -> &Self {
        self.conf_reg().with(f);
        self
    }

}

#[doc="Status and Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc(pub u32);
impl Sc {
    #[doc="Prescale Factor Selection"]
    #[inline] pub fn ps(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if PS != 0"]
    #[inline] pub fn test_ps(&self) -> bool {
        self.ps() != 0
    }

    #[doc="Sets the PS field."]
    #[inline] pub fn set_ps<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Mode Selection"]
    #[inline] pub fn cmod(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if CMOD != 0"]
    #[inline] pub fn test_cmod(&self) -> bool {
        self.cmod() != 0
    }

    #[doc="Sets the CMOD field."]
    #[inline] pub fn set_cmod<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Center-Aligned PWM Select"]
    #[inline] pub fn cpwms(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CPWMS != 0"]
    #[inline] pub fn test_cpwms(&self) -> bool {
        self.cpwms() != 0
    }

    #[doc="Sets the CPWMS field."]
    #[inline] pub fn set_cpwms<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Timer Overflow Interrupt Enable"]
    #[inline] pub fn toie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TOIE != 0"]
    #[inline] pub fn test_toie(&self) -> bool {
        self.toie() != 0
    }

    #[doc="Sets the TOIE field."]
    #[inline] pub fn set_toie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Timer Overflow Flag"]
    #[inline] pub fn tof(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TOF != 0"]
    #[inline] pub fn test_tof(&self) -> bool {
        self.tof() != 0
    }

    #[doc="Sets the TOF field."]
    #[inline] pub fn set_tof<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="DMA Enable"]
    #[inline] pub fn dma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if DMA != 0"]
    #[inline] pub fn test_dma(&self) -> bool {
        self.dma() != 0
    }

    #[doc="Sets the DMA field."]
    #[inline] pub fn set_dma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Sc {
    #[inline]
    fn from(other: u32) -> Self {
         Sc(other)
    }
}

impl ::core::fmt::Display for Sc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ps() != 0 { try!(write!(f, " ps=0x{:x}", self.ps()))}
        if self.cmod() != 0 { try!(write!(f, " cmod=0x{:x}", self.cmod()))}
        if self.cpwms() != 0 { try!(write!(f, " cpwms"))}
        if self.toie() != 0 { try!(write!(f, " toie"))}
        if self.tof() != 0 { try!(write!(f, " tof"))}
        if self.dma() != 0 { try!(write!(f, " dma"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc="Counter value"]
    #[inline] pub fn count(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if COUNT != 0"]
    #[inline] pub fn test_count(&self) -> bool {
        self.count() != 0
    }

    #[doc="Sets the COUNT field."]
    #[inline] pub fn set_count<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cnt {
    #[inline]
    fn from(other: u32) -> Self {
         Cnt(other)
    }
}

impl ::core::fmt::Display for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.count() != 0 { try!(write!(f, " count=0x{:x}", self.count()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Modulo"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mod(pub u32);
impl Mod {
    #[doc="Modulo value"]
    #[inline] pub fn _mod(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if MOD != 0"]
    #[inline] pub fn test_mod(&self) -> bool {
        self._mod() != 0
    }

    #[doc="Sets the MOD field."]
    #[inline] pub fn set_mod<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mod {
    #[inline]
    fn from(other: u32) -> Self {
         Mod(other)
    }
}

impl ::core::fmt::Display for Mod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self._mod() != 0 { try!(write!(f, " mod=0x{:x}", self._mod()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel (n) Status and Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csc(pub u32);
impl Csc {
    #[doc="DMA Enable"]
    #[inline] pub fn dma(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if DMA != 0"]
    #[inline] pub fn test_dma(&self) -> bool {
        self.dma() != 0
    }

    #[doc="Sets the DMA field."]
    #[inline] pub fn set_dma<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Edge or Level Select"]
    #[inline] pub fn elsa(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ELSA != 0"]
    #[inline] pub fn test_elsa(&self) -> bool {
        self.elsa() != 0
    }

    #[doc="Sets the ELSA field."]
    #[inline] pub fn set_elsa<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Edge or Level Select"]
    #[inline] pub fn elsb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if ELSB != 0"]
    #[inline] pub fn test_elsb(&self) -> bool {
        self.elsb() != 0
    }

    #[doc="Sets the ELSB field."]
    #[inline] pub fn set_elsb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel Mode Select"]
    #[inline] pub fn msa(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if MSA != 0"]
    #[inline] pub fn test_msa(&self) -> bool {
        self.msa() != 0
    }

    #[doc="Sets the MSA field."]
    #[inline] pub fn set_msa<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel Mode Select"]
    #[inline] pub fn msb(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if MSB != 0"]
    #[inline] pub fn test_msb(&self) -> bool {
        self.msb() != 0
    }

    #[doc="Sets the MSB field."]
    #[inline] pub fn set_msb<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel Interrupt Enable"]
    #[inline] pub fn chie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CHIE != 0"]
    #[inline] pub fn test_chie(&self) -> bool {
        self.chie() != 0
    }

    #[doc="Sets the CHIE field."]
    #[inline] pub fn set_chie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel Flag"]
    #[inline] pub fn chf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CHF != 0"]
    #[inline] pub fn test_chf(&self) -> bool {
        self.chf() != 0
    }

    #[doc="Sets the CHF field."]
    #[inline] pub fn set_chf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Csc {
    #[inline]
    fn from(other: u32) -> Self {
         Csc(other)
    }
}

impl ::core::fmt::Display for Csc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Csc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dma() != 0 { try!(write!(f, " dma"))}
        if self.elsa() != 0 { try!(write!(f, " elsa"))}
        if self.elsb() != 0 { try!(write!(f, " elsb"))}
        if self.msa() != 0 { try!(write!(f, " msa"))}
        if self.msb() != 0 { try!(write!(f, " msb"))}
        if self.chie() != 0 { try!(write!(f, " chie"))}
        if self.chf() != 0 { try!(write!(f, " chf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channel (n) Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cv(pub u32);
impl Cv {
    #[doc="Channel Value"]
    #[inline] pub fn val(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if VAL != 0"]
    #[inline] pub fn test_val(&self) -> bool {
        self.val() != 0
    }

    #[doc="Sets the VAL field."]
    #[inline] pub fn set_val<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cv {
    #[inline]
    fn from(other: u32) -> Self {
         Cv(other)
    }
}

impl ::core::fmt::Display for Cv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.val() != 0 { try!(write!(f, " val=0x{:x}", self.val()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capture and Compare Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="Channel n Flag"]
    #[inline] pub fn chf<I: Into<::bobbin_bits::R6>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CHF != 0"]
    #[inline] pub fn test_chf<I: Into<::bobbin_bits::R6>>(&self, index: I) -> bool{
        self.chf(index) != 0
    }

    #[doc="Sets the CHF field."]
    #[inline] pub fn set_chf<I: Into<::bobbin_bits::R6>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Status {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.chf(0) != 0 { try!(write!(f, " chf[0]"))}
        if self.chf(1) != 0 { try!(write!(f, " chf[1]"))}
        if self.chf(2) != 0 { try!(write!(f, " chf[2]"))}
        if self.chf(3) != 0 { try!(write!(f, " chf[3]"))}
        if self.chf(4) != 0 { try!(write!(f, " chf[4]"))}
        if self.chf(5) != 0 { try!(write!(f, " chf[5]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Conf(pub u32);
impl Conf {
    #[doc="Doze Enable"]
    #[inline] pub fn dozeen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if DOZEEN != 0"]
    #[inline] pub fn test_dozeen(&self) -> bool {
        self.dozeen() != 0
    }

    #[doc="Sets the DOZEEN field."]
    #[inline] pub fn set_dozeen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Debug Mode"]
    #[inline] pub fn dbgmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if DBGMODE != 0"]
    #[inline] pub fn test_dbgmode(&self) -> bool {
        self.dbgmode() != 0
    }

    #[doc="Sets the DBGMODE field."]
    #[inline] pub fn set_dbgmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Global time base enable"]
    #[inline] pub fn gtbeen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if GTBEEN != 0"]
    #[inline] pub fn test_gtbeen(&self) -> bool {
        self.gtbeen() != 0
    }

    #[doc="Sets the GTBEEN field."]
    #[inline] pub fn set_gtbeen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Counter Start on Trigger"]
    #[inline] pub fn csot(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if CSOT != 0"]
    #[inline] pub fn test_csot(&self) -> bool {
        self.csot() != 0
    }

    #[doc="Sets the CSOT field."]
    #[inline] pub fn set_csot<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Counter Stop On Overflow"]
    #[inline] pub fn csoo(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if CSOO != 0"]
    #[inline] pub fn test_csoo(&self) -> bool {
        self.csoo() != 0
    }

    #[doc="Sets the CSOO field."]
    #[inline] pub fn set_csoo<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Counter Reload On Trigger"]
    #[inline] pub fn crot(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if CROT != 0"]
    #[inline] pub fn test_crot(&self) -> bool {
        self.crot() != 0
    }

    #[doc="Sets the CROT field."]
    #[inline] pub fn set_crot<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Trigger Select"]
    #[inline] pub fn trgsel(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xf) as u8) } // [27:24]
    }

    #[doc="Returns true if TRGSEL != 0"]
    #[inline] pub fn test_trgsel(&self) -> bool {
        self.trgsel() != 0
    }

    #[doc="Sets the TRGSEL field."]
    #[inline] pub fn set_trgsel<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Conf {
    #[inline]
    fn from(other: u32) -> Self {
         Conf(other)
    }
}

impl ::core::fmt::Display for Conf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Conf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dozeen() != 0 { try!(write!(f, " dozeen"))}
        if self.dbgmode() != 0 { try!(write!(f, " dbgmode=0x{:x}", self.dbgmode()))}
        if self.gtbeen() != 0 { try!(write!(f, " gtbeen"))}
        if self.csot() != 0 { try!(write!(f, " csot"))}
        if self.csoo() != 0 { try!(write!(f, " csoo"))}
        if self.crot() != 0 { try!(write!(f, " crot"))}
        if self.trgsel() != 0 { try!(write!(f, " trgsel=0x{:x}", self.trgsel()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

