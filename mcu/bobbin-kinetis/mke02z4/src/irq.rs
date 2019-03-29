::bobbin_mcu::periph!( IRQ, Irq, IRQ_PERIPH, IrqPeriph, IRQ_OWNED, IRQ_REF_COUNT, 0x40031000, 0x00, 0x01);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="IRQ Peripheral"]
pub struct IrqPeriph(pub usize); 

impl IrqPeriph {
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

}

#[doc="Interrupt Pin Request Status and Control Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sc(pub u8);
impl Sc {
    #[doc="IRQ Detection Mode"]
    #[inline] pub fn irqmod(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if IRQMOD != 0"]
    #[inline] pub fn test_irqmod(&self) -> bool {
        self.irqmod() != 0
    }

    #[doc="Sets the IRQMOD field."]
    #[inline] pub fn set_irqmod<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="IRQ Interrupt Enable"]
    #[inline] pub fn irqie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if IRQIE != 0"]
    #[inline] pub fn test_irqie(&self) -> bool {
        self.irqie() != 0
    }

    #[doc="Sets the IRQIE field."]
    #[inline] pub fn set_irqie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="IRQ Acknowledge"]
    #[inline] pub fn irqack(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IRQACK != 0"]
    #[inline] pub fn test_irqack(&self) -> bool {
        self.irqack() != 0
    }

    #[doc="Sets the IRQACK field."]
    #[inline] pub fn set_irqack<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="IRQ Flag"]
    #[inline] pub fn irqf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if IRQF != 0"]
    #[inline] pub fn test_irqf(&self) -> bool {
        self.irqf() != 0
    }

    #[doc="Sets the IRQF field."]
    #[inline] pub fn set_irqf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IRQ Pin Enable"]
    #[inline] pub fn irqpe(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if IRQPE != 0"]
    #[inline] pub fn test_irqpe(&self) -> bool {
        self.irqpe() != 0
    }

    #[doc="Sets the IRQPE field."]
    #[inline] pub fn set_irqpe<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Interrupt Request (IRQ) Edge Select"]
    #[inline] pub fn irqedg(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if IRQEDG != 0"]
    #[inline] pub fn test_irqedg(&self) -> bool {
        self.irqedg() != 0
    }

    #[doc="Sets the IRQEDG field."]
    #[inline] pub fn set_irqedg<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Interrupt Request (IRQ) Pull Device Disable"]
    #[inline] pub fn irqpdd(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if IRQPDD != 0"]
    #[inline] pub fn test_irqpdd(&self) -> bool {
        self.irqpdd() != 0
    }

    #[doc="Sets the IRQPDD field."]
    #[inline] pub fn set_irqpdd<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Sc {
    #[inline]
    fn from(other: u8) -> Self {
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
        if self.irqmod() != 0 { try!(write!(f, " irqmod"))}
        if self.irqie() != 0 { try!(write!(f, " irqie"))}
        if self.irqack() != 0 { try!(write!(f, " irqack"))}
        if self.irqf() != 0 { try!(write!(f, " irqf"))}
        if self.irqpe() != 0 { try!(write!(f, " irqpe"))}
        if self.irqedg() != 0 { try!(write!(f, " irqedg"))}
        if self.irqpdd() != 0 { try!(write!(f, " irqpdd"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

