::bobbin_mcu::periph!( PORT, Port, PORT_PERIPH, PortPeriph, PORT_OWNED, PORT_REF_COUNT, 0x40049000, 0x00, 0x0a);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="PORT Peripheral"]
pub struct PortPeriph(pub usize); 

impl PortPeriph {
    #[doc="Get the IOFLT Register."]
    #[inline] pub fn ioflt_reg(&self) -> ::bobbin_mcu::register::Register<Ioflt> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ioflt, 0x0)
    }

    #[doc="Get the *mut pointer for the IOFLT register."]
    #[inline] pub fn ioflt_mut(&self) -> *mut Ioflt { 
        self.ioflt_reg().ptr()
    }

    #[doc="Get the *const pointer for the IOFLT register."]
    #[inline] pub fn ioflt_ptr(&self) -> *const Ioflt { 
        self.ioflt_reg().ptr()
    }

    #[doc="Read the IOFLT register."]
    #[inline] pub fn ioflt(&self) -> Ioflt { 
        self.ioflt_reg().read()
    }

    #[doc="Write the IOFLT register."]
    #[inline] pub fn write_ioflt(&self, value: Ioflt) -> &Self { 
        self.ioflt_reg().write(value);
        self
    }

    #[doc="Set the IOFLT register."]
    #[inline] pub fn set_ioflt<F: FnOnce(Ioflt) -> Ioflt>(&self, f: F) -> &Self {
        self.ioflt_reg().set(f);
        self
    }

    #[doc="Modify the IOFLT register."]
    #[inline] pub fn with_ioflt<F: FnOnce(Ioflt) -> Ioflt>(&self, f: F) -> &Self {
        self.ioflt_reg().with(f);
        self
    }

    #[doc="Get the PUEL Register."]
    #[inline] pub fn puel_reg(&self) -> ::bobbin_mcu::register::Register<Puel> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Puel, 0x4)
    }

    #[doc="Get the *mut pointer for the PUEL register."]
    #[inline] pub fn puel_mut(&self) -> *mut Puel { 
        self.puel_reg().ptr()
    }

    #[doc="Get the *const pointer for the PUEL register."]
    #[inline] pub fn puel_ptr(&self) -> *const Puel { 
        self.puel_reg().ptr()
    }

    #[doc="Read the PUEL register."]
    #[inline] pub fn puel(&self) -> Puel { 
        self.puel_reg().read()
    }

    #[doc="Write the PUEL register."]
    #[inline] pub fn write_puel(&self, value: Puel) -> &Self { 
        self.puel_reg().write(value);
        self
    }

    #[doc="Set the PUEL register."]
    #[inline] pub fn set_puel<F: FnOnce(Puel) -> Puel>(&self, f: F) -> &Self {
        self.puel_reg().set(f);
        self
    }

    #[doc="Modify the PUEL register."]
    #[inline] pub fn with_puel<F: FnOnce(Puel) -> Puel>(&self, f: F) -> &Self {
        self.puel_reg().with(f);
        self
    }

    #[doc="Get the PUEH Register."]
    #[inline] pub fn pueh_reg(&self) -> ::bobbin_mcu::register::Register<Pueh> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pueh, 0x8)
    }

    #[doc="Get the *mut pointer for the PUEH register."]
    #[inline] pub fn pueh_mut(&self) -> *mut Pueh { 
        self.pueh_reg().ptr()
    }

    #[doc="Get the *const pointer for the PUEH register."]
    #[inline] pub fn pueh_ptr(&self) -> *const Pueh { 
        self.pueh_reg().ptr()
    }

    #[doc="Read the PUEH register."]
    #[inline] pub fn pueh(&self) -> Pueh { 
        self.pueh_reg().read()
    }

    #[doc="Write the PUEH register."]
    #[inline] pub fn write_pueh(&self, value: Pueh) -> &Self { 
        self.pueh_reg().write(value);
        self
    }

    #[doc="Set the PUEH register."]
    #[inline] pub fn set_pueh<F: FnOnce(Pueh) -> Pueh>(&self, f: F) -> &Self {
        self.pueh_reg().set(f);
        self
    }

    #[doc="Modify the PUEH register."]
    #[inline] pub fn with_pueh<F: FnOnce(Pueh) -> Pueh>(&self, f: F) -> &Self {
        self.pueh_reg().with(f);
        self
    }

    #[doc="Get the HDRVE Register."]
    #[inline] pub fn hdrve_reg(&self) -> ::bobbin_mcu::register::Register<Hdrve> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Hdrve, 0xc)
    }

    #[doc="Get the *mut pointer for the HDRVE register."]
    #[inline] pub fn hdrve_mut(&self) -> *mut Hdrve { 
        self.hdrve_reg().ptr()
    }

    #[doc="Get the *const pointer for the HDRVE register."]
    #[inline] pub fn hdrve_ptr(&self) -> *const Hdrve { 
        self.hdrve_reg().ptr()
    }

    #[doc="Read the HDRVE register."]
    #[inline] pub fn hdrve(&self) -> Hdrve { 
        self.hdrve_reg().read()
    }

    #[doc="Write the HDRVE register."]
    #[inline] pub fn write_hdrve(&self, value: Hdrve) -> &Self { 
        self.hdrve_reg().write(value);
        self
    }

    #[doc="Set the HDRVE register."]
    #[inline] pub fn set_hdrve<F: FnOnce(Hdrve) -> Hdrve>(&self, f: F) -> &Self {
        self.hdrve_reg().set(f);
        self
    }

    #[doc="Modify the HDRVE register."]
    #[inline] pub fn with_hdrve<F: FnOnce(Hdrve) -> Hdrve>(&self, f: F) -> &Self {
        self.hdrve_reg().with(f);
        self
    }

}

#[doc="Port Filter Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ioflt(pub u32);
impl Ioflt {
    #[doc="Filter Selection for Input from PTA"]
    #[inline] pub fn flta(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if FLTA != 0"]
    #[inline] pub fn test_flta(&self) -> bool {
        self.flta() != 0
    }

    #[doc="Sets the FLTA field."]
    #[inline] pub fn set_flta<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Filter Selection for Input from PTB"]
    #[inline] pub fn fltb(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x3) as u8) } // [3:2]
    }

    #[doc="Returns true if FLTB != 0"]
    #[inline] pub fn test_fltb(&self) -> bool {
        self.fltb() != 0
    }

    #[doc="Sets the FLTB field."]
    #[inline] pub fn set_fltb<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Filter Selection for Input from PTC"]
    #[inline] pub fn fltc(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if FLTC != 0"]
    #[inline] pub fn test_fltc(&self) -> bool {
        self.fltc() != 0
    }

    #[doc="Sets the FLTC field."]
    #[inline] pub fn set_fltc<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Filter Selection for Input from PTD"]
    #[inline] pub fn fltd(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if FLTD != 0"]
    #[inline] pub fn test_fltd(&self) -> bool {
        self.fltd() != 0
    }

    #[doc="Sets the FLTD field."]
    #[inline] pub fn set_fltd<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Filter Selection for Input from PTD"]
    #[inline] pub fn flte(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3) as u8) } // [9:8]
    }

    #[doc="Returns true if FLTE != 0"]
    #[inline] pub fn test_flte(&self) -> bool {
        self.flte() != 0
    }

    #[doc="Sets the FLTE field."]
    #[inline] pub fn set_flte<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Filter Selection for Input from PTF"]
    #[inline] pub fn fltf(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x3) as u8) } // [11:10]
    }

    #[doc="Returns true if FLTF != 0"]
    #[inline] pub fn test_fltf(&self) -> bool {
        self.fltf() != 0
    }

    #[doc="Sets the FLTF field."]
    #[inline] pub fn set_fltf<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Filter Selection for Input from PTG"]
    #[inline] pub fn fltg(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x3) as u8) } // [13:12]
    }

    #[doc="Returns true if FLTG != 0"]
    #[inline] pub fn test_fltg(&self) -> bool {
        self.fltg() != 0
    }

    #[doc="Sets the FLTG field."]
    #[inline] pub fn set_fltg<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Filter Selection for Input from PTH"]
    #[inline] pub fn flth(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x3) as u8) } // [15:14]
    }

    #[doc="Returns true if FLTH != 0"]
    #[inline] pub fn test_flth(&self) -> bool {
        self.flth() != 0
    }

    #[doc="Sets the FLTH field."]
    #[inline] pub fn set_flth<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Filter Selection for Input from RESET/IRQ"]
    #[inline] pub fn fltrst(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x3) as u8) } // [17:16]
    }

    #[doc="Returns true if FLTRST != 0"]
    #[inline] pub fn test_fltrst(&self) -> bool {
        self.fltrst() != 0
    }

    #[doc="Sets the FLTRST field."]
    #[inline] pub fn set_fltrst<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Filter selection for Input from KBI0"]
    #[inline] pub fn fltkbi0(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x3) as u8) } // [19:18]
    }

    #[doc="Returns true if FLTKBI0 != 0"]
    #[inline] pub fn test_fltkbi0(&self) -> bool {
        self.fltkbi0() != 0
    }

    #[doc="Sets the FLTKBI0 field."]
    #[inline] pub fn set_fltkbi0<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Filter Selection for Input from KBI1"]
    #[inline] pub fn fltkbi1(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x3) as u8) } // [21:20]
    }

    #[doc="Returns true if FLTKBI1 != 0"]
    #[inline] pub fn test_fltkbi1(&self) -> bool {
        self.fltkbi1() != 0
    }

    #[doc="Sets the FLTKBI1 field."]
    #[inline] pub fn set_fltkbi1<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Filter Selection for Input from NMI"]
    #[inline] pub fn fltnmi(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x3) as u8) } // [23:22]
    }

    #[doc="Returns true if FLTNMI != 0"]
    #[inline] pub fn test_fltnmi(&self) -> bool {
        self.fltnmi() != 0
    }

    #[doc="Sets the FLTNMI field."]
    #[inline] pub fn set_fltnmi<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Filter Division Set 1"]
    #[inline] pub fn fltdiv1(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if FLTDIV1 != 0"]
    #[inline] pub fn test_fltdiv1(&self) -> bool {
        self.fltdiv1() != 0
    }

    #[doc="Sets the FLTDIV1 field."]
    #[inline] pub fn set_fltdiv1<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Filter Division Set 2"]
    #[inline] pub fn fltdiv2(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x7) as u8) } // [28:26]
    }

    #[doc="Returns true if FLTDIV2 != 0"]
    #[inline] pub fn test_fltdiv2(&self) -> bool {
        self.fltdiv2() != 0
    }

    #[doc="Sets the FLTDIV2 field."]
    #[inline] pub fn set_fltdiv2<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Filter Division Set 3"]
    #[inline] pub fn fltdiv3(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x7) as u8) } // [31:29]
    }

    #[doc="Returns true if FLTDIV3 != 0"]
    #[inline] pub fn test_fltdiv3(&self) -> bool {
        self.fltdiv3() != 0
    }

    #[doc="Sets the FLTDIV3 field."]
    #[inline] pub fn set_fltdiv3<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 29);
        self.0 |= value << 29;
        self
    }

}

impl From<u32> for Ioflt {
    #[inline]
    fn from(other: u32) -> Self {
         Ioflt(other)
    }
}

impl ::core::fmt::Display for Ioflt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ioflt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flta() != 0 { try!(write!(f, " flta=0x{:x}", self.flta()))}
        if self.fltb() != 0 { try!(write!(f, " fltb=0x{:x}", self.fltb()))}
        if self.fltc() != 0 { try!(write!(f, " fltc=0x{:x}", self.fltc()))}
        if self.fltd() != 0 { try!(write!(f, " fltd=0x{:x}", self.fltd()))}
        if self.flte() != 0 { try!(write!(f, " flte=0x{:x}", self.flte()))}
        if self.fltf() != 0 { try!(write!(f, " fltf=0x{:x}", self.fltf()))}
        if self.fltg() != 0 { try!(write!(f, " fltg=0x{:x}", self.fltg()))}
        if self.flth() != 0 { try!(write!(f, " flth=0x{:x}", self.flth()))}
        if self.fltrst() != 0 { try!(write!(f, " fltrst=0x{:x}", self.fltrst()))}
        if self.fltkbi0() != 0 { try!(write!(f, " fltkbi0=0x{:x}", self.fltkbi0()))}
        if self.fltkbi1() != 0 { try!(write!(f, " fltkbi1=0x{:x}", self.fltkbi1()))}
        if self.fltnmi() != 0 { try!(write!(f, " fltnmi=0x{:x}", self.fltnmi()))}
        if self.fltdiv1() != 0 { try!(write!(f, " fltdiv1=0x{:x}", self.fltdiv1()))}
        if self.fltdiv2() != 0 { try!(write!(f, " fltdiv2=0x{:x}", self.fltdiv2()))}
        if self.fltdiv3() != 0 { try!(write!(f, " fltdiv3=0x{:x}", self.fltdiv3()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Pullup Enable Low Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Puel(pub u32);
impl Puel {
    #[doc="Pull Enable for Port A Bit 0"]
    #[inline] pub fn ptape0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PTAPE0 != 0"]
    #[inline] pub fn test_ptape0(&self) -> bool {
        self.ptape0() != 0
    }

    #[doc="Sets the PTAPE0 field."]
    #[inline] pub fn set_ptape0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Pull Enable for Port A Bit 1"]
    #[inline] pub fn ptape1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PTAPE1 != 0"]
    #[inline] pub fn test_ptape1(&self) -> bool {
        self.ptape1() != 0
    }

    #[doc="Sets the PTAPE1 field."]
    #[inline] pub fn set_ptape1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pull Enable for Port A Bit 2"]
    #[inline] pub fn ptape2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PTAPE2 != 0"]
    #[inline] pub fn test_ptape2(&self) -> bool {
        self.ptape2() != 0
    }

    #[doc="Sets the PTAPE2 field."]
    #[inline] pub fn set_ptape2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pull Enable for Port A Bit 3"]
    #[inline] pub fn ptape3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PTAPE3 != 0"]
    #[inline] pub fn test_ptape3(&self) -> bool {
        self.ptape3() != 0
    }

    #[doc="Sets the PTAPE3 field."]
    #[inline] pub fn set_ptape3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pull Enable for Port A Bit 4"]
    #[inline] pub fn ptape4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PTAPE4 != 0"]
    #[inline] pub fn test_ptape4(&self) -> bool {
        self.ptape4() != 0
    }

    #[doc="Sets the PTAPE4 field."]
    #[inline] pub fn set_ptape4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pull Enable for Port A Bit 5"]
    #[inline] pub fn ptape5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PTAPE5 != 0"]
    #[inline] pub fn test_ptape5(&self) -> bool {
        self.ptape5() != 0
    }

    #[doc="Sets the PTAPE5 field."]
    #[inline] pub fn set_ptape5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Pull Enable for Port A Bit 6"]
    #[inline] pub fn ptape6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PTAPE6 != 0"]
    #[inline] pub fn test_ptape6(&self) -> bool {
        self.ptape6() != 0
    }

    #[doc="Sets the PTAPE6 field."]
    #[inline] pub fn set_ptape6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Pull Enable for Port A Bit 7"]
    #[inline] pub fn ptape7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PTAPE7 != 0"]
    #[inline] pub fn test_ptape7(&self) -> bool {
        self.ptape7() != 0
    }

    #[doc="Sets the PTAPE7 field."]
    #[inline] pub fn set_ptape7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Pull Enable for Port B Bit 0"]
    #[inline] pub fn ptbpe0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PTBPE0 != 0"]
    #[inline] pub fn test_ptbpe0(&self) -> bool {
        self.ptbpe0() != 0
    }

    #[doc="Sets the PTBPE0 field."]
    #[inline] pub fn set_ptbpe0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pull Enable for Port B Bit 1"]
    #[inline] pub fn ptbpe1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PTBPE1 != 0"]
    #[inline] pub fn test_ptbpe1(&self) -> bool {
        self.ptbpe1() != 0
    }

    #[doc="Sets the PTBPE1 field."]
    #[inline] pub fn set_ptbpe1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pull Enable for Port B Bit 2"]
    #[inline] pub fn ptbpe2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PTBPE2 != 0"]
    #[inline] pub fn test_ptbpe2(&self) -> bool {
        self.ptbpe2() != 0
    }

    #[doc="Sets the PTBPE2 field."]
    #[inline] pub fn set_ptbpe2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pull Enable for Port B Bit 3"]
    #[inline] pub fn ptbpe3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PTBPE3 != 0"]
    #[inline] pub fn test_ptbpe3(&self) -> bool {
        self.ptbpe3() != 0
    }

    #[doc="Sets the PTBPE3 field."]
    #[inline] pub fn set_ptbpe3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pull Enable for Port B Bit 4"]
    #[inline] pub fn ptbpe4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PTBPE4 != 0"]
    #[inline] pub fn test_ptbpe4(&self) -> bool {
        self.ptbpe4() != 0
    }

    #[doc="Sets the PTBPE4 field."]
    #[inline] pub fn set_ptbpe4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pull Enable for Port B Bit 5"]
    #[inline] pub fn ptbpe5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PTBPE5 != 0"]
    #[inline] pub fn test_ptbpe5(&self) -> bool {
        self.ptbpe5() != 0
    }

    #[doc="Sets the PTBPE5 field."]
    #[inline] pub fn set_ptbpe5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Pull Enable for Port B Bit 6"]
    #[inline] pub fn ptbpe6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PTBPE6 != 0"]
    #[inline] pub fn test_ptbpe6(&self) -> bool {
        self.ptbpe6() != 0
    }

    #[doc="Sets the PTBPE6 field."]
    #[inline] pub fn set_ptbpe6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Pull Enable for Port B Bit 7"]
    #[inline] pub fn ptbpe7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PTBPE7 != 0"]
    #[inline] pub fn test_ptbpe7(&self) -> bool {
        self.ptbpe7() != 0
    }

    #[doc="Sets the PTBPE7 field."]
    #[inline] pub fn set_ptbpe7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Pull Enable for Port C Bit 0"]
    #[inline] pub fn ptcpe0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PTCPE0 != 0"]
    #[inline] pub fn test_ptcpe0(&self) -> bool {
        self.ptcpe0() != 0
    }

    #[doc="Sets the PTCPE0 field."]
    #[inline] pub fn set_ptcpe0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pull Enable for Port C Bit 1"]
    #[inline] pub fn ptcpe1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PTCPE1 != 0"]
    #[inline] pub fn test_ptcpe1(&self) -> bool {
        self.ptcpe1() != 0
    }

    #[doc="Sets the PTCPE1 field."]
    #[inline] pub fn set_ptcpe1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pull Enable for Port C Bit 2"]
    #[inline] pub fn ptcpe2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PTCPE2 != 0"]
    #[inline] pub fn test_ptcpe2(&self) -> bool {
        self.ptcpe2() != 0
    }

    #[doc="Sets the PTCPE2 field."]
    #[inline] pub fn set_ptcpe2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pull Enable for Port C Bit 3"]
    #[inline] pub fn ptcpe3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PTCPE3 != 0"]
    #[inline] pub fn test_ptcpe3(&self) -> bool {
        self.ptcpe3() != 0
    }

    #[doc="Sets the PTCPE3 field."]
    #[inline] pub fn set_ptcpe3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pull Enable for Port C Bit 4"]
    #[inline] pub fn ptcpe4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PTCPE4 != 0"]
    #[inline] pub fn test_ptcpe4(&self) -> bool {
        self.ptcpe4() != 0
    }

    #[doc="Sets the PTCPE4 field."]
    #[inline] pub fn set_ptcpe4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Pull Enable for Port C Bit 5"]
    #[inline] pub fn ptcpe5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PTCPE5 != 0"]
    #[inline] pub fn test_ptcpe5(&self) -> bool {
        self.ptcpe5() != 0
    }

    #[doc="Sets the PTCPE5 field."]
    #[inline] pub fn set_ptcpe5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Pull Enable for Port C Bit 6"]
    #[inline] pub fn ptcpe6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PTCPE6 != 0"]
    #[inline] pub fn test_ptcpe6(&self) -> bool {
        self.ptcpe6() != 0
    }

    #[doc="Sets the PTCPE6 field."]
    #[inline] pub fn set_ptcpe6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Pull Enable for Port C Bit 7"]
    #[inline] pub fn ptcpe7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PTCPE7 != 0"]
    #[inline] pub fn test_ptcpe7(&self) -> bool {
        self.ptcpe7() != 0
    }

    #[doc="Sets the PTCPE7 field."]
    #[inline] pub fn set_ptcpe7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Pull Enable for Port D Bit 0"]
    #[inline] pub fn ptdpe0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PTDPE0 != 0"]
    #[inline] pub fn test_ptdpe0(&self) -> bool {
        self.ptdpe0() != 0
    }

    #[doc="Sets the PTDPE0 field."]
    #[inline] pub fn set_ptdpe0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pull Enable for Port D Bit 1"]
    #[inline] pub fn ptdpe1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PTDPE1 != 0"]
    #[inline] pub fn test_ptdpe1(&self) -> bool {
        self.ptdpe1() != 0
    }

    #[doc="Sets the PTDPE1 field."]
    #[inline] pub fn set_ptdpe1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pull Enable for Port D Bit 2"]
    #[inline] pub fn ptdpe2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PTDPE2 != 0"]
    #[inline] pub fn test_ptdpe2(&self) -> bool {
        self.ptdpe2() != 0
    }

    #[doc="Sets the PTDPE2 field."]
    #[inline] pub fn set_ptdpe2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pull Enable for Port D Bit 3"]
    #[inline] pub fn ptdpe3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PTDPE3 != 0"]
    #[inline] pub fn test_ptdpe3(&self) -> bool {
        self.ptdpe3() != 0
    }

    #[doc="Sets the PTDPE3 field."]
    #[inline] pub fn set_ptdpe3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Pull Enable for Port D Bit 4"]
    #[inline] pub fn ptdpe4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PTDPE4 != 0"]
    #[inline] pub fn test_ptdpe4(&self) -> bool {
        self.ptdpe4() != 0
    }

    #[doc="Sets the PTDPE4 field."]
    #[inline] pub fn set_ptdpe4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Pull Enable for Port D Bit 5"]
    #[inline] pub fn ptdpe5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if PTDPE5 != 0"]
    #[inline] pub fn test_ptdpe5(&self) -> bool {
        self.ptdpe5() != 0
    }

    #[doc="Sets the PTDPE5 field."]
    #[inline] pub fn set_ptdpe5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Pull Enable for Port D Bit 6"]
    #[inline] pub fn ptdpe6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if PTDPE6 != 0"]
    #[inline] pub fn test_ptdpe6(&self) -> bool {
        self.ptdpe6() != 0
    }

    #[doc="Sets the PTDPE6 field."]
    #[inline] pub fn set_ptdpe6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Pull Enable for Port D Bit 7"]
    #[inline] pub fn ptdpe7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PTDPE7 != 0"]
    #[inline] pub fn test_ptdpe7(&self) -> bool {
        self.ptdpe7() != 0
    }

    #[doc="Sets the PTDPE7 field."]
    #[inline] pub fn set_ptdpe7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Puel {
    #[inline]
    fn from(other: u32) -> Self {
         Puel(other)
    }
}

impl ::core::fmt::Display for Puel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Puel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ptape0() != 0 { try!(write!(f, " ptape0"))}
        if self.ptape1() != 0 { try!(write!(f, " ptape1"))}
        if self.ptape2() != 0 { try!(write!(f, " ptape2"))}
        if self.ptape3() != 0 { try!(write!(f, " ptape3"))}
        if self.ptape4() != 0 { try!(write!(f, " ptape4"))}
        if self.ptape5() != 0 { try!(write!(f, " ptape5"))}
        if self.ptape6() != 0 { try!(write!(f, " ptape6"))}
        if self.ptape7() != 0 { try!(write!(f, " ptape7"))}
        if self.ptbpe0() != 0 { try!(write!(f, " ptbpe0"))}
        if self.ptbpe1() != 0 { try!(write!(f, " ptbpe1"))}
        if self.ptbpe2() != 0 { try!(write!(f, " ptbpe2"))}
        if self.ptbpe3() != 0 { try!(write!(f, " ptbpe3"))}
        if self.ptbpe4() != 0 { try!(write!(f, " ptbpe4"))}
        if self.ptbpe5() != 0 { try!(write!(f, " ptbpe5"))}
        if self.ptbpe6() != 0 { try!(write!(f, " ptbpe6"))}
        if self.ptbpe7() != 0 { try!(write!(f, " ptbpe7"))}
        if self.ptcpe0() != 0 { try!(write!(f, " ptcpe0"))}
        if self.ptcpe1() != 0 { try!(write!(f, " ptcpe1"))}
        if self.ptcpe2() != 0 { try!(write!(f, " ptcpe2"))}
        if self.ptcpe3() != 0 { try!(write!(f, " ptcpe3"))}
        if self.ptcpe4() != 0 { try!(write!(f, " ptcpe4"))}
        if self.ptcpe5() != 0 { try!(write!(f, " ptcpe5"))}
        if self.ptcpe6() != 0 { try!(write!(f, " ptcpe6"))}
        if self.ptcpe7() != 0 { try!(write!(f, " ptcpe7"))}
        if self.ptdpe0() != 0 { try!(write!(f, " ptdpe0"))}
        if self.ptdpe1() != 0 { try!(write!(f, " ptdpe1"))}
        if self.ptdpe2() != 0 { try!(write!(f, " ptdpe2"))}
        if self.ptdpe3() != 0 { try!(write!(f, " ptdpe3"))}
        if self.ptdpe4() != 0 { try!(write!(f, " ptdpe4"))}
        if self.ptdpe5() != 0 { try!(write!(f, " ptdpe5"))}
        if self.ptdpe6() != 0 { try!(write!(f, " ptdpe6"))}
        if self.ptdpe7() != 0 { try!(write!(f, " ptdpe7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Pullup Enable High Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pueh(pub u32);
impl Pueh {
    #[doc="Pull Enable for Port E Bit 0"]
    #[inline] pub fn ptepe0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PTEPE0 != 0"]
    #[inline] pub fn test_ptepe0(&self) -> bool {
        self.ptepe0() != 0
    }

    #[doc="Sets the PTEPE0 field."]
    #[inline] pub fn set_ptepe0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Pull Enable for Port E Bit 1"]
    #[inline] pub fn ptepe1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PTEPE1 != 0"]
    #[inline] pub fn test_ptepe1(&self) -> bool {
        self.ptepe1() != 0
    }

    #[doc="Sets the PTEPE1 field."]
    #[inline] pub fn set_ptepe1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pull Enable for Port E Bit 2"]
    #[inline] pub fn ptepe2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PTEPE2 != 0"]
    #[inline] pub fn test_ptepe2(&self) -> bool {
        self.ptepe2() != 0
    }

    #[doc="Sets the PTEPE2 field."]
    #[inline] pub fn set_ptepe2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pull Enable for Port E Bit 3"]
    #[inline] pub fn ptepe3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PTEPE3 != 0"]
    #[inline] pub fn test_ptepe3(&self) -> bool {
        self.ptepe3() != 0
    }

    #[doc="Sets the PTEPE3 field."]
    #[inline] pub fn set_ptepe3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Pull Enable for Port E Bit 4"]
    #[inline] pub fn ptepe4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PTEPE4 != 0"]
    #[inline] pub fn test_ptepe4(&self) -> bool {
        self.ptepe4() != 0
    }

    #[doc="Sets the PTEPE4 field."]
    #[inline] pub fn set_ptepe4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Pull Enable for Port E Bit 5"]
    #[inline] pub fn ptepe5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PTEPE5 != 0"]
    #[inline] pub fn test_ptepe5(&self) -> bool {
        self.ptepe5() != 0
    }

    #[doc="Sets the PTEPE5 field."]
    #[inline] pub fn set_ptepe5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Pull Enable for Port E Bit 6"]
    #[inline] pub fn ptepe6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PTEPE6 != 0"]
    #[inline] pub fn test_ptepe6(&self) -> bool {
        self.ptepe6() != 0
    }

    #[doc="Sets the PTEPE6 field."]
    #[inline] pub fn set_ptepe6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Pull Enable for Port E Bit 7"]
    #[inline] pub fn ptepe7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PTEPE7 != 0"]
    #[inline] pub fn test_ptepe7(&self) -> bool {
        self.ptepe7() != 0
    }

    #[doc="Sets the PTEPE7 field."]
    #[inline] pub fn set_ptepe7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Pull Enable for Port F Bit 0"]
    #[inline] pub fn ptfpe0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PTFPE0 != 0"]
    #[inline] pub fn test_ptfpe0(&self) -> bool {
        self.ptfpe0() != 0
    }

    #[doc="Sets the PTFPE0 field."]
    #[inline] pub fn set_ptfpe0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Pull Enable for Port F Bit 1"]
    #[inline] pub fn ptfpe1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PTFPE1 != 0"]
    #[inline] pub fn test_ptfpe1(&self) -> bool {
        self.ptfpe1() != 0
    }

    #[doc="Sets the PTFPE1 field."]
    #[inline] pub fn set_ptfpe1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Pull Enable for Port F Bit 2"]
    #[inline] pub fn ptfpe2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PTFPE2 != 0"]
    #[inline] pub fn test_ptfpe2(&self) -> bool {
        self.ptfpe2() != 0
    }

    #[doc="Sets the PTFPE2 field."]
    #[inline] pub fn set_ptfpe2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Pull Enable for Port F Bit 3"]
    #[inline] pub fn ptfpe3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PTFPE3 != 0"]
    #[inline] pub fn test_ptfpe3(&self) -> bool {
        self.ptfpe3() != 0
    }

    #[doc="Sets the PTFPE3 field."]
    #[inline] pub fn set_ptfpe3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Pull Enable for Port F Bit 4"]
    #[inline] pub fn ptfpe4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PTFPE4 != 0"]
    #[inline] pub fn test_ptfpe4(&self) -> bool {
        self.ptfpe4() != 0
    }

    #[doc="Sets the PTFPE4 field."]
    #[inline] pub fn set_ptfpe4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Pull Enable for Port F Bit 5"]
    #[inline] pub fn ptfpe5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PTFPE5 != 0"]
    #[inline] pub fn test_ptfpe5(&self) -> bool {
        self.ptfpe5() != 0
    }

    #[doc="Sets the PTFPE5 field."]
    #[inline] pub fn set_ptfpe5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Pull Enable for Port F Bit 6"]
    #[inline] pub fn ptfpe6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PTFPE6 != 0"]
    #[inline] pub fn test_ptfpe6(&self) -> bool {
        self.ptfpe6() != 0
    }

    #[doc="Sets the PTFPE6 field."]
    #[inline] pub fn set_ptfpe6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Pull Enable for Port F Bit 7"]
    #[inline] pub fn ptfpe7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PTFPE7 != 0"]
    #[inline] pub fn test_ptfpe7(&self) -> bool {
        self.ptfpe7() != 0
    }

    #[doc="Sets the PTFPE7 field."]
    #[inline] pub fn set_ptfpe7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Pull Enable for Port G Bit 0"]
    #[inline] pub fn ptgpe0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PTGPE0 != 0"]
    #[inline] pub fn test_ptgpe0(&self) -> bool {
        self.ptgpe0() != 0
    }

    #[doc="Sets the PTGPE0 field."]
    #[inline] pub fn set_ptgpe0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Pull Enable for Port G Bit 1"]
    #[inline] pub fn ptgpe1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PTGPE1 != 0"]
    #[inline] pub fn test_ptgpe1(&self) -> bool {
        self.ptgpe1() != 0
    }

    #[doc="Sets the PTGPE1 field."]
    #[inline] pub fn set_ptgpe1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Pull Enable for Port G Bit 2"]
    #[inline] pub fn ptgpe2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PTGPE2 != 0"]
    #[inline] pub fn test_ptgpe2(&self) -> bool {
        self.ptgpe2() != 0
    }

    #[doc="Sets the PTGPE2 field."]
    #[inline] pub fn set_ptgpe2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Pull Enable for Port G Bit 3"]
    #[inline] pub fn ptgpe3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PTGPE3 != 0"]
    #[inline] pub fn test_ptgpe3(&self) -> bool {
        self.ptgpe3() != 0
    }

    #[doc="Sets the PTGPE3 field."]
    #[inline] pub fn set_ptgpe3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Pull Enable for Port H Bit 0"]
    #[inline] pub fn pthpe0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PTHPE0 != 0"]
    #[inline] pub fn test_pthpe0(&self) -> bool {
        self.pthpe0() != 0
    }

    #[doc="Sets the PTHPE0 field."]
    #[inline] pub fn set_pthpe0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Pull Enable for Port H Bit 1"]
    #[inline] pub fn pthpe1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PTHPE1 != 0"]
    #[inline] pub fn test_pthpe1(&self) -> bool {
        self.pthpe1() != 0
    }

    #[doc="Sets the PTHPE1 field."]
    #[inline] pub fn set_pthpe1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Pull Enable for Port H Bit 2"]
    #[inline] pub fn pthpe2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PTHPE2 != 0"]
    #[inline] pub fn test_pthpe2(&self) -> bool {
        self.pthpe2() != 0
    }

    #[doc="Sets the PTHPE2 field."]
    #[inline] pub fn set_pthpe2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Pull Enable for Port H Bit 6"]
    #[inline] pub fn pthpe6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if PTHPE6 != 0"]
    #[inline] pub fn test_pthpe6(&self) -> bool {
        self.pthpe6() != 0
    }

    #[doc="Sets the PTHPE6 field."]
    #[inline] pub fn set_pthpe6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Pull Enable for Port H Bit 7"]
    #[inline] pub fn pthpe7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PTHPE7 != 0"]
    #[inline] pub fn test_pthpe7(&self) -> bool {
        self.pthpe7() != 0
    }

    #[doc="Sets the PTHPE7 field."]
    #[inline] pub fn set_pthpe7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Pueh {
    #[inline]
    fn from(other: u32) -> Self {
         Pueh(other)
    }
}

impl ::core::fmt::Display for Pueh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pueh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ptepe0() != 0 { try!(write!(f, " ptepe0"))}
        if self.ptepe1() != 0 { try!(write!(f, " ptepe1"))}
        if self.ptepe2() != 0 { try!(write!(f, " ptepe2"))}
        if self.ptepe3() != 0 { try!(write!(f, " ptepe3"))}
        if self.ptepe4() != 0 { try!(write!(f, " ptepe4"))}
        if self.ptepe5() != 0 { try!(write!(f, " ptepe5"))}
        if self.ptepe6() != 0 { try!(write!(f, " ptepe6"))}
        if self.ptepe7() != 0 { try!(write!(f, " ptepe7"))}
        if self.ptfpe0() != 0 { try!(write!(f, " ptfpe0"))}
        if self.ptfpe1() != 0 { try!(write!(f, " ptfpe1"))}
        if self.ptfpe2() != 0 { try!(write!(f, " ptfpe2"))}
        if self.ptfpe3() != 0 { try!(write!(f, " ptfpe3"))}
        if self.ptfpe4() != 0 { try!(write!(f, " ptfpe4"))}
        if self.ptfpe5() != 0 { try!(write!(f, " ptfpe5"))}
        if self.ptfpe6() != 0 { try!(write!(f, " ptfpe6"))}
        if self.ptfpe7() != 0 { try!(write!(f, " ptfpe7"))}
        if self.ptgpe0() != 0 { try!(write!(f, " ptgpe0"))}
        if self.ptgpe1() != 0 { try!(write!(f, " ptgpe1"))}
        if self.ptgpe2() != 0 { try!(write!(f, " ptgpe2"))}
        if self.ptgpe3() != 0 { try!(write!(f, " ptgpe3"))}
        if self.pthpe0() != 0 { try!(write!(f, " pthpe0"))}
        if self.pthpe1() != 0 { try!(write!(f, " pthpe1"))}
        if self.pthpe2() != 0 { try!(write!(f, " pthpe2"))}
        if self.pthpe6() != 0 { try!(write!(f, " pthpe6"))}
        if self.pthpe7() != 0 { try!(write!(f, " pthpe7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port High Drive Enable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Hdrve(pub u32);
impl Hdrve {
    #[doc="High Current Drive Capability of PTB4"]
    #[inline] pub fn ptb4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PTB4 != 0"]
    #[inline] pub fn test_ptb4(&self) -> bool {
        self.ptb4() != 0
    }

    #[doc="Sets the PTB4 field."]
    #[inline] pub fn set_ptb4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="High Current Drive Capability of PTB5"]
    #[inline] pub fn ptb5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PTB5 != 0"]
    #[inline] pub fn test_ptb5(&self) -> bool {
        self.ptb5() != 0
    }

    #[doc="Sets the PTB5 field."]
    #[inline] pub fn set_ptb5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="High Current Drive Capability of PTD0"]
    #[inline] pub fn ptd0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PTD0 != 0"]
    #[inline] pub fn test_ptd0(&self) -> bool {
        self.ptd0() != 0
    }

    #[doc="Sets the PTD0 field."]
    #[inline] pub fn set_ptd0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="High Current Drive Capability of PTD1"]
    #[inline] pub fn ptd1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PTD1 != 0"]
    #[inline] pub fn test_ptd1(&self) -> bool {
        self.ptd1() != 0
    }

    #[doc="Sets the PTD1 field."]
    #[inline] pub fn set_ptd1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="High Current Drive Capability of PTE0"]
    #[inline] pub fn pte0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PTE0 != 0"]
    #[inline] pub fn test_pte0(&self) -> bool {
        self.pte0() != 0
    }

    #[doc="Sets the PTE0 field."]
    #[inline] pub fn set_pte0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="High Current Drive Capability of PTE1"]
    #[inline] pub fn pte1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PTE1 != 0"]
    #[inline] pub fn test_pte1(&self) -> bool {
        self.pte1() != 0
    }

    #[doc="Sets the PTE1 field."]
    #[inline] pub fn set_pte1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="High Current Drive Capability of PTH0"]
    #[inline] pub fn pth0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PTH0 != 0"]
    #[inline] pub fn test_pth0(&self) -> bool {
        self.pth0() != 0
    }

    #[doc="Sets the PTH0 field."]
    #[inline] pub fn set_pth0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="High Current Drive Capability of PTH1"]
    #[inline] pub fn pth1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PTH1 != 0"]
    #[inline] pub fn test_pth1(&self) -> bool {
        self.pth1() != 0
    }

    #[doc="Sets the PTH1 field."]
    #[inline] pub fn set_pth1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Hdrve {
    #[inline]
    fn from(other: u32) -> Self {
         Hdrve(other)
    }
}

impl ::core::fmt::Display for Hdrve {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Hdrve {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ptb4() != 0 { try!(write!(f, " ptb4"))}
        if self.ptb5() != 0 { try!(write!(f, " ptb5"))}
        if self.ptd0() != 0 { try!(write!(f, " ptd0"))}
        if self.ptd1() != 0 { try!(write!(f, " ptd1"))}
        if self.pte0() != 0 { try!(write!(f, " pte0"))}
        if self.pte1() != 0 { try!(write!(f, " pte1"))}
        if self.pth0() != 0 { try!(write!(f, " pth0"))}
        if self.pth1() != 0 { try!(write!(f, " pth1"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

