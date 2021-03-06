
::bobbin_mcu::periph!( CEC, Cec, CEC_PERIPH, CecPeriph, CEC_OWNED, CEC_REF_COUNT, 0x40006c00, 0x00, 0x0b);


#[doc="HDMI-CEC controller"]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CecPeriph(pub usize);
impl CecPeriph {
    #[doc="Get the CR Register."]
    #[inline] pub fn cr_reg(&self) -> ::bobbin_mcu::register::Register<Cr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cr, 0x0)
    }

    #[doc="Get the *mut pointer for the CR register."]
    #[inline] pub fn cr_mut(&self) -> *mut Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CR register."]
    #[inline] pub fn cr_ptr(&self) -> *const Cr { 
        self.cr_reg().ptr()
    }

    #[doc="Read the CR register."]
    #[inline] pub fn cr(&self) -> Cr { 
        self.cr_reg().read()
    }

    #[doc="Write the CR register."]
    #[inline] pub fn write_cr(&self, value: Cr) -> &Self { 
        self.cr_reg().write(value);
        self
    }

    #[doc="Set the CR register."]
    #[inline] pub fn set_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().set(f);
        self
    }

    #[doc="Modify the CR register."]
    #[inline] pub fn with_cr<F: FnOnce(Cr) -> Cr>(&self, f: F) -> &Self {
        self.cr_reg().with(f);
        self
    }

    #[doc="Get the CFGR Register."]
    #[inline] pub fn cfgr_reg(&self) -> ::bobbin_mcu::register::Register<Cfgr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cfgr, 0x4)
    }

    #[doc="Get the *mut pointer for the CFGR register."]
    #[inline] pub fn cfgr_mut(&self) -> *mut Cfgr { 
        self.cfgr_reg().ptr()
    }

    #[doc="Get the *const pointer for the CFGR register."]
    #[inline] pub fn cfgr_ptr(&self) -> *const Cfgr { 
        self.cfgr_reg().ptr()
    }

    #[doc="Read the CFGR register."]
    #[inline] pub fn cfgr(&self) -> Cfgr { 
        self.cfgr_reg().read()
    }

    #[doc="Write the CFGR register."]
    #[inline] pub fn write_cfgr(&self, value: Cfgr) -> &Self { 
        self.cfgr_reg().write(value);
        self
    }

    #[doc="Set the CFGR register."]
    #[inline] pub fn set_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        self.cfgr_reg().set(f);
        self
    }

    #[doc="Modify the CFGR register."]
    #[inline] pub fn with_cfgr<F: FnOnce(Cfgr) -> Cfgr>(&self, f: F) -> &Self {
        self.cfgr_reg().with(f);
        self
    }

    #[doc="Get the TXDR Register."]
    #[inline] pub fn txdr_reg(&self) -> ::bobbin_mcu::register::Register<Txdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Txdr, 0x8)
    }

    #[doc="Get the *mut pointer for the TXDR register."]
    #[inline] pub fn txdr_mut(&self) -> *mut Txdr { 
        self.txdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the TXDR register."]
    #[inline] pub fn txdr_ptr(&self) -> *const Txdr { 
        self.txdr_reg().ptr()
    }

    #[doc="Write the TXDR register."]
    #[inline] pub fn write_txdr(&self, value: Txdr) -> &Self { 
        self.txdr_reg().write(value);
        self
    }

    #[doc="Set the TXDR register."]
    #[inline] pub fn set_txdr<F: FnOnce(Txdr) -> Txdr>(&self, f: F) -> &Self {
        self.txdr_reg().set(f);
        self
    }

    #[doc="Get the RXDR Register."]
    #[inline] pub fn rxdr_reg(&self) -> ::bobbin_mcu::register::Register<Rxdr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Rxdr, 0xc)
    }

    #[doc="Get the *mut pointer for the RXDR register."]
    #[inline] pub fn rxdr_mut(&self) -> *mut Rxdr { 
        self.rxdr_reg().ptr()
    }

    #[doc="Get the *const pointer for the RXDR register."]
    #[inline] pub fn rxdr_ptr(&self) -> *const Rxdr { 
        self.rxdr_reg().ptr()
    }

    #[doc="Read the RXDR register."]
    #[inline] pub fn rxdr(&self) -> Rxdr { 
        self.rxdr_reg().read()
    }

    #[doc="Get the ISR Register."]
    #[inline] pub fn isr_reg(&self) -> ::bobbin_mcu::register::Register<Isr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Isr, 0x10)
    }

    #[doc="Get the *mut pointer for the ISR register."]
    #[inline] pub fn isr_mut(&self) -> *mut Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Get the *const pointer for the ISR register."]
    #[inline] pub fn isr_ptr(&self) -> *const Isr { 
        self.isr_reg().ptr()
    }

    #[doc="Read the ISR register."]
    #[inline] pub fn isr(&self) -> Isr { 
        self.isr_reg().read()
    }

    #[doc="Write the ISR register."]
    #[inline] pub fn write_isr(&self, value: Isr) -> &Self { 
        self.isr_reg().write(value);
        self
    }

    #[doc="Set the ISR register."]
    #[inline] pub fn set_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
        self.isr_reg().set(f);
        self
    }

    #[doc="Modify the ISR register."]
    #[inline] pub fn with_isr<F: FnOnce(Isr) -> Isr>(&self, f: F) -> &Self {
        self.isr_reg().with(f);
        self
    }

    #[doc="Get the IER Register."]
    #[inline] pub fn ier_reg(&self) -> ::bobbin_mcu::register::Register<Ier> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ier, 0x14)
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        self.ier_reg().ptr()
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const Ier { 
        self.ier_reg().ptr()
    }

    #[doc="Read the IER register."]
    #[inline] pub fn ier(&self) -> Ier { 
        self.ier_reg().read()
    }

    #[doc="Write the IER register."]
    #[inline] pub fn write_ier(&self, value: Ier) -> &Self { 
        self.ier_reg().write(value);
        self
    }

    #[doc="Set the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        self.ier_reg().set(f);
        self
    }

    #[doc="Modify the IER register."]
    #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        self.ier_reg().with(f);
        self
    }

}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="Tx End Of Message"]
    #[inline] pub fn txeom(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TXEOM != 0"]
    #[inline] pub fn test_txeom(&self) -> bool {
        self.txeom() != 0
    }

    #[doc="Sets the TXEOM field."]
    #[inline] pub fn set_txeom<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Tx start of message"]
    #[inline] pub fn txsom(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXSOM != 0"]
    #[inline] pub fn test_txsom(&self) -> bool {
        self.txsom() != 0
    }

    #[doc="Sets the TXSOM field."]
    #[inline] pub fn set_txsom<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="CEC Enable"]
    #[inline] pub fn cecen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CECEN != 0"]
    #[inline] pub fn test_cecen(&self) -> bool {
        self.cecen() != 0
    }

    #[doc="Sets the CECEN field."]
    #[inline] pub fn set_cecen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cr {
    #[inline]
    fn from(other: u32) -> Self {
         Cr(other)
    }
}

impl ::core::fmt::Display for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txeom() != 0 { try!(write!(f, " txeom"))}
        if self.txsom() != 0 { try!(write!(f, " txsom"))}
        if self.cecen() != 0 { try!(write!(f, " cecen"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="configuration register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cfgr(pub u32);
impl Cfgr {
    #[doc="Signal Free Time"]
    #[inline] pub fn sft(&self) -> ::bobbin_bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x7) as u8) } // [2:0]
    }

    #[doc="Returns true if SFT != 0"]
    #[inline] pub fn test_sft(&self) -> bool {
        self.sft() != 0
    }

    #[doc="Sets the SFT field."]
    #[inline] pub fn set_sft<V: Into<::bobbin_bits::U3>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Rx-Tolerance"]
    #[inline] pub fn rxtol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RXTOL != 0"]
    #[inline] pub fn test_rxtol(&self) -> bool {
        self.rxtol() != 0
    }

    #[doc="Sets the RXTOL field."]
    #[inline] pub fn set_rxtol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Rx-stop on bit rising error"]
    #[inline] pub fn brestp(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if BRESTP != 0"]
    #[inline] pub fn test_brestp(&self) -> bool {
        self.brestp() != 0
    }

    #[doc="Sets the BRESTP field."]
    #[inline] pub fn set_brestp<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Generate error-bit on bit rising error"]
    #[inline] pub fn bregen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if BREGEN != 0"]
    #[inline] pub fn test_bregen(&self) -> bool {
        self.bregen() != 0
    }

    #[doc="Sets the BREGEN field."]
    #[inline] pub fn set_bregen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Generate Error-Bit on Long Bit Period Error"]
    #[inline] pub fn lbpegen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if LBPEGEN != 0"]
    #[inline] pub fn test_lbpegen(&self) -> bool {
        self.lbpegen() != 0
    }

    #[doc="Sets the LBPEGEN field."]
    #[inline] pub fn set_lbpegen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Avoid Error-Bit Generation in Broadcast"]
    #[inline] pub fn brdnogen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if BRDNOGEN != 0"]
    #[inline] pub fn test_brdnogen(&self) -> bool {
        self.brdnogen() != 0
    }

    #[doc="Sets the BRDNOGEN field."]
    #[inline] pub fn set_brdnogen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="SFT Option Bit"]
    #[inline] pub fn sftop(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SFTOP != 0"]
    #[inline] pub fn test_sftop(&self) -> bool {
        self.sftop() != 0
    }

    #[doc="Sets the SFTOP field."]
    #[inline] pub fn set_sftop<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Own addresses configuration"]
    #[inline] pub fn oar(&self) -> ::bobbin_bits::U15 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x7fff) as u16) } // [30:16]
    }

    #[doc="Returns true if OAR != 0"]
    #[inline] pub fn test_oar(&self) -> bool {
        self.oar() != 0
    }

    #[doc="Sets the OAR field."]
    #[inline] pub fn set_oar<V: Into<::bobbin_bits::U15>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U15 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7fff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Listen mode"]
    #[inline] pub fn lstn(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LSTN != 0"]
    #[inline] pub fn test_lstn(&self) -> bool {
        self.lstn() != 0
    }

    #[doc="Sets the LSTN field."]
    #[inline] pub fn set_lstn<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

}

impl From<u32> for Cfgr {
    #[inline]
    fn from(other: u32) -> Self {
         Cfgr(other)
    }
}

impl ::core::fmt::Display for Cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cfgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.sft() != 0 { try!(write!(f, " sft=0x{:x}", self.sft()))}
        if self.rxtol() != 0 { try!(write!(f, " rxtol"))}
        if self.brestp() != 0 { try!(write!(f, " brestp"))}
        if self.bregen() != 0 { try!(write!(f, " bregen"))}
        if self.lbpegen() != 0 { try!(write!(f, " lbpegen"))}
        if self.brdnogen() != 0 { try!(write!(f, " brdnogen"))}
        if self.sftop() != 0 { try!(write!(f, " sftop"))}
        if self.oar() != 0 { try!(write!(f, " oar=0x{:x}", self.oar()))}
        if self.lstn() != 0 { try!(write!(f, " lstn"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Tx data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Txdr(pub u32);
impl Txdr {
    #[doc="Tx Data register"]
    #[inline] pub fn txd(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if TXD != 0"]
    #[inline] pub fn test_txd(&self) -> bool {
        self.txd() != 0
    }

    #[doc="Sets the TXD field."]
    #[inline] pub fn set_txd<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Txdr {
    #[inline]
    fn from(other: u32) -> Self {
         Txdr(other)
    }
}

impl ::core::fmt::Display for Txdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Txdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txd() != 0 { try!(write!(f, " txd=0x{:x}", self.txd()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Rx Data Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rxdr(pub u32);
impl Rxdr {
    #[doc="CEC Rx Data Register"]
    #[inline] pub fn rxdr(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if RXDR != 0"]
    #[inline] pub fn test_rxdr(&self) -> bool {
        self.rxdr() != 0
    }

    #[doc="Sets the RXDR field."]
    #[inline] pub fn set_rxdr<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rxdr {
    #[inline]
    fn from(other: u32) -> Self {
         Rxdr(other)
    }
}

impl ::core::fmt::Display for Rxdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rxdr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rxdr() != 0 { try!(write!(f, " rxdr=0x{:x}", self.rxdr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Interrupt and Status Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc="Tx-Missing acknowledge error"]
    #[inline] pub fn txacke(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TXACKE != 0"]
    #[inline] pub fn test_txacke(&self) -> bool {
        self.txacke() != 0
    }

    #[doc="Sets the TXACKE field."]
    #[inline] pub fn set_txacke<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Tx-Error"]
    #[inline] pub fn txerr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TXERR != 0"]
    #[inline] pub fn test_txerr(&self) -> bool {
        self.txerr() != 0
    }

    #[doc="Sets the TXERR field."]
    #[inline] pub fn set_txerr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Tx-Buffer Underrun"]
    #[inline] pub fn txudr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TXUDR != 0"]
    #[inline] pub fn test_txudr(&self) -> bool {
        self.txudr() != 0
    }

    #[doc="Sets the TXUDR field."]
    #[inline] pub fn set_txudr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="End of Transmission"]
    #[inline] pub fn txend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TXEND != 0"]
    #[inline] pub fn test_txend(&self) -> bool {
        self.txend() != 0
    }

    #[doc="Sets the TXEND field."]
    #[inline] pub fn set_txend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Tx-Byte Request"]
    #[inline] pub fn txbr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXBR != 0"]
    #[inline] pub fn test_txbr(&self) -> bool {
        self.txbr() != 0
    }

    #[doc="Sets the TXBR field."]
    #[inline] pub fn set_txbr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Arbitration Lost"]
    #[inline] pub fn arblst(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ARBLST != 0"]
    #[inline] pub fn test_arblst(&self) -> bool {
        self.arblst() != 0
    }

    #[doc="Sets the ARBLST field."]
    #[inline] pub fn set_arblst<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Rx-Missing Acknowledge"]
    #[inline] pub fn rxacke(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXACKE != 0"]
    #[inline] pub fn test_rxacke(&self) -> bool {
        self.rxacke() != 0
    }

    #[doc="Sets the RXACKE field."]
    #[inline] pub fn set_rxacke<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Rx-Long Bit Period Error"]
    #[inline] pub fn lbpe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LBPE != 0"]
    #[inline] pub fn test_lbpe(&self) -> bool {
        self.lbpe() != 0
    }

    #[doc="Sets the LBPE field."]
    #[inline] pub fn set_lbpe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Rx-Short Bit period error"]
    #[inline] pub fn sbpe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SBPE != 0"]
    #[inline] pub fn test_sbpe(&self) -> bool {
        self.sbpe() != 0
    }

    #[doc="Sets the SBPE field."]
    #[inline] pub fn set_sbpe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Rx-Bit rising error"]
    #[inline] pub fn bre(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BRE != 0"]
    #[inline] pub fn test_bre(&self) -> bool {
        self.bre() != 0
    }

    #[doc="Sets the BRE field."]
    #[inline] pub fn set_bre<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Rx-Overrun"]
    #[inline] pub fn rxovr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXOVR != 0"]
    #[inline] pub fn test_rxovr(&self) -> bool {
        self.rxovr() != 0
    }

    #[doc="Sets the RXOVR field."]
    #[inline] pub fn set_rxovr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="End Of Reception"]
    #[inline] pub fn rxend(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RXEND != 0"]
    #[inline] pub fn test_rxend(&self) -> bool {
        self.rxend() != 0
    }

    #[doc="Sets the RXEND field."]
    #[inline] pub fn set_rxend<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Rx-Byte Received"]
    #[inline] pub fn rxbr(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXBR != 0"]
    #[inline] pub fn test_rxbr(&self) -> bool {
        self.rxbr() != 0
    }

    #[doc="Sets the RXBR field."]
    #[inline] pub fn set_rxbr<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Isr {
    #[inline]
    fn from(other: u32) -> Self {
         Isr(other)
    }
}

impl ::core::fmt::Display for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txacke() != 0 { try!(write!(f, " txacke"))}
        if self.txerr() != 0 { try!(write!(f, " txerr"))}
        if self.txudr() != 0 { try!(write!(f, " txudr"))}
        if self.txend() != 0 { try!(write!(f, " txend"))}
        if self.txbr() != 0 { try!(write!(f, " txbr"))}
        if self.arblst() != 0 { try!(write!(f, " arblst"))}
        if self.rxacke() != 0 { try!(write!(f, " rxacke"))}
        if self.lbpe() != 0 { try!(write!(f, " lbpe"))}
        if self.sbpe() != 0 { try!(write!(f, " sbpe"))}
        if self.bre() != 0 { try!(write!(f, " bre"))}
        if self.rxovr() != 0 { try!(write!(f, " rxovr"))}
        if self.rxend() != 0 { try!(write!(f, " rxend"))}
        if self.rxbr() != 0 { try!(write!(f, " rxbr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="Tx-Missing Acknowledge Error Interrupt Enable"]
    #[inline] pub fn txackie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if TXACKIE != 0"]
    #[inline] pub fn test_txackie(&self) -> bool {
        self.txackie() != 0
    }

    #[doc="Sets the TXACKIE field."]
    #[inline] pub fn set_txackie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Tx-Error Interrupt Enable"]
    #[inline] pub fn txerrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TXERRIE != 0"]
    #[inline] pub fn test_txerrie(&self) -> bool {
        self.txerrie() != 0
    }

    #[doc="Sets the TXERRIE field."]
    #[inline] pub fn set_txerrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Tx-Underrun interrupt enable"]
    #[inline] pub fn txudrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if TXUDRIE != 0"]
    #[inline] pub fn test_txudrie(&self) -> bool {
        self.txudrie() != 0
    }

    #[doc="Sets the TXUDRIE field."]
    #[inline] pub fn set_txudrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Tx-End of message interrupt enable"]
    #[inline] pub fn txendie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TXENDIE != 0"]
    #[inline] pub fn test_txendie(&self) -> bool {
        self.txendie() != 0
    }

    #[doc="Sets the TXENDIE field."]
    #[inline] pub fn set_txendie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Tx-Byte Request Interrupt Enable"]
    #[inline] pub fn txbrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXBRIE != 0"]
    #[inline] pub fn test_txbrie(&self) -> bool {
        self.txbrie() != 0
    }

    #[doc="Sets the TXBRIE field."]
    #[inline] pub fn set_txbrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Arbitration Lost Interrupt Enable"]
    #[inline] pub fn arblstie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ARBLSTIE != 0"]
    #[inline] pub fn test_arblstie(&self) -> bool {
        self.arblstie() != 0
    }

    #[doc="Sets the ARBLSTIE field."]
    #[inline] pub fn set_arblstie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Rx-Missing Acknowledge Error Interrupt Enable"]
    #[inline] pub fn rxackie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if RXACKIE != 0"]
    #[inline] pub fn test_rxackie(&self) -> bool {
        self.rxackie() != 0
    }

    #[doc="Sets the RXACKIE field."]
    #[inline] pub fn set_rxackie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Long Bit Period Error Interrupt Enable"]
    #[inline] pub fn lbpeie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if LBPEIE != 0"]
    #[inline] pub fn test_lbpeie(&self) -> bool {
        self.lbpeie() != 0
    }

    #[doc="Sets the LBPEIE field."]
    #[inline] pub fn set_lbpeie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Short Bit Period Error Interrupt Enable"]
    #[inline] pub fn sbpeie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SBPEIE != 0"]
    #[inline] pub fn test_sbpeie(&self) -> bool {
        self.sbpeie() != 0
    }

    #[doc="Sets the SBPEIE field."]
    #[inline] pub fn set_sbpeie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Bit Rising Error Interrupt Enable"]
    #[inline] pub fn breie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if BREIE != 0"]
    #[inline] pub fn test_breie(&self) -> bool {
        self.breie() != 0
    }

    #[doc="Sets the BREIE field."]
    #[inline] pub fn set_breie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Rx-Buffer Overrun Interrupt Enable"]
    #[inline] pub fn rxovrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if RXOVRIE != 0"]
    #[inline] pub fn test_rxovrie(&self) -> bool {
        self.rxovrie() != 0
    }

    #[doc="Sets the RXOVRIE field."]
    #[inline] pub fn set_rxovrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="End Of Reception Interrupt Enable"]
    #[inline] pub fn rxendie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RXENDIE != 0"]
    #[inline] pub fn test_rxendie(&self) -> bool {
        self.rxendie() != 0
    }

    #[doc="Sets the RXENDIE field."]
    #[inline] pub fn set_rxendie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Rx-Byte Received Interrupt Enable"]
    #[inline] pub fn rxbrie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RXBRIE != 0"]
    #[inline] pub fn test_rxbrie(&self) -> bool {
        self.rxbrie() != 0
    }

    #[doc="Sets the RXBRIE field."]
    #[inline] pub fn set_rxbrie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Ier {
    #[inline]
    fn from(other: u32) -> Self {
         Ier(other)
    }
}

impl ::core::fmt::Display for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.txackie() != 0 { try!(write!(f, " txackie"))}
        if self.txerrie() != 0 { try!(write!(f, " txerrie"))}
        if self.txudrie() != 0 { try!(write!(f, " txudrie"))}
        if self.txendie() != 0 { try!(write!(f, " txendie"))}
        if self.txbrie() != 0 { try!(write!(f, " txbrie"))}
        if self.arblstie() != 0 { try!(write!(f, " arblstie"))}
        if self.rxackie() != 0 { try!(write!(f, " rxackie"))}
        if self.lbpeie() != 0 { try!(write!(f, " lbpeie"))}
        if self.sbpeie() != 0 { try!(write!(f, " sbpeie"))}
        if self.breie() != 0 { try!(write!(f, " breie"))}
        if self.rxovrie() != 0 { try!(write!(f, " rxovrie"))}
        if self.rxendie() != 0 { try!(write!(f, " rxendie"))}
        if self.rxbrie() != 0 { try!(write!(f, " rxbrie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

