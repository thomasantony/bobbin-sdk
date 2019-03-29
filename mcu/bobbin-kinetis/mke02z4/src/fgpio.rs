::bobbin_mcu::periph!( FGPIOA, Fgpioa, FGPIOA_PERIPH, FgpioPeriph, FGPIOA_OWNED, FGPIOA_REF_COUNT, 0xf8000000, 0x00, 0x1d);
::bobbin_mcu::periph!( FGPIOB, Fgpiob, FGPIOB_PERIPH, FgpioPeriph, FGPIOB_OWNED, FGPIOB_REF_COUNT, 0xf8000040, 0x01, 0x1e);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="FGPIO Peripheral"]
pub struct FgpioPeriph(pub usize); 

impl FgpioPeriph {
    #[doc="Get the PDOR Register."]
    #[inline] pub fn pdor_reg(&self) -> ::bobbin_mcu::register::Register<Pdor> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdor, 0x0)
    }

    #[doc="Get the *mut pointer for the PDOR register."]
    #[inline] pub fn pdor_mut(&self) -> *mut Pdor { 
        self.pdor_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDOR register."]
    #[inline] pub fn pdor_ptr(&self) -> *const Pdor { 
        self.pdor_reg().ptr()
    }

    #[doc="Read the PDOR register."]
    #[inline] pub fn pdor(&self) -> Pdor { 
        self.pdor_reg().read()
    }

    #[doc="Write the PDOR register."]
    #[inline] pub fn write_pdor(&self, value: Pdor) -> &Self { 
        self.pdor_reg().write(value);
        self
    }

    #[doc="Set the PDOR register."]
    #[inline] pub fn set_pdor<F: FnOnce(Pdor) -> Pdor>(&self, f: F) -> &Self {
        self.pdor_reg().set(f);
        self
    }

    #[doc="Modify the PDOR register."]
    #[inline] pub fn with_pdor<F: FnOnce(Pdor) -> Pdor>(&self, f: F) -> &Self {
        self.pdor_reg().with(f);
        self
    }

    #[doc="Get the PSOR Register."]
    #[inline] pub fn psor_reg(&self) -> ::bobbin_mcu::register::Register<Psor> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Psor, 0x4)
    }

    #[doc="Get the *mut pointer for the PSOR register."]
    #[inline] pub fn psor_mut(&self) -> *mut Psor { 
        self.psor_reg().ptr()
    }

    #[doc="Get the *const pointer for the PSOR register."]
    #[inline] pub fn psor_ptr(&self) -> *const Psor { 
        self.psor_reg().ptr()
    }

    #[doc="Write the PSOR register."]
    #[inline] pub fn write_psor(&self, value: Psor) -> &Self { 
        self.psor_reg().write(value);
        self
    }

    #[doc="Set the PSOR register."]
    #[inline] pub fn set_psor<F: FnOnce(Psor) -> Psor>(&self, f: F) -> &Self {
        self.psor_reg().set(f);
        self
    }

    #[doc="Get the PCOR Register."]
    #[inline] pub fn pcor_reg(&self) -> ::bobbin_mcu::register::Register<Pcor> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pcor, 0x8)
    }

    #[doc="Get the *mut pointer for the PCOR register."]
    #[inline] pub fn pcor_mut(&self) -> *mut Pcor { 
        self.pcor_reg().ptr()
    }

    #[doc="Get the *const pointer for the PCOR register."]
    #[inline] pub fn pcor_ptr(&self) -> *const Pcor { 
        self.pcor_reg().ptr()
    }

    #[doc="Write the PCOR register."]
    #[inline] pub fn write_pcor(&self, value: Pcor) -> &Self { 
        self.pcor_reg().write(value);
        self
    }

    #[doc="Set the PCOR register."]
    #[inline] pub fn set_pcor<F: FnOnce(Pcor) -> Pcor>(&self, f: F) -> &Self {
        self.pcor_reg().set(f);
        self
    }

    #[doc="Get the PTOR Register."]
    #[inline] pub fn ptor_reg(&self) -> ::bobbin_mcu::register::Register<Ptor> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ptor, 0xc)
    }

    #[doc="Get the *mut pointer for the PTOR register."]
    #[inline] pub fn ptor_mut(&self) -> *mut Ptor { 
        self.ptor_reg().ptr()
    }

    #[doc="Get the *const pointer for the PTOR register."]
    #[inline] pub fn ptor_ptr(&self) -> *const Ptor { 
        self.ptor_reg().ptr()
    }

    #[doc="Write the PTOR register."]
    #[inline] pub fn write_ptor(&self, value: Ptor) -> &Self { 
        self.ptor_reg().write(value);
        self
    }

    #[doc="Set the PTOR register."]
    #[inline] pub fn set_ptor<F: FnOnce(Ptor) -> Ptor>(&self, f: F) -> &Self {
        self.ptor_reg().set(f);
        self
    }

    #[doc="Get the PDIR Register."]
    #[inline] pub fn pdir_reg(&self) -> ::bobbin_mcu::register::Register<Pdir> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pdir, 0x10)
    }

    #[doc="Get the *mut pointer for the PDIR register."]
    #[inline] pub fn pdir_mut(&self) -> *mut Pdir { 
        self.pdir_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDIR register."]
    #[inline] pub fn pdir_ptr(&self) -> *const Pdir { 
        self.pdir_reg().ptr()
    }

    #[doc="Read the PDIR register."]
    #[inline] pub fn pdir(&self) -> Pdir { 
        self.pdir_reg().read()
    }

    #[doc="Get the PDDR Register."]
    #[inline] pub fn pddr_reg(&self) -> ::bobbin_mcu::register::Register<Pddr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pddr, 0x14)
    }

    #[doc="Get the *mut pointer for the PDDR register."]
    #[inline] pub fn pddr_mut(&self) -> *mut Pddr { 
        self.pddr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PDDR register."]
    #[inline] pub fn pddr_ptr(&self) -> *const Pddr { 
        self.pddr_reg().ptr()
    }

    #[doc="Read the PDDR register."]
    #[inline] pub fn pddr(&self) -> Pddr { 
        self.pddr_reg().read()
    }

    #[doc="Write the PDDR register."]
    #[inline] pub fn write_pddr(&self, value: Pddr) -> &Self { 
        self.pddr_reg().write(value);
        self
    }

    #[doc="Set the PDDR register."]
    #[inline] pub fn set_pddr<F: FnOnce(Pddr) -> Pddr>(&self, f: F) -> &Self {
        self.pddr_reg().set(f);
        self
    }

    #[doc="Modify the PDDR register."]
    #[inline] pub fn with_pddr<F: FnOnce(Pddr) -> Pddr>(&self, f: F) -> &Self {
        self.pddr_reg().with(f);
        self
    }

    #[doc="Get the PIDR Register."]
    #[inline] pub fn pidr_reg(&self) -> ::bobbin_mcu::register::Register<Pidr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pidr, 0x18)
    }

    #[doc="Get the *mut pointer for the PIDR register."]
    #[inline] pub fn pidr_mut(&self) -> *mut Pidr { 
        self.pidr_reg().ptr()
    }

    #[doc="Get the *const pointer for the PIDR register."]
    #[inline] pub fn pidr_ptr(&self) -> *const Pidr { 
        self.pidr_reg().ptr()
    }

    #[doc="Read the PIDR register."]
    #[inline] pub fn pidr(&self) -> Pidr { 
        self.pidr_reg().read()
    }

    #[doc="Write the PIDR register."]
    #[inline] pub fn write_pidr(&self, value: Pidr) -> &Self { 
        self.pidr_reg().write(value);
        self
    }

    #[doc="Set the PIDR register."]
    #[inline] pub fn set_pidr<F: FnOnce(Pidr) -> Pidr>(&self, f: F) -> &Self {
        self.pidr_reg().set(f);
        self
    }

    #[doc="Modify the PIDR register."]
    #[inline] pub fn with_pidr<F: FnOnce(Pidr) -> Pidr>(&self, f: F) -> &Self {
        self.pidr_reg().with(f);
        self
    }

}

#[doc="Port Data Output Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdor(pub u32);
impl Pdor {
    #[doc="Port Data Output"]
    #[inline] pub fn pdo(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PDO != 0"]
    #[inline] pub fn test_pdo(&self) -> bool {
        self.pdo() != 0
    }

    #[doc="Sets the PDO field."]
    #[inline] pub fn set_pdo<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdor {
    #[inline]
    fn from(other: u32) -> Self {
         Pdor(other)
    }
}

impl ::core::fmt::Display for Pdor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Set Output Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Psor(pub u32);
impl Psor {
    #[doc="Port Set Output"]
    #[inline] pub fn ptso(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PTSO != 0"]
    #[inline] pub fn test_ptso(&self) -> bool {
        self.ptso() != 0
    }

    #[doc="Sets the PTSO field."]
    #[inline] pub fn set_ptso<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Psor {
    #[inline]
    fn from(other: u32) -> Self {
         Psor(other)
    }
}

impl ::core::fmt::Display for Psor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Psor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Clear Output Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcor(pub u32);
impl Pcor {
    #[doc="Port Clear Output"]
    #[inline] pub fn ptco(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PTCO != 0"]
    #[inline] pub fn test_ptco(&self) -> bool {
        self.ptco() != 0
    }

    #[doc="Sets the PTCO field."]
    #[inline] pub fn set_ptco<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pcor {
    #[inline]
    fn from(other: u32) -> Self {
         Pcor(other)
    }
}

impl ::core::fmt::Display for Pcor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pcor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Toggle Output Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptor(pub u32);
impl Ptor {
    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PTTO != 0"]
    #[inline] pub fn test_ptto(&self) -> bool {
        self.ptto() != 0
    }

    #[doc="Sets the PTTO field."]
    #[inline] pub fn set_ptto<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ptor {
    #[inline]
    fn from(other: u32) -> Self {
         Ptor(other)
    }
}

impl ::core::fmt::Display for Ptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Data Input Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdir(pub u32);
impl Pdir {
    #[doc="Port Data Input"]
    #[inline] pub fn pdi(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PDI != 0"]
    #[inline] pub fn test_pdi(&self) -> bool {
        self.pdi() != 0
    }

    #[doc="Sets the PDI field."]
    #[inline] pub fn set_pdi<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pdir {
    #[inline]
    fn from(other: u32) -> Self {
         Pdir(other)
    }
}

impl ::core::fmt::Display for Pdir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pdir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Data Direction Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pddr(pub u32);
impl Pddr {
    #[doc="Port Data Direction"]
    #[inline] pub fn pdd(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PDD != 0"]
    #[inline] pub fn test_pdd(&self) -> bool {
        self.pdd() != 0
    }

    #[doc="Sets the PDD field."]
    #[inline] pub fn set_pdd<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pddr {
    #[inline]
    fn from(other: u32) -> Self {
         Pddr(other)
    }
}

impl ::core::fmt::Display for Pddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pddr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Input Disable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pidr(pub u32);
impl Pidr {
    #[doc="Port Input Disable"]
    #[inline] pub fn pid(&self) -> ::bobbin_bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if PID != 0"]
    #[inline] pub fn test_pid(&self) -> bool {
        self.pid() != 0
    }

    #[doc="Sets the PID field."]
    #[inline] pub fn set_pid<V: Into<::bobbin_bits::U32>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Pidr {
    #[inline]
    fn from(other: u32) -> Self {
         Pidr(other)
    }
}

impl ::core::fmt::Display for Pidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pidr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

