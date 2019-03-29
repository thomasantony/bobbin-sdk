::bobbin_mcu::periph!( FTMRH, Ftmrh, FTMRH_PERIPH, FtmrhPeriph, FTMRH_OWNED, FTMRH_REF_COUNT, 0x40020000, 0x00, 0x00);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="FTMRH Peripheral"]
pub struct FtmrhPeriph(pub usize); 

impl FtmrhPeriph {
    #[doc="Get the FCLKDIV Register."]
    #[inline] pub fn fclkdiv_reg(&self) -> ::bobbin_mcu::register::Register<Fclkdiv> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fclkdiv, 0x0)
    }

    #[doc="Get the *mut pointer for the FCLKDIV register."]
    #[inline] pub fn fclkdiv_mut(&self) -> *mut Fclkdiv { 
        self.fclkdiv_reg().ptr()
    }

    #[doc="Get the *const pointer for the FCLKDIV register."]
    #[inline] pub fn fclkdiv_ptr(&self) -> *const Fclkdiv { 
        self.fclkdiv_reg().ptr()
    }

    #[doc="Read the FCLKDIV register."]
    #[inline] pub fn fclkdiv(&self) -> Fclkdiv { 
        self.fclkdiv_reg().read()
    }

    #[doc="Write the FCLKDIV register."]
    #[inline] pub fn write_fclkdiv(&self, value: Fclkdiv) -> &Self { 
        self.fclkdiv_reg().write(value);
        self
    }

    #[doc="Set the FCLKDIV register."]
    #[inline] pub fn set_fclkdiv<F: FnOnce(Fclkdiv) -> Fclkdiv>(&self, f: F) -> &Self {
        self.fclkdiv_reg().set(f);
        self
    }

    #[doc="Modify the FCLKDIV register."]
    #[inline] pub fn with_fclkdiv<F: FnOnce(Fclkdiv) -> Fclkdiv>(&self, f: F) -> &Self {
        self.fclkdiv_reg().with(f);
        self
    }

    #[doc="Get the FSEC Register."]
    #[inline] pub fn fsec_reg(&self) -> ::bobbin_mcu::register::Register<Fsec> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fsec, 0x1)
    }

    #[doc="Get the *mut pointer for the FSEC register."]
    #[inline] pub fn fsec_mut(&self) -> *mut Fsec { 
        self.fsec_reg().ptr()
    }

    #[doc="Get the *const pointer for the FSEC register."]
    #[inline] pub fn fsec_ptr(&self) -> *const Fsec { 
        self.fsec_reg().ptr()
    }

    #[doc="Read the FSEC register."]
    #[inline] pub fn fsec(&self) -> Fsec { 
        self.fsec_reg().read()
    }

    #[doc="Get the FCCOBIX Register."]
    #[inline] pub fn fccobix_reg(&self) -> ::bobbin_mcu::register::Register<Fccobix> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fccobix, 0x2)
    }

    #[doc="Get the *mut pointer for the FCCOBIX register."]
    #[inline] pub fn fccobix_mut(&self) -> *mut Fccobix { 
        self.fccobix_reg().ptr()
    }

    #[doc="Get the *const pointer for the FCCOBIX register."]
    #[inline] pub fn fccobix_ptr(&self) -> *const Fccobix { 
        self.fccobix_reg().ptr()
    }

    #[doc="Read the FCCOBIX register."]
    #[inline] pub fn fccobix(&self) -> Fccobix { 
        self.fccobix_reg().read()
    }

    #[doc="Write the FCCOBIX register."]
    #[inline] pub fn write_fccobix(&self, value: Fccobix) -> &Self { 
        self.fccobix_reg().write(value);
        self
    }

    #[doc="Set the FCCOBIX register."]
    #[inline] pub fn set_fccobix<F: FnOnce(Fccobix) -> Fccobix>(&self, f: F) -> &Self {
        self.fccobix_reg().set(f);
        self
    }

    #[doc="Modify the FCCOBIX register."]
    #[inline] pub fn with_fccobix<F: FnOnce(Fccobix) -> Fccobix>(&self, f: F) -> &Self {
        self.fccobix_reg().with(f);
        self
    }

    #[doc="Get the FCNFG Register."]
    #[inline] pub fn fcnfg_reg(&self) -> ::bobbin_mcu::register::Register<Fcnfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fcnfg, 0x4)
    }

    #[doc="Get the *mut pointer for the FCNFG register."]
    #[inline] pub fn fcnfg_mut(&self) -> *mut Fcnfg { 
        self.fcnfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the FCNFG register."]
    #[inline] pub fn fcnfg_ptr(&self) -> *const Fcnfg { 
        self.fcnfg_reg().ptr()
    }

    #[doc="Read the FCNFG register."]
    #[inline] pub fn fcnfg(&self) -> Fcnfg { 
        self.fcnfg_reg().read()
    }

    #[doc="Write the FCNFG register."]
    #[inline] pub fn write_fcnfg(&self, value: Fcnfg) -> &Self { 
        self.fcnfg_reg().write(value);
        self
    }

    #[doc="Set the FCNFG register."]
    #[inline] pub fn set_fcnfg<F: FnOnce(Fcnfg) -> Fcnfg>(&self, f: F) -> &Self {
        self.fcnfg_reg().set(f);
        self
    }

    #[doc="Modify the FCNFG register."]
    #[inline] pub fn with_fcnfg<F: FnOnce(Fcnfg) -> Fcnfg>(&self, f: F) -> &Self {
        self.fcnfg_reg().with(f);
        self
    }

    #[doc="Get the FERCNFG Register."]
    #[inline] pub fn fercnfg_reg(&self) -> ::bobbin_mcu::register::Register<Fercnfg> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fercnfg, 0x5)
    }

    #[doc="Get the *mut pointer for the FERCNFG register."]
    #[inline] pub fn fercnfg_mut(&self) -> *mut Fercnfg { 
        self.fercnfg_reg().ptr()
    }

    #[doc="Get the *const pointer for the FERCNFG register."]
    #[inline] pub fn fercnfg_ptr(&self) -> *const Fercnfg { 
        self.fercnfg_reg().ptr()
    }

    #[doc="Read the FERCNFG register."]
    #[inline] pub fn fercnfg(&self) -> Fercnfg { 
        self.fercnfg_reg().read()
    }

    #[doc="Write the FERCNFG register."]
    #[inline] pub fn write_fercnfg(&self, value: Fercnfg) -> &Self { 
        self.fercnfg_reg().write(value);
        self
    }

    #[doc="Set the FERCNFG register."]
    #[inline] pub fn set_fercnfg<F: FnOnce(Fercnfg) -> Fercnfg>(&self, f: F) -> &Self {
        self.fercnfg_reg().set(f);
        self
    }

    #[doc="Modify the FERCNFG register."]
    #[inline] pub fn with_fercnfg<F: FnOnce(Fercnfg) -> Fercnfg>(&self, f: F) -> &Self {
        self.fercnfg_reg().with(f);
        self
    }

    #[doc="Get the FSTAT Register."]
    #[inline] pub fn fstat_reg(&self) -> ::bobbin_mcu::register::Register<Fstat> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fstat, 0x6)
    }

    #[doc="Get the *mut pointer for the FSTAT register."]
    #[inline] pub fn fstat_mut(&self) -> *mut Fstat { 
        self.fstat_reg().ptr()
    }

    #[doc="Get the *const pointer for the FSTAT register."]
    #[inline] pub fn fstat_ptr(&self) -> *const Fstat { 
        self.fstat_reg().ptr()
    }

    #[doc="Read the FSTAT register."]
    #[inline] pub fn fstat(&self) -> Fstat { 
        self.fstat_reg().read()
    }

    #[doc="Write the FSTAT register."]
    #[inline] pub fn write_fstat(&self, value: Fstat) -> &Self { 
        self.fstat_reg().write(value);
        self
    }

    #[doc="Set the FSTAT register."]
    #[inline] pub fn set_fstat<F: FnOnce(Fstat) -> Fstat>(&self, f: F) -> &Self {
        self.fstat_reg().set(f);
        self
    }

    #[doc="Modify the FSTAT register."]
    #[inline] pub fn with_fstat<F: FnOnce(Fstat) -> Fstat>(&self, f: F) -> &Self {
        self.fstat_reg().with(f);
        self
    }

    #[doc="Get the FERSTAT Register."]
    #[inline] pub fn ferstat_reg(&self) -> ::bobbin_mcu::register::Register<Ferstat> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ferstat, 0x7)
    }

    #[doc="Get the *mut pointer for the FERSTAT register."]
    #[inline] pub fn ferstat_mut(&self) -> *mut Ferstat { 
        self.ferstat_reg().ptr()
    }

    #[doc="Get the *const pointer for the FERSTAT register."]
    #[inline] pub fn ferstat_ptr(&self) -> *const Ferstat { 
        self.ferstat_reg().ptr()
    }

    #[doc="Read the FERSTAT register."]
    #[inline] pub fn ferstat(&self) -> Ferstat { 
        self.ferstat_reg().read()
    }

    #[doc="Write the FERSTAT register."]
    #[inline] pub fn write_ferstat(&self, value: Ferstat) -> &Self { 
        self.ferstat_reg().write(value);
        self
    }

    #[doc="Set the FERSTAT register."]
    #[inline] pub fn set_ferstat<F: FnOnce(Ferstat) -> Ferstat>(&self, f: F) -> &Self {
        self.ferstat_reg().set(f);
        self
    }

    #[doc="Modify the FERSTAT register."]
    #[inline] pub fn with_ferstat<F: FnOnce(Ferstat) -> Ferstat>(&self, f: F) -> &Self {
        self.ferstat_reg().with(f);
        self
    }

    #[doc="Get the FPROT Register."]
    #[inline] pub fn fprot_reg(&self) -> ::bobbin_mcu::register::Register<Fprot> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fprot, 0x8)
    }

    #[doc="Get the *mut pointer for the FPROT register."]
    #[inline] pub fn fprot_mut(&self) -> *mut Fprot { 
        self.fprot_reg().ptr()
    }

    #[doc="Get the *const pointer for the FPROT register."]
    #[inline] pub fn fprot_ptr(&self) -> *const Fprot { 
        self.fprot_reg().ptr()
    }

    #[doc="Read the FPROT register."]
    #[inline] pub fn fprot(&self) -> Fprot { 
        self.fprot_reg().read()
    }

    #[doc="Write the FPROT register."]
    #[inline] pub fn write_fprot(&self, value: Fprot) -> &Self { 
        self.fprot_reg().write(value);
        self
    }

    #[doc="Set the FPROT register."]
    #[inline] pub fn set_fprot<F: FnOnce(Fprot) -> Fprot>(&self, f: F) -> &Self {
        self.fprot_reg().set(f);
        self
    }

    #[doc="Modify the FPROT register."]
    #[inline] pub fn with_fprot<F: FnOnce(Fprot) -> Fprot>(&self, f: F) -> &Self {
        self.fprot_reg().with(f);
        self
    }

    #[doc="Get the EEPROT Register."]
    #[inline] pub fn eeprot_reg(&self) -> ::bobbin_mcu::register::Register<Eeprot> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Eeprot, 0x9)
    }

    #[doc="Get the *mut pointer for the EEPROT register."]
    #[inline] pub fn eeprot_mut(&self) -> *mut Eeprot { 
        self.eeprot_reg().ptr()
    }

    #[doc="Get the *const pointer for the EEPROT register."]
    #[inline] pub fn eeprot_ptr(&self) -> *const Eeprot { 
        self.eeprot_reg().ptr()
    }

    #[doc="Read the EEPROT register."]
    #[inline] pub fn eeprot(&self) -> Eeprot { 
        self.eeprot_reg().read()
    }

    #[doc="Write the EEPROT register."]
    #[inline] pub fn write_eeprot(&self, value: Eeprot) -> &Self { 
        self.eeprot_reg().write(value);
        self
    }

    #[doc="Set the EEPROT register."]
    #[inline] pub fn set_eeprot<F: FnOnce(Eeprot) -> Eeprot>(&self, f: F) -> &Self {
        self.eeprot_reg().set(f);
        self
    }

    #[doc="Modify the EEPROT register."]
    #[inline] pub fn with_eeprot<F: FnOnce(Eeprot) -> Eeprot>(&self, f: F) -> &Self {
        self.eeprot_reg().with(f);
        self
    }

    #[doc="Get the FCCOBHI Register."]
    #[inline] pub fn fccobhi_reg(&self) -> ::bobbin_mcu::register::Register<Fccobhi> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fccobhi, 0xa)
    }

    #[doc="Get the *mut pointer for the FCCOBHI register."]
    #[inline] pub fn fccobhi_mut(&self) -> *mut Fccobhi { 
        self.fccobhi_reg().ptr()
    }

    #[doc="Get the *const pointer for the FCCOBHI register."]
    #[inline] pub fn fccobhi_ptr(&self) -> *const Fccobhi { 
        self.fccobhi_reg().ptr()
    }

    #[doc="Read the FCCOBHI register."]
    #[inline] pub fn fccobhi(&self) -> Fccobhi { 
        self.fccobhi_reg().read()
    }

    #[doc="Write the FCCOBHI register."]
    #[inline] pub fn write_fccobhi(&self, value: Fccobhi) -> &Self { 
        self.fccobhi_reg().write(value);
        self
    }

    #[doc="Set the FCCOBHI register."]
    #[inline] pub fn set_fccobhi<F: FnOnce(Fccobhi) -> Fccobhi>(&self, f: F) -> &Self {
        self.fccobhi_reg().set(f);
        self
    }

    #[doc="Modify the FCCOBHI register."]
    #[inline] pub fn with_fccobhi<F: FnOnce(Fccobhi) -> Fccobhi>(&self, f: F) -> &Self {
        self.fccobhi_reg().with(f);
        self
    }

    #[doc="Get the FCCOBLO Register."]
    #[inline] pub fn fccoblo_reg(&self) -> ::bobbin_mcu::register::Register<Fccoblo> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fccoblo, 0xb)
    }

    #[doc="Get the *mut pointer for the FCCOBLO register."]
    #[inline] pub fn fccoblo_mut(&self) -> *mut Fccoblo { 
        self.fccoblo_reg().ptr()
    }

    #[doc="Get the *const pointer for the FCCOBLO register."]
    #[inline] pub fn fccoblo_ptr(&self) -> *const Fccoblo { 
        self.fccoblo_reg().ptr()
    }

    #[doc="Read the FCCOBLO register."]
    #[inline] pub fn fccoblo(&self) -> Fccoblo { 
        self.fccoblo_reg().read()
    }

    #[doc="Write the FCCOBLO register."]
    #[inline] pub fn write_fccoblo(&self, value: Fccoblo) -> &Self { 
        self.fccoblo_reg().write(value);
        self
    }

    #[doc="Set the FCCOBLO register."]
    #[inline] pub fn set_fccoblo<F: FnOnce(Fccoblo) -> Fccoblo>(&self, f: F) -> &Self {
        self.fccoblo_reg().set(f);
        self
    }

    #[doc="Modify the FCCOBLO register."]
    #[inline] pub fn with_fccoblo<F: FnOnce(Fccoblo) -> Fccoblo>(&self, f: F) -> &Self {
        self.fccoblo_reg().with(f);
        self
    }

    #[doc="Get the FOPT Register."]
    #[inline] pub fn fopt_reg(&self) -> ::bobbin_mcu::register::Register<Fopt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fopt, 0xc)
    }

    #[doc="Get the *mut pointer for the FOPT register."]
    #[inline] pub fn fopt_mut(&self) -> *mut Fopt { 
        self.fopt_reg().ptr()
    }

    #[doc="Get the *const pointer for the FOPT register."]
    #[inline] pub fn fopt_ptr(&self) -> *const Fopt { 
        self.fopt_reg().ptr()
    }

    #[doc="Read the FOPT register."]
    #[inline] pub fn fopt(&self) -> Fopt { 
        self.fopt_reg().read()
    }

}

#[doc="Flash Clock Divider Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fclkdiv(pub u8);
impl Fclkdiv {
    #[doc="Clock Divider Bits"]
    #[inline] pub fn fdiv(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if FDIV != 0"]
    #[inline] pub fn test_fdiv(&self) -> bool {
        self.fdiv() != 0
    }

    #[doc="Sets the FDIV field."]
    #[inline] pub fn set_fdiv<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Clock Divider Locked"]
    #[inline] pub fn fdivlck(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FDIVLCK != 0"]
    #[inline] pub fn test_fdivlck(&self) -> bool {
        self.fdivlck() != 0
    }

    #[doc="Sets the FDIVLCK field."]
    #[inline] pub fn set_fdivlck<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Clock Divider Loaded"]
    #[inline] pub fn fdivld(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FDIVLD != 0"]
    #[inline] pub fn test_fdivld(&self) -> bool {
        self.fdivld() != 0
    }

    #[doc="Sets the FDIVLD field."]
    #[inline] pub fn set_fdivld<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Fclkdiv {
    #[inline]
    fn from(other: u8) -> Self {
         Fclkdiv(other)
    }
}

impl ::core::fmt::Display for Fclkdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fclkdiv {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fdiv() != 0 { try!(write!(f, " fdiv=0x{:x}", self.fdiv()))}
        if self.fdivlck() != 0 { try!(write!(f, " fdivlck"))}
        if self.fdivld() != 0 { try!(write!(f, " fdivld"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Security Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fsec(pub u8);
impl Fsec {
    #[doc="Flash Security Bits"]
    #[inline] pub fn sec(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if SEC != 0"]
    #[inline] pub fn test_sec(&self) -> bool {
        self.sec() != 0
    }

    #[doc="Sets the SEC field."]
    #[inline] pub fn set_sec<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Backdoor Key Security Enable Bits"]
    #[inline] pub fn keyen(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if KEYEN != 0"]
    #[inline] pub fn test_keyen(&self) -> bool {
        self.keyen() != 0
    }

    #[doc="Sets the KEYEN field."]
    #[inline] pub fn set_keyen<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Fsec {
    #[inline]
    fn from(other: u8) -> Self {
         Fsec(other)
    }
}

impl ::core::fmt::Display for Fsec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fsec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sec() != 0 { try!(write!(f, " sec=0x{:x}", self.sec()))}
        if self.keyen() != 0 { try!(write!(f, " keyen=0x{:x}", self.keyen()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash CCOB Index Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fccobix(pub u8);
impl Fccobix {
    #[doc="Common Command Register Index"]
    #[inline] pub fn ccobix(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if CCOBIX != 0"]
    #[inline] pub fn test_ccobix(&self) -> bool {
        self.ccobix() != 0
    }

    #[doc="Sets the CCOBIX field."]
    #[inline] pub fn set_ccobix<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fccobix {
    #[inline]
    fn from(other: u8) -> Self {
         Fccobix(other)
    }
}

impl ::core::fmt::Display for Fccobix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fccobix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccobix() != 0 { try!(write!(f, " ccobix=0x{:x}", self.ccobix()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fcnfg(pub u8);
impl Fcnfg {
    #[doc="Force Single Bit Fault Detect"]
    #[inline] pub fn fsfd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FSFD != 0"]
    #[inline] pub fn test_fsfd(&self) -> bool {
        self.fsfd() != 0
    }

    #[doc="Sets the FSFD field."]
    #[inline] pub fn set_fsfd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Force Double Bit Fault Detect"]
    #[inline] pub fn fdfd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FDFD != 0"]
    #[inline] pub fn test_fdfd(&self) -> bool {
        self.fdfd() != 0
    }

    #[doc="Sets the FDFD field."]
    #[inline] pub fn set_fdfd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Ignore Single Bit Fault"]
    #[inline] pub fn ignsf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IGNSF != 0"]
    #[inline] pub fn test_ignsf(&self) -> bool {
        self.ignsf() != 0
    }

    #[doc="Sets the IGNSF field."]
    #[inline] pub fn set_ignsf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Command Complete Interrupt Enable"]
    #[inline] pub fn ccie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CCIE != 0"]
    #[inline] pub fn test_ccie(&self) -> bool {
        self.ccie() != 0
    }

    #[doc="Sets the CCIE field."]
    #[inline] pub fn set_ccie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Fcnfg {
    #[inline]
    fn from(other: u8) -> Self {
         Fcnfg(other)
    }
}

impl ::core::fmt::Display for Fcnfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fcnfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fsfd() != 0 { try!(write!(f, " fsfd"))}
        if self.fdfd() != 0 { try!(write!(f, " fdfd"))}
        if self.ignsf() != 0 { try!(write!(f, " ignsf"))}
        if self.ccie() != 0 { try!(write!(f, " ccie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Error Configuration Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fercnfg(pub u8);
impl Fercnfg {
    #[doc="Single Bit Fault Detect Interrupt Enable"]
    #[inline] pub fn sfdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SFDIE != 0"]
    #[inline] pub fn test_sfdie(&self) -> bool {
        self.sfdie() != 0
    }

    #[doc="Sets the SFDIE field."]
    #[inline] pub fn set_sfdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Double Bit Fault Detect Interrupt Enable"]
    #[inline] pub fn dfdie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DFDIE != 0"]
    #[inline] pub fn test_dfdie(&self) -> bool {
        self.dfdie() != 0
    }

    #[doc="Sets the DFDIE field."]
    #[inline] pub fn set_dfdie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Fercnfg {
    #[inline]
    fn from(other: u8) -> Self {
         Fercnfg(other)
    }
}

impl ::core::fmt::Display for Fercnfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fercnfg {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sfdie() != 0 { try!(write!(f, " sfdie"))}
        if self.dfdie() != 0 { try!(write!(f, " dfdie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fstat(pub u8);
impl Fstat {
    #[doc="Memory Controller Command Completion Status Flag"]
    #[inline] pub fn mgstat(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if MGSTAT != 0"]
    #[inline] pub fn test_mgstat(&self) -> bool {
        self.mgstat() != 0
    }

    #[doc="Sets the MGSTAT field."]
    #[inline] pub fn set_mgstat<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Memory Controller Busy Flag"]
    #[inline] pub fn mgbusy(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if MGBUSY != 0"]
    #[inline] pub fn test_mgbusy(&self) -> bool {
        self.mgbusy() != 0
    }

    #[doc="Sets the MGBUSY field."]
    #[inline] pub fn set_mgbusy<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Flash Protection Violation Flag"]
    #[inline] pub fn fpviol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FPVIOL != 0"]
    #[inline] pub fn test_fpviol(&self) -> bool {
        self.fpviol() != 0
    }

    #[doc="Sets the FPVIOL field."]
    #[inline] pub fn set_fpviol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Flash Access Error Flag"]
    #[inline] pub fn accerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACCERR != 0"]
    #[inline] pub fn test_accerr(&self) -> bool {
        self.accerr() != 0
    }

    #[doc="Sets the ACCERR field."]
    #[inline] pub fn set_accerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Command Complete Interrupt Flag"]
    #[inline] pub fn ccif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CCIF != 0"]
    #[inline] pub fn test_ccif(&self) -> bool {
        self.ccif() != 0
    }

    #[doc="Sets the CCIF field."]
    #[inline] pub fn set_ccif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Fstat {
    #[inline]
    fn from(other: u8) -> Self {
         Fstat(other)
    }
}

impl ::core::fmt::Display for Fstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.mgstat() != 0 { try!(write!(f, " mgstat=0x{:x}", self.mgstat()))}
        if self.mgbusy() != 0 { try!(write!(f, " mgbusy"))}
        if self.fpviol() != 0 { try!(write!(f, " fpviol"))}
        if self.accerr() != 0 { try!(write!(f, " accerr"))}
        if self.ccif() != 0 { try!(write!(f, " ccif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Error Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ferstat(pub u8);
impl Ferstat {
    #[doc="Single Bit Fault Detect Interrupt Flag"]
    #[inline] pub fn sfdif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SFDIF != 0"]
    #[inline] pub fn test_sfdif(&self) -> bool {
        self.sfdif() != 0
    }

    #[doc="Sets the SFDIF field."]
    #[inline] pub fn set_sfdif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Double Bit Fault Detect Interrupt Flag"]
    #[inline] pub fn dfdif(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if DFDIF != 0"]
    #[inline] pub fn test_dfdif(&self) -> bool {
        self.dfdif() != 0
    }

    #[doc="Sets the DFDIF field."]
    #[inline] pub fn set_dfdif<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u8> for Ferstat {
    #[inline]
    fn from(other: u8) -> Self {
         Ferstat(other)
    }
}

impl ::core::fmt::Display for Ferstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ferstat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sfdif() != 0 { try!(write!(f, " sfdif"))}
        if self.dfdif() != 0 { try!(write!(f, " dfdif"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Protection Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fprot(pub u8);
impl Fprot {
    #[doc="Flash Protection Lower Address Size"]
    #[inline] pub fn fpls(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if FPLS != 0"]
    #[inline] pub fn test_fpls(&self) -> bool {
        self.fpls() != 0
    }

    #[doc="Sets the FPLS field."]
    #[inline] pub fn set_fpls<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Flash Protection Lower Address Range Disable"]
    #[inline] pub fn fpldis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FPLDIS != 0"]
    #[inline] pub fn test_fpldis(&self) -> bool {
        self.fpldis() != 0
    }

    #[doc="Sets the FPLDIS field."]
    #[inline] pub fn set_fpldis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Flash Protection Higher Address Size"]
    #[inline] pub fn fphs(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if FPHS != 0"]
    #[inline] pub fn test_fphs(&self) -> bool {
        self.fphs() != 0
    }

    #[doc="Sets the FPHS field."]
    #[inline] pub fn set_fphs<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Flash Protection Higher Address Range Disable"]
    #[inline] pub fn fphdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FPHDIS != 0"]
    #[inline] pub fn test_fphdis(&self) -> bool {
        self.fphdis() != 0
    }

    #[doc="Sets the FPHDIS field."]
    #[inline] pub fn set_fphdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Reserved Nonvolatile Bit"]
    #[inline] pub fn rnv6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RNV6 != 0"]
    #[inline] pub fn test_rnv6(&self) -> bool {
        self.rnv6() != 0
    }

    #[doc="Sets the RNV6 field."]
    #[inline] pub fn set_rnv6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Flash Protection Operation Enable"]
    #[inline] pub fn fpopen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FPOPEN != 0"]
    #[inline] pub fn test_fpopen(&self) -> bool {
        self.fpopen() != 0
    }

    #[doc="Sets the FPOPEN field."]
    #[inline] pub fn set_fpopen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Fprot {
    #[inline]
    fn from(other: u8) -> Self {
         Fprot(other)
    }
}

impl ::core::fmt::Display for Fprot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fprot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fpls() != 0 { try!(write!(f, " fpls=0x{:x}", self.fpls()))}
        if self.fpldis() != 0 { try!(write!(f, " fpldis"))}
        if self.fphs() != 0 { try!(write!(f, " fphs=0x{:x}", self.fphs()))}
        if self.fphdis() != 0 { try!(write!(f, " fphdis"))}
        if self.rnv6() != 0 { try!(write!(f, " rnv6"))}
        if self.fpopen() != 0 { try!(write!(f, " fpopen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="EEPROM Protection Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Eeprot(pub u8);
impl Eeprot {
    #[doc="EEPROM Protection Size"]
    #[inline] pub fn dps(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if DPS != 0"]
    #[inline] pub fn test_dps(&self) -> bool {
        self.dps() != 0
    }

    #[doc="Sets the DPS field."]
    #[inline] pub fn set_dps<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="EEPROM Protection Control"]
    #[inline] pub fn dpopen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if DPOPEN != 0"]
    #[inline] pub fn test_dpopen(&self) -> bool {
        self.dpopen() != 0
    }

    #[doc="Sets the DPOPEN field."]
    #[inline] pub fn set_dpopen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u8> for Eeprot {
    #[inline]
    fn from(other: u8) -> Self {
         Eeprot(other)
    }
}

impl ::core::fmt::Display for Eeprot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Eeprot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dps() != 0 { try!(write!(f, " dps=0x{:x}", self.dps()))}
        if self.dpopen() != 0 { try!(write!(f, " dpopen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Common Command Object Register:High"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fccobhi(pub u8);
impl Fccobhi {
    #[doc="Common Command Object Bit 15:8"]
    #[inline] pub fn ccob(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CCOB != 0"]
    #[inline] pub fn test_ccob(&self) -> bool {
        self.ccob() != 0
    }

    #[doc="Sets the CCOB field."]
    #[inline] pub fn set_ccob<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fccobhi {
    #[inline]
    fn from(other: u8) -> Self {
         Fccobhi(other)
    }
}

impl ::core::fmt::Display for Fccobhi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fccobhi {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccob() != 0 { try!(write!(f, " ccob=0x{:x}", self.ccob()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Common Command Object Register: Low"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fccoblo(pub u8);
impl Fccoblo {
    #[doc="Common Command Object Bit 7:0"]
    #[inline] pub fn ccob(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if CCOB != 0"]
    #[inline] pub fn test_ccob(&self) -> bool {
        self.ccob() != 0
    }

    #[doc="Sets the CCOB field."]
    #[inline] pub fn set_ccob<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fccoblo {
    #[inline]
    fn from(other: u8) -> Self {
         Fccoblo(other)
    }
}

impl ::core::fmt::Display for Fccoblo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fccoblo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ccob() != 0 { try!(write!(f, " ccob=0x{:x}", self.ccob()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Flash Option Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fopt(pub u8);
impl Fopt {
    #[doc="Nonvolatile Bits"]
    #[inline] pub fn nv(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if NV != 0"]
    #[inline] pub fn test_nv(&self) -> bool {
        self.nv() != 0
    }

    #[doc="Sets the NV field."]
    #[inline] pub fn set_nv<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Fopt {
    #[inline]
    fn from(other: u8) -> Self {
         Fopt(other)
    }
}

impl ::core::fmt::Display for Fopt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fopt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.nv() != 0 { try!(write!(f, " nv=0x{:x}", self.nv()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

