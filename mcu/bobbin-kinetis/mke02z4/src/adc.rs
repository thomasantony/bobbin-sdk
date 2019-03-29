::bobbin_mcu::periph!( ADC, Adc, ADC_PERIPH, AdcPeriph, ADC_OWNED, ADC_REF_COUNT, 0x4003b000, 0x00, 0x07);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="ADC Peripheral"]
pub struct AdcPeriph(pub usize); 

impl AdcPeriph {
    #[doc="Get the SC1 Register."]
    #[inline] pub fn sc1_reg(&self) -> ::bobbin_mcu::register::Register<Sc1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sc1, 0x0)
    }

    #[doc="Get the *mut pointer for the SC1 register."]
    #[inline] pub fn sc1_mut(&self) -> *mut Sc1 { 
        self.sc1_reg().ptr()
    }

    #[doc="Get the *const pointer for the SC1 register."]
    #[inline] pub fn sc1_ptr(&self) -> *const Sc1 { 
        self.sc1_reg().ptr()
    }

    #[doc="Read the SC1 register."]
    #[inline] pub fn sc1(&self) -> Sc1 { 
        self.sc1_reg().read()
    }

    #[doc="Write the SC1 register."]
    #[inline] pub fn write_sc1(&self, value: Sc1) -> &Self { 
        self.sc1_reg().write(value);
        self
    }

    #[doc="Set the SC1 register."]
    #[inline] pub fn set_sc1<F: FnOnce(Sc1) -> Sc1>(&self, f: F) -> &Self {
        self.sc1_reg().set(f);
        self
    }

    #[doc="Modify the SC1 register."]
    #[inline] pub fn with_sc1<F: FnOnce(Sc1) -> Sc1>(&self, f: F) -> &Self {
        self.sc1_reg().with(f);
        self
    }

    #[doc="Get the SC2 Register."]
    #[inline] pub fn sc2_reg(&self) -> ::bobbin_mcu::register::Register<Sc2> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sc2, 0x4)
    }

    #[doc="Get the *mut pointer for the SC2 register."]
    #[inline] pub fn sc2_mut(&self) -> *mut Sc2 { 
        self.sc2_reg().ptr()
    }

    #[doc="Get the *const pointer for the SC2 register."]
    #[inline] pub fn sc2_ptr(&self) -> *const Sc2 { 
        self.sc2_reg().ptr()
    }

    #[doc="Read the SC2 register."]
    #[inline] pub fn sc2(&self) -> Sc2 { 
        self.sc2_reg().read()
    }

    #[doc="Write the SC2 register."]
    #[inline] pub fn write_sc2(&self, value: Sc2) -> &Self { 
        self.sc2_reg().write(value);
        self
    }

    #[doc="Set the SC2 register."]
    #[inline] pub fn set_sc2<F: FnOnce(Sc2) -> Sc2>(&self, f: F) -> &Self {
        self.sc2_reg().set(f);
        self
    }

    #[doc="Modify the SC2 register."]
    #[inline] pub fn with_sc2<F: FnOnce(Sc2) -> Sc2>(&self, f: F) -> &Self {
        self.sc2_reg().with(f);
        self
    }

    #[doc="Get the SC3 Register."]
    #[inline] pub fn sc3_reg(&self) -> ::bobbin_mcu::register::Register<Sc3> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sc3, 0x8)
    }

    #[doc="Get the *mut pointer for the SC3 register."]
    #[inline] pub fn sc3_mut(&self) -> *mut Sc3 { 
        self.sc3_reg().ptr()
    }

    #[doc="Get the *const pointer for the SC3 register."]
    #[inline] pub fn sc3_ptr(&self) -> *const Sc3 { 
        self.sc3_reg().ptr()
    }

    #[doc="Read the SC3 register."]
    #[inline] pub fn sc3(&self) -> Sc3 { 
        self.sc3_reg().read()
    }

    #[doc="Write the SC3 register."]
    #[inline] pub fn write_sc3(&self, value: Sc3) -> &Self { 
        self.sc3_reg().write(value);
        self
    }

    #[doc="Set the SC3 register."]
    #[inline] pub fn set_sc3<F: FnOnce(Sc3) -> Sc3>(&self, f: F) -> &Self {
        self.sc3_reg().set(f);
        self
    }

    #[doc="Modify the SC3 register."]
    #[inline] pub fn with_sc3<F: FnOnce(Sc3) -> Sc3>(&self, f: F) -> &Self {
        self.sc3_reg().with(f);
        self
    }

    #[doc="Get the SC4 Register."]
    #[inline] pub fn sc4_reg(&self) -> ::bobbin_mcu::register::Register<Sc4> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sc4, 0xc)
    }

    #[doc="Get the *mut pointer for the SC4 register."]
    #[inline] pub fn sc4_mut(&self) -> *mut Sc4 { 
        self.sc4_reg().ptr()
    }

    #[doc="Get the *const pointer for the SC4 register."]
    #[inline] pub fn sc4_ptr(&self) -> *const Sc4 { 
        self.sc4_reg().ptr()
    }

    #[doc="Read the SC4 register."]
    #[inline] pub fn sc4(&self) -> Sc4 { 
        self.sc4_reg().read()
    }

    #[doc="Write the SC4 register."]
    #[inline] pub fn write_sc4(&self, value: Sc4) -> &Self { 
        self.sc4_reg().write(value);
        self
    }

    #[doc="Set the SC4 register."]
    #[inline] pub fn set_sc4<F: FnOnce(Sc4) -> Sc4>(&self, f: F) -> &Self {
        self.sc4_reg().set(f);
        self
    }

    #[doc="Modify the SC4 register."]
    #[inline] pub fn with_sc4<F: FnOnce(Sc4) -> Sc4>(&self, f: F) -> &Self {
        self.sc4_reg().with(f);
        self
    }

    #[doc="Get the R Register."]
    #[inline] pub fn r_reg(&self) -> ::bobbin_mcu::register::Register<R> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut R, 0x10)
    }

    #[doc="Get the *mut pointer for the R register."]
    #[inline] pub fn r_mut(&self) -> *mut R { 
        self.r_reg().ptr()
    }

    #[doc="Get the *const pointer for the R register."]
    #[inline] pub fn r_ptr(&self) -> *const R { 
        self.r_reg().ptr()
    }

    #[doc="Read the R register."]
    #[inline] pub fn r(&self) -> R { 
        self.r_reg().read()
    }

    #[doc="Get the CV Register."]
    #[inline] pub fn cv_reg(&self) -> ::bobbin_mcu::register::Register<Cv> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cv, 0x14)
    }

    #[doc="Get the *mut pointer for the CV register."]
    #[inline] pub fn cv_mut(&self) -> *mut Cv { 
        self.cv_reg().ptr()
    }

    #[doc="Get the *const pointer for the CV register."]
    #[inline] pub fn cv_ptr(&self) -> *const Cv { 
        self.cv_reg().ptr()
    }

    #[doc="Read the CV register."]
    #[inline] pub fn cv(&self) -> Cv { 
        self.cv_reg().read()
    }

    #[doc="Write the CV register."]
    #[inline] pub fn write_cv(&self, value: Cv) -> &Self { 
        self.cv_reg().write(value);
        self
    }

    #[doc="Set the CV register."]
    #[inline] pub fn set_cv<F: FnOnce(Cv) -> Cv>(&self, f: F) -> &Self {
        self.cv_reg().set(f);
        self
    }

    #[doc="Modify the CV register."]
    #[inline] pub fn with_cv<F: FnOnce(Cv) -> Cv>(&self, f: F) -> &Self {
        self.cv_reg().with(f);
        self
    }

    #[doc="Get the APCTL1 Register."]
    #[inline] pub fn apctl1_reg(&self) -> ::bobbin_mcu::register::Register<Apctl1> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Apctl1, 0x18)
    }

    #[doc="Get the *mut pointer for the APCTL1 register."]
    #[inline] pub fn apctl1_mut(&self) -> *mut Apctl1 { 
        self.apctl1_reg().ptr()
    }

    #[doc="Get the *const pointer for the APCTL1 register."]
    #[inline] pub fn apctl1_ptr(&self) -> *const Apctl1 { 
        self.apctl1_reg().ptr()
    }

    #[doc="Read the APCTL1 register."]
    #[inline] pub fn apctl1(&self) -> Apctl1 { 
        self.apctl1_reg().read()
    }

    #[doc="Write the APCTL1 register."]
    #[inline] pub fn write_apctl1(&self, value: Apctl1) -> &Self { 
        self.apctl1_reg().write(value);
        self
    }

    #[doc="Set the APCTL1 register."]
    #[inline] pub fn set_apctl1<F: FnOnce(Apctl1) -> Apctl1>(&self, f: F) -> &Self {
        self.apctl1_reg().set(f);
        self
    }

    #[doc="Modify the APCTL1 register."]
    #[inline] pub fn with_apctl1<F: FnOnce(Apctl1) -> Apctl1>(&self, f: F) -> &Self {
        self.apctl1_reg().with(f);
        self
    }

}

#[doc="Status and Control Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc1(pub u32);
impl Sc1 {
    #[doc="Input Channel Select"]
    #[inline] pub fn adch(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if ADCH != 0"]
    #[inline] pub fn test_adch(&self) -> bool {
        self.adch() != 0
    }

    #[doc="Sets the ADCH field."]
    #[inline] pub fn set_adch<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Continuous Conversion Enable"]
    #[inline] pub fn adco(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ADCO != 0"]
    #[inline] pub fn test_adco(&self) -> bool {
        self.adco() != 0
    }

    #[doc="Sets the ADCO field."]
    #[inline] pub fn set_adco<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Interrupt Enable"]
    #[inline] pub fn aien(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if AIEN != 0"]
    #[inline] pub fn test_aien(&self) -> bool {
        self.aien() != 0
    }

    #[doc="Sets the AIEN field."]
    #[inline] pub fn set_aien<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Conversion Complete Flag"]
    #[inline] pub fn coco(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if COCO != 0"]
    #[inline] pub fn test_coco(&self) -> bool {
        self.coco() != 0
    }

    #[doc="Sets the COCO field."]
    #[inline] pub fn set_coco<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Sc1 {
    #[inline]
    fn from(other: u32) -> Self {
         Sc1(other)
    }
}

impl ::core::fmt::Display for Sc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sc1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adch() != 0 { try!(write!(f, " adch=0x{:x}", self.adch()))}
        if self.adco() != 0 { try!(write!(f, " adco"))}
        if self.aien() != 0 { try!(write!(f, " aien"))}
        if self.coco() != 0 { try!(write!(f, " coco"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status and Control Register 2"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc2(pub u32);
impl Sc2 {
    #[doc="Voltage Reference Selection"]
    #[inline] pub fn refsel(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if REFSEL != 0"]
    #[inline] pub fn test_refsel(&self) -> bool {
        self.refsel() != 0
    }

    #[doc="Sets the REFSEL field."]
    #[inline] pub fn set_refsel<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Result FIFO full"]
    #[inline] pub fn ffull(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FFULL != 0"]
    #[inline] pub fn test_ffull(&self) -> bool {
        self.ffull() != 0
    }

    #[doc="Sets the FFULL field."]
    #[inline] pub fn set_ffull<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Result FIFO empty"]
    #[inline] pub fn fempty(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FEMPTY != 0"]
    #[inline] pub fn test_fempty(&self) -> bool {
        self.fempty() != 0
    }

    #[doc="Sets the FEMPTY field."]
    #[inline] pub fn set_fempty<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Compare Function Greater Than Enable"]
    #[inline] pub fn acfgt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ACFGT != 0"]
    #[inline] pub fn test_acfgt(&self) -> bool {
        self.acfgt() != 0
    }

    #[doc="Sets the ACFGT field."]
    #[inline] pub fn set_acfgt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Compare Function Enable"]
    #[inline] pub fn acfe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACFE != 0"]
    #[inline] pub fn test_acfe(&self) -> bool {
        self.acfe() != 0
    }

    #[doc="Sets the ACFE field."]
    #[inline] pub fn set_acfe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Conversion Trigger Select"]
    #[inline] pub fn adtrg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ADTRG != 0"]
    #[inline] pub fn test_adtrg(&self) -> bool {
        self.adtrg() != 0
    }

    #[doc="Sets the ADTRG field."]
    #[inline] pub fn set_adtrg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Conversion Active"]
    #[inline] pub fn adact(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ADACT != 0"]
    #[inline] pub fn test_adact(&self) -> bool {
        self.adact() != 0
    }

    #[doc="Sets the ADACT field."]
    #[inline] pub fn set_adact<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Sc2 {
    #[inline]
    fn from(other: u32) -> Self {
         Sc2(other)
    }
}

impl ::core::fmt::Display for Sc2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sc2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.refsel() != 0 { try!(write!(f, " refsel=0x{:x}", self.refsel()))}
        if self.ffull() != 0 { try!(write!(f, " ffull"))}
        if self.fempty() != 0 { try!(write!(f, " fempty"))}
        if self.acfgt() != 0 { try!(write!(f, " acfgt"))}
        if self.acfe() != 0 { try!(write!(f, " acfe"))}
        if self.adtrg() != 0 { try!(write!(f, " adtrg"))}
        if self.adact() != 0 { try!(write!(f, " adact"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status and Control Register 3"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc3(pub u32);
impl Sc3 {
    #[doc="Input Clock Select"]
    #[inline] pub fn adiclk(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if ADICLK != 0"]
    #[inline] pub fn test_adiclk(&self) -> bool {
        self.adiclk() != 0
    }

    #[doc="Sets the ADICLK field."]
    #[inline] pub fn set_adiclk<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Conversion Mode Selection"]
    #[inline] pub fn mode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if MODE != 0"]
    #[inline] pub fn test_mode(&self) -> bool {
        self.mode() != 0
    }

    #[doc="Sets the MODE field."]
    #[inline] pub fn set_mode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Long Sample Time Configuration"]
    #[inline] pub fn adlsmp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if ADLSMP != 0"]
    #[inline] pub fn test_adlsmp(&self) -> bool {
        self.adlsmp() != 0
    }

    #[doc="Sets the ADLSMP field."]
    #[inline] pub fn set_adlsmp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Clock Divide Select"]
    #[inline] pub fn adiv(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if ADIV != 0"]
    #[inline] pub fn test_adiv(&self) -> bool {
        self.adiv() != 0
    }

    #[doc="Sets the ADIV field."]
    #[inline] pub fn set_adiv<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Low-Power Configuration"]
    #[inline] pub fn adlpc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ADLPC != 0"]
    #[inline] pub fn test_adlpc(&self) -> bool {
        self.adlpc() != 0
    }

    #[doc="Sets the ADLPC field."]
    #[inline] pub fn set_adlpc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Sc3 {
    #[inline]
    fn from(other: u32) -> Self {
         Sc3(other)
    }
}

impl ::core::fmt::Display for Sc3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sc3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adiclk() != 0 { try!(write!(f, " adiclk=0x{:x}", self.adiclk()))}
        if self.mode() != 0 { try!(write!(f, " mode=0x{:x}", self.mode()))}
        if self.adlsmp() != 0 { try!(write!(f, " adlsmp"))}
        if self.adiv() != 0 { try!(write!(f, " adiv=0x{:x}", self.adiv()))}
        if self.adlpc() != 0 { try!(write!(f, " adlpc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Status and Control Register 4"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc4(pub u32);
impl Sc4 {
    #[doc="FIFO Depth"]
    #[inline] pub fn afdep(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if AFDEP != 0"]
    #[inline] pub fn test_afdep(&self) -> bool {
        self.afdep() != 0
    }

    #[doc="Sets the AFDEP field."]
    #[inline] pub fn set_afdep<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Compare Function Selection"]
    #[inline] pub fn acfsel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if ACFSEL != 0"]
    #[inline] pub fn test_acfsel(&self) -> bool {
        self.acfsel() != 0
    }

    #[doc="Sets the ACFSEL field."]
    #[inline] pub fn set_acfsel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="FIFO Scan Mode Enable"]
    #[inline] pub fn ascane(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ASCANE != 0"]
    #[inline] pub fn test_ascane(&self) -> bool {
        self.ascane() != 0
    }

    #[doc="Sets the ASCANE field."]
    #[inline] pub fn set_ascane<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Sc4 {
    #[inline]
    fn from(other: u32) -> Self {
         Sc4(other)
    }
}

impl ::core::fmt::Display for Sc4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sc4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.afdep() != 0 { try!(write!(f, " afdep=0x{:x}", self.afdep()))}
        if self.acfsel() != 0 { try!(write!(f, " acfsel"))}
        if self.ascane() != 0 { try!(write!(f, " ascane"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Conversion Result Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct R(pub u32);
impl R {
    #[doc="Conversion Result"]
    #[inline] pub fn adr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if ADR != 0"]
    #[inline] pub fn test_adr(&self) -> bool {
        self.adr() != 0
    }

    #[doc="Sets the ADR field."]
    #[inline] pub fn set_adr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for R {
    #[inline]
    fn from(other: u32) -> Self {
         R(other)
    }
}

impl ::core::fmt::Display for R {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for R {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adr() != 0 { try!(write!(f, " adr=0x{:x}", self.adr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Compare Value Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cv(pub u32);
impl Cv {
    #[doc="Conversion Result[11:0]"]
    #[inline] pub fn cv(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if CV != 0"]
    #[inline] pub fn test_cv(&self) -> bool {
        self.cv() != 0
    }

    #[doc="Sets the CV field."]
    #[inline] pub fn set_cv<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
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
        if self.cv() != 0 { try!(write!(f, " cv=0x{:x}", self.cv()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Pin Control 1 Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Apctl1(pub u32);
impl Apctl1 {
    #[doc="ADC Pin Control"]
    #[inline] pub fn adpc(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if ADPC != 0"]
    #[inline] pub fn test_adpc(&self) -> bool {
        self.adpc() != 0
    }

    #[doc="Sets the ADPC field."]
    #[inline] pub fn set_adpc<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Apctl1 {
    #[inline]
    fn from(other: u32) -> Self {
         Apctl1(other)
    }
}

impl ::core::fmt::Display for Apctl1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Apctl1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.adpc() != 0 { try!(write!(f, " adpc=0x{:x}", self.adpc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

