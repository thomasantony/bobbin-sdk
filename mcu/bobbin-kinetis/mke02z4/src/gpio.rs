::bobbin_mcu::periph!( GPIOA, Gpioa, GPIOA_PERIPH, GpioPeriph, GPIOA_OWNED, GPIOA_REF_COUNT, 0x400ff000, 0x00, 0x19);
::bobbin_mcu::periph!( GPIOB, Gpiob, GPIOB_PERIPH, GpioPeriph, GPIOB_OWNED, GPIOB_REF_COUNT, 0x400ff040, 0x01, 0x1a);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="GPIO Peripheral"]
pub struct GpioPeriph(pub usize); 

impl GpioPeriph {
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
    #[inline] pub fn pdo0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PDO0 != 0"]
    #[inline] pub fn test_pdo0(&self) -> bool {
        self.pdo0() != 0
    }

    #[doc="Sets the PDO0 field."]
    #[inline] pub fn set_pdo0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PDO1 != 0"]
    #[inline] pub fn test_pdo1(&self) -> bool {
        self.pdo1() != 0
    }

    #[doc="Sets the PDO1 field."]
    #[inline] pub fn set_pdo1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PDO2 != 0"]
    #[inline] pub fn test_pdo2(&self) -> bool {
        self.pdo2() != 0
    }

    #[doc="Sets the PDO2 field."]
    #[inline] pub fn set_pdo2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PDO3 != 0"]
    #[inline] pub fn test_pdo3(&self) -> bool {
        self.pdo3() != 0
    }

    #[doc="Sets the PDO3 field."]
    #[inline] pub fn set_pdo3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PDO4 != 0"]
    #[inline] pub fn test_pdo4(&self) -> bool {
        self.pdo4() != 0
    }

    #[doc="Sets the PDO4 field."]
    #[inline] pub fn set_pdo4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PDO5 != 0"]
    #[inline] pub fn test_pdo5(&self) -> bool {
        self.pdo5() != 0
    }

    #[doc="Sets the PDO5 field."]
    #[inline] pub fn set_pdo5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PDO6 != 0"]
    #[inline] pub fn test_pdo6(&self) -> bool {
        self.pdo6() != 0
    }

    #[doc="Sets the PDO6 field."]
    #[inline] pub fn set_pdo6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PDO7 != 0"]
    #[inline] pub fn test_pdo7(&self) -> bool {
        self.pdo7() != 0
    }

    #[doc="Sets the PDO7 field."]
    #[inline] pub fn set_pdo7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PDO8 != 0"]
    #[inline] pub fn test_pdo8(&self) -> bool {
        self.pdo8() != 0
    }

    #[doc="Sets the PDO8 field."]
    #[inline] pub fn set_pdo8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PDO9 != 0"]
    #[inline] pub fn test_pdo9(&self) -> bool {
        self.pdo9() != 0
    }

    #[doc="Sets the PDO9 field."]
    #[inline] pub fn set_pdo9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PDO10 != 0"]
    #[inline] pub fn test_pdo10(&self) -> bool {
        self.pdo10() != 0
    }

    #[doc="Sets the PDO10 field."]
    #[inline] pub fn set_pdo10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PDO11 != 0"]
    #[inline] pub fn test_pdo11(&self) -> bool {
        self.pdo11() != 0
    }

    #[doc="Sets the PDO11 field."]
    #[inline] pub fn set_pdo11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PDO12 != 0"]
    #[inline] pub fn test_pdo12(&self) -> bool {
        self.pdo12() != 0
    }

    #[doc="Sets the PDO12 field."]
    #[inline] pub fn set_pdo12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PDO13 != 0"]
    #[inline] pub fn test_pdo13(&self) -> bool {
        self.pdo13() != 0
    }

    #[doc="Sets the PDO13 field."]
    #[inline] pub fn set_pdo13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PDO14 != 0"]
    #[inline] pub fn test_pdo14(&self) -> bool {
        self.pdo14() != 0
    }

    #[doc="Sets the PDO14 field."]
    #[inline] pub fn set_pdo14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PDO15 != 0"]
    #[inline] pub fn test_pdo15(&self) -> bool {
        self.pdo15() != 0
    }

    #[doc="Sets the PDO15 field."]
    #[inline] pub fn set_pdo15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo16(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PDO16 != 0"]
    #[inline] pub fn test_pdo16(&self) -> bool {
        self.pdo16() != 0
    }

    #[doc="Sets the PDO16 field."]
    #[inline] pub fn set_pdo16<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo17(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PDO17 != 0"]
    #[inline] pub fn test_pdo17(&self) -> bool {
        self.pdo17() != 0
    }

    #[doc="Sets the PDO17 field."]
    #[inline] pub fn set_pdo17<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo18(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PDO18 != 0"]
    #[inline] pub fn test_pdo18(&self) -> bool {
        self.pdo18() != 0
    }

    #[doc="Sets the PDO18 field."]
    #[inline] pub fn set_pdo18<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo19(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PDO19 != 0"]
    #[inline] pub fn test_pdo19(&self) -> bool {
        self.pdo19() != 0
    }

    #[doc="Sets the PDO19 field."]
    #[inline] pub fn set_pdo19<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo20(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PDO20 != 0"]
    #[inline] pub fn test_pdo20(&self) -> bool {
        self.pdo20() != 0
    }

    #[doc="Sets the PDO20 field."]
    #[inline] pub fn set_pdo20<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo21(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PDO21 != 0"]
    #[inline] pub fn test_pdo21(&self) -> bool {
        self.pdo21() != 0
    }

    #[doc="Sets the PDO21 field."]
    #[inline] pub fn set_pdo21<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo22(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PDO22 != 0"]
    #[inline] pub fn test_pdo22(&self) -> bool {
        self.pdo22() != 0
    }

    #[doc="Sets the PDO22 field."]
    #[inline] pub fn set_pdo22<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo23(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PDO23 != 0"]
    #[inline] pub fn test_pdo23(&self) -> bool {
        self.pdo23() != 0
    }

    #[doc="Sets the PDO23 field."]
    #[inline] pub fn set_pdo23<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo24(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PDO24 != 0"]
    #[inline] pub fn test_pdo24(&self) -> bool {
        self.pdo24() != 0
    }

    #[doc="Sets the PDO24 field."]
    #[inline] pub fn set_pdo24<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo25(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PDO25 != 0"]
    #[inline] pub fn test_pdo25(&self) -> bool {
        self.pdo25() != 0
    }

    #[doc="Sets the PDO25 field."]
    #[inline] pub fn set_pdo25<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo26(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PDO26 != 0"]
    #[inline] pub fn test_pdo26(&self) -> bool {
        self.pdo26() != 0
    }

    #[doc="Sets the PDO26 field."]
    #[inline] pub fn set_pdo26<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo27(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PDO27 != 0"]
    #[inline] pub fn test_pdo27(&self) -> bool {
        self.pdo27() != 0
    }

    #[doc="Sets the PDO27 field."]
    #[inline] pub fn set_pdo27<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo28(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PDO28 != 0"]
    #[inline] pub fn test_pdo28(&self) -> bool {
        self.pdo28() != 0
    }

    #[doc="Sets the PDO28 field."]
    #[inline] pub fn set_pdo28<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo29(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if PDO29 != 0"]
    #[inline] pub fn test_pdo29(&self) -> bool {
        self.pdo29() != 0
    }

    #[doc="Sets the PDO29 field."]
    #[inline] pub fn set_pdo29<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo30(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if PDO30 != 0"]
    #[inline] pub fn test_pdo30(&self) -> bool {
        self.pdo30() != 0
    }

    #[doc="Sets the PDO30 field."]
    #[inline] pub fn set_pdo30<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Port Data Output"]
    #[inline] pub fn pdo31(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PDO31 != 0"]
    #[inline] pub fn test_pdo31(&self) -> bool {
        self.pdo31() != 0
    }

    #[doc="Sets the PDO31 field."]
    #[inline] pub fn set_pdo31<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.pdo0() != 0 { try!(write!(f, " pdo0"))}
        if self.pdo1() != 0 { try!(write!(f, " pdo1"))}
        if self.pdo2() != 0 { try!(write!(f, " pdo2"))}
        if self.pdo3() != 0 { try!(write!(f, " pdo3"))}
        if self.pdo4() != 0 { try!(write!(f, " pdo4"))}
        if self.pdo5() != 0 { try!(write!(f, " pdo5"))}
        if self.pdo6() != 0 { try!(write!(f, " pdo6"))}
        if self.pdo7() != 0 { try!(write!(f, " pdo7"))}
        if self.pdo8() != 0 { try!(write!(f, " pdo8"))}
        if self.pdo9() != 0 { try!(write!(f, " pdo9"))}
        if self.pdo10() != 0 { try!(write!(f, " pdo10"))}
        if self.pdo11() != 0 { try!(write!(f, " pdo11"))}
        if self.pdo12() != 0 { try!(write!(f, " pdo12"))}
        if self.pdo13() != 0 { try!(write!(f, " pdo13"))}
        if self.pdo14() != 0 { try!(write!(f, " pdo14"))}
        if self.pdo15() != 0 { try!(write!(f, " pdo15"))}
        if self.pdo16() != 0 { try!(write!(f, " pdo16"))}
        if self.pdo17() != 0 { try!(write!(f, " pdo17"))}
        if self.pdo18() != 0 { try!(write!(f, " pdo18"))}
        if self.pdo19() != 0 { try!(write!(f, " pdo19"))}
        if self.pdo20() != 0 { try!(write!(f, " pdo20"))}
        if self.pdo21() != 0 { try!(write!(f, " pdo21"))}
        if self.pdo22() != 0 { try!(write!(f, " pdo22"))}
        if self.pdo23() != 0 { try!(write!(f, " pdo23"))}
        if self.pdo24() != 0 { try!(write!(f, " pdo24"))}
        if self.pdo25() != 0 { try!(write!(f, " pdo25"))}
        if self.pdo26() != 0 { try!(write!(f, " pdo26"))}
        if self.pdo27() != 0 { try!(write!(f, " pdo27"))}
        if self.pdo28() != 0 { try!(write!(f, " pdo28"))}
        if self.pdo29() != 0 { try!(write!(f, " pdo29"))}
        if self.pdo30() != 0 { try!(write!(f, " pdo30"))}
        if self.pdo31() != 0 { try!(write!(f, " pdo31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Set Output Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Psor(pub u32);
impl Psor {
    #[doc="Port Set Output"]
    #[inline] pub fn ptso0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PTSO0 != 0"]
    #[inline] pub fn test_ptso0(&self) -> bool {
        self.ptso0() != 0
    }

    #[doc="Sets the PTSO0 field."]
    #[inline] pub fn set_ptso0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PTSO1 != 0"]
    #[inline] pub fn test_ptso1(&self) -> bool {
        self.ptso1() != 0
    }

    #[doc="Sets the PTSO1 field."]
    #[inline] pub fn set_ptso1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PTSO2 != 0"]
    #[inline] pub fn test_ptso2(&self) -> bool {
        self.ptso2() != 0
    }

    #[doc="Sets the PTSO2 field."]
    #[inline] pub fn set_ptso2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PTSO3 != 0"]
    #[inline] pub fn test_ptso3(&self) -> bool {
        self.ptso3() != 0
    }

    #[doc="Sets the PTSO3 field."]
    #[inline] pub fn set_ptso3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PTSO4 != 0"]
    #[inline] pub fn test_ptso4(&self) -> bool {
        self.ptso4() != 0
    }

    #[doc="Sets the PTSO4 field."]
    #[inline] pub fn set_ptso4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PTSO5 != 0"]
    #[inline] pub fn test_ptso5(&self) -> bool {
        self.ptso5() != 0
    }

    #[doc="Sets the PTSO5 field."]
    #[inline] pub fn set_ptso5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PTSO6 != 0"]
    #[inline] pub fn test_ptso6(&self) -> bool {
        self.ptso6() != 0
    }

    #[doc="Sets the PTSO6 field."]
    #[inline] pub fn set_ptso6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PTSO7 != 0"]
    #[inline] pub fn test_ptso7(&self) -> bool {
        self.ptso7() != 0
    }

    #[doc="Sets the PTSO7 field."]
    #[inline] pub fn set_ptso7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PTSO8 != 0"]
    #[inline] pub fn test_ptso8(&self) -> bool {
        self.ptso8() != 0
    }

    #[doc="Sets the PTSO8 field."]
    #[inline] pub fn set_ptso8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PTSO9 != 0"]
    #[inline] pub fn test_ptso9(&self) -> bool {
        self.ptso9() != 0
    }

    #[doc="Sets the PTSO9 field."]
    #[inline] pub fn set_ptso9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PTSO10 != 0"]
    #[inline] pub fn test_ptso10(&self) -> bool {
        self.ptso10() != 0
    }

    #[doc="Sets the PTSO10 field."]
    #[inline] pub fn set_ptso10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PTSO11 != 0"]
    #[inline] pub fn test_ptso11(&self) -> bool {
        self.ptso11() != 0
    }

    #[doc="Sets the PTSO11 field."]
    #[inline] pub fn set_ptso11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PTSO12 != 0"]
    #[inline] pub fn test_ptso12(&self) -> bool {
        self.ptso12() != 0
    }

    #[doc="Sets the PTSO12 field."]
    #[inline] pub fn set_ptso12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PTSO13 != 0"]
    #[inline] pub fn test_ptso13(&self) -> bool {
        self.ptso13() != 0
    }

    #[doc="Sets the PTSO13 field."]
    #[inline] pub fn set_ptso13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PTSO14 != 0"]
    #[inline] pub fn test_ptso14(&self) -> bool {
        self.ptso14() != 0
    }

    #[doc="Sets the PTSO14 field."]
    #[inline] pub fn set_ptso14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PTSO15 != 0"]
    #[inline] pub fn test_ptso15(&self) -> bool {
        self.ptso15() != 0
    }

    #[doc="Sets the PTSO15 field."]
    #[inline] pub fn set_ptso15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso16(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PTSO16 != 0"]
    #[inline] pub fn test_ptso16(&self) -> bool {
        self.ptso16() != 0
    }

    #[doc="Sets the PTSO16 field."]
    #[inline] pub fn set_ptso16<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso17(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PTSO17 != 0"]
    #[inline] pub fn test_ptso17(&self) -> bool {
        self.ptso17() != 0
    }

    #[doc="Sets the PTSO17 field."]
    #[inline] pub fn set_ptso17<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso18(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PTSO18 != 0"]
    #[inline] pub fn test_ptso18(&self) -> bool {
        self.ptso18() != 0
    }

    #[doc="Sets the PTSO18 field."]
    #[inline] pub fn set_ptso18<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso19(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PTSO19 != 0"]
    #[inline] pub fn test_ptso19(&self) -> bool {
        self.ptso19() != 0
    }

    #[doc="Sets the PTSO19 field."]
    #[inline] pub fn set_ptso19<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso20(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PTSO20 != 0"]
    #[inline] pub fn test_ptso20(&self) -> bool {
        self.ptso20() != 0
    }

    #[doc="Sets the PTSO20 field."]
    #[inline] pub fn set_ptso20<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso21(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PTSO21 != 0"]
    #[inline] pub fn test_ptso21(&self) -> bool {
        self.ptso21() != 0
    }

    #[doc="Sets the PTSO21 field."]
    #[inline] pub fn set_ptso21<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso22(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PTSO22 != 0"]
    #[inline] pub fn test_ptso22(&self) -> bool {
        self.ptso22() != 0
    }

    #[doc="Sets the PTSO22 field."]
    #[inline] pub fn set_ptso22<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso23(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PTSO23 != 0"]
    #[inline] pub fn test_ptso23(&self) -> bool {
        self.ptso23() != 0
    }

    #[doc="Sets the PTSO23 field."]
    #[inline] pub fn set_ptso23<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso24(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PTSO24 != 0"]
    #[inline] pub fn test_ptso24(&self) -> bool {
        self.ptso24() != 0
    }

    #[doc="Sets the PTSO24 field."]
    #[inline] pub fn set_ptso24<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso25(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PTSO25 != 0"]
    #[inline] pub fn test_ptso25(&self) -> bool {
        self.ptso25() != 0
    }

    #[doc="Sets the PTSO25 field."]
    #[inline] pub fn set_ptso25<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso26(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PTSO26 != 0"]
    #[inline] pub fn test_ptso26(&self) -> bool {
        self.ptso26() != 0
    }

    #[doc="Sets the PTSO26 field."]
    #[inline] pub fn set_ptso26<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso27(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PTSO27 != 0"]
    #[inline] pub fn test_ptso27(&self) -> bool {
        self.ptso27() != 0
    }

    #[doc="Sets the PTSO27 field."]
    #[inline] pub fn set_ptso27<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso28(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PTSO28 != 0"]
    #[inline] pub fn test_ptso28(&self) -> bool {
        self.ptso28() != 0
    }

    #[doc="Sets the PTSO28 field."]
    #[inline] pub fn set_ptso28<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso29(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if PTSO29 != 0"]
    #[inline] pub fn test_ptso29(&self) -> bool {
        self.ptso29() != 0
    }

    #[doc="Sets the PTSO29 field."]
    #[inline] pub fn set_ptso29<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso30(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if PTSO30 != 0"]
    #[inline] pub fn test_ptso30(&self) -> bool {
        self.ptso30() != 0
    }

    #[doc="Sets the PTSO30 field."]
    #[inline] pub fn set_ptso30<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Port Set Output"]
    #[inline] pub fn ptso31(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PTSO31 != 0"]
    #[inline] pub fn test_ptso31(&self) -> bool {
        self.ptso31() != 0
    }

    #[doc="Sets the PTSO31 field."]
    #[inline] pub fn set_ptso31<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.ptso0() != 0 { try!(write!(f, " ptso0"))}
        if self.ptso1() != 0 { try!(write!(f, " ptso1"))}
        if self.ptso2() != 0 { try!(write!(f, " ptso2"))}
        if self.ptso3() != 0 { try!(write!(f, " ptso3"))}
        if self.ptso4() != 0 { try!(write!(f, " ptso4"))}
        if self.ptso5() != 0 { try!(write!(f, " ptso5"))}
        if self.ptso6() != 0 { try!(write!(f, " ptso6"))}
        if self.ptso7() != 0 { try!(write!(f, " ptso7"))}
        if self.ptso8() != 0 { try!(write!(f, " ptso8"))}
        if self.ptso9() != 0 { try!(write!(f, " ptso9"))}
        if self.ptso10() != 0 { try!(write!(f, " ptso10"))}
        if self.ptso11() != 0 { try!(write!(f, " ptso11"))}
        if self.ptso12() != 0 { try!(write!(f, " ptso12"))}
        if self.ptso13() != 0 { try!(write!(f, " ptso13"))}
        if self.ptso14() != 0 { try!(write!(f, " ptso14"))}
        if self.ptso15() != 0 { try!(write!(f, " ptso15"))}
        if self.ptso16() != 0 { try!(write!(f, " ptso16"))}
        if self.ptso17() != 0 { try!(write!(f, " ptso17"))}
        if self.ptso18() != 0 { try!(write!(f, " ptso18"))}
        if self.ptso19() != 0 { try!(write!(f, " ptso19"))}
        if self.ptso20() != 0 { try!(write!(f, " ptso20"))}
        if self.ptso21() != 0 { try!(write!(f, " ptso21"))}
        if self.ptso22() != 0 { try!(write!(f, " ptso22"))}
        if self.ptso23() != 0 { try!(write!(f, " ptso23"))}
        if self.ptso24() != 0 { try!(write!(f, " ptso24"))}
        if self.ptso25() != 0 { try!(write!(f, " ptso25"))}
        if self.ptso26() != 0 { try!(write!(f, " ptso26"))}
        if self.ptso27() != 0 { try!(write!(f, " ptso27"))}
        if self.ptso28() != 0 { try!(write!(f, " ptso28"))}
        if self.ptso29() != 0 { try!(write!(f, " ptso29"))}
        if self.ptso30() != 0 { try!(write!(f, " ptso30"))}
        if self.ptso31() != 0 { try!(write!(f, " ptso31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Clear Output Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pcor(pub u32);
impl Pcor {
    #[doc="Port Clear Output"]
    #[inline] pub fn ptco0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PTCO0 != 0"]
    #[inline] pub fn test_ptco0(&self) -> bool {
        self.ptco0() != 0
    }

    #[doc="Sets the PTCO0 field."]
    #[inline] pub fn set_ptco0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PTCO1 != 0"]
    #[inline] pub fn test_ptco1(&self) -> bool {
        self.ptco1() != 0
    }

    #[doc="Sets the PTCO1 field."]
    #[inline] pub fn set_ptco1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PTCO2 != 0"]
    #[inline] pub fn test_ptco2(&self) -> bool {
        self.ptco2() != 0
    }

    #[doc="Sets the PTCO2 field."]
    #[inline] pub fn set_ptco2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PTCO3 != 0"]
    #[inline] pub fn test_ptco3(&self) -> bool {
        self.ptco3() != 0
    }

    #[doc="Sets the PTCO3 field."]
    #[inline] pub fn set_ptco3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PTCO4 != 0"]
    #[inline] pub fn test_ptco4(&self) -> bool {
        self.ptco4() != 0
    }

    #[doc="Sets the PTCO4 field."]
    #[inline] pub fn set_ptco4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PTCO5 != 0"]
    #[inline] pub fn test_ptco5(&self) -> bool {
        self.ptco5() != 0
    }

    #[doc="Sets the PTCO5 field."]
    #[inline] pub fn set_ptco5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PTCO6 != 0"]
    #[inline] pub fn test_ptco6(&self) -> bool {
        self.ptco6() != 0
    }

    #[doc="Sets the PTCO6 field."]
    #[inline] pub fn set_ptco6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PTCO7 != 0"]
    #[inline] pub fn test_ptco7(&self) -> bool {
        self.ptco7() != 0
    }

    #[doc="Sets the PTCO7 field."]
    #[inline] pub fn set_ptco7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PTCO8 != 0"]
    #[inline] pub fn test_ptco8(&self) -> bool {
        self.ptco8() != 0
    }

    #[doc="Sets the PTCO8 field."]
    #[inline] pub fn set_ptco8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PTCO9 != 0"]
    #[inline] pub fn test_ptco9(&self) -> bool {
        self.ptco9() != 0
    }

    #[doc="Sets the PTCO9 field."]
    #[inline] pub fn set_ptco9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PTCO10 != 0"]
    #[inline] pub fn test_ptco10(&self) -> bool {
        self.ptco10() != 0
    }

    #[doc="Sets the PTCO10 field."]
    #[inline] pub fn set_ptco10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PTCO11 != 0"]
    #[inline] pub fn test_ptco11(&self) -> bool {
        self.ptco11() != 0
    }

    #[doc="Sets the PTCO11 field."]
    #[inline] pub fn set_ptco11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PTCO12 != 0"]
    #[inline] pub fn test_ptco12(&self) -> bool {
        self.ptco12() != 0
    }

    #[doc="Sets the PTCO12 field."]
    #[inline] pub fn set_ptco12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PTCO13 != 0"]
    #[inline] pub fn test_ptco13(&self) -> bool {
        self.ptco13() != 0
    }

    #[doc="Sets the PTCO13 field."]
    #[inline] pub fn set_ptco13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PTCO14 != 0"]
    #[inline] pub fn test_ptco14(&self) -> bool {
        self.ptco14() != 0
    }

    #[doc="Sets the PTCO14 field."]
    #[inline] pub fn set_ptco14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PTCO15 != 0"]
    #[inline] pub fn test_ptco15(&self) -> bool {
        self.ptco15() != 0
    }

    #[doc="Sets the PTCO15 field."]
    #[inline] pub fn set_ptco15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco16(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PTCO16 != 0"]
    #[inline] pub fn test_ptco16(&self) -> bool {
        self.ptco16() != 0
    }

    #[doc="Sets the PTCO16 field."]
    #[inline] pub fn set_ptco16<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco17(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PTCO17 != 0"]
    #[inline] pub fn test_ptco17(&self) -> bool {
        self.ptco17() != 0
    }

    #[doc="Sets the PTCO17 field."]
    #[inline] pub fn set_ptco17<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco18(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PTCO18 != 0"]
    #[inline] pub fn test_ptco18(&self) -> bool {
        self.ptco18() != 0
    }

    #[doc="Sets the PTCO18 field."]
    #[inline] pub fn set_ptco18<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco19(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PTCO19 != 0"]
    #[inline] pub fn test_ptco19(&self) -> bool {
        self.ptco19() != 0
    }

    #[doc="Sets the PTCO19 field."]
    #[inline] pub fn set_ptco19<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco20(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PTCO20 != 0"]
    #[inline] pub fn test_ptco20(&self) -> bool {
        self.ptco20() != 0
    }

    #[doc="Sets the PTCO20 field."]
    #[inline] pub fn set_ptco20<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco21(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PTCO21 != 0"]
    #[inline] pub fn test_ptco21(&self) -> bool {
        self.ptco21() != 0
    }

    #[doc="Sets the PTCO21 field."]
    #[inline] pub fn set_ptco21<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco22(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PTCO22 != 0"]
    #[inline] pub fn test_ptco22(&self) -> bool {
        self.ptco22() != 0
    }

    #[doc="Sets the PTCO22 field."]
    #[inline] pub fn set_ptco22<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco23(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PTCO23 != 0"]
    #[inline] pub fn test_ptco23(&self) -> bool {
        self.ptco23() != 0
    }

    #[doc="Sets the PTCO23 field."]
    #[inline] pub fn set_ptco23<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco24(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PTCO24 != 0"]
    #[inline] pub fn test_ptco24(&self) -> bool {
        self.ptco24() != 0
    }

    #[doc="Sets the PTCO24 field."]
    #[inline] pub fn set_ptco24<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco25(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PTCO25 != 0"]
    #[inline] pub fn test_ptco25(&self) -> bool {
        self.ptco25() != 0
    }

    #[doc="Sets the PTCO25 field."]
    #[inline] pub fn set_ptco25<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco26(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PTCO26 != 0"]
    #[inline] pub fn test_ptco26(&self) -> bool {
        self.ptco26() != 0
    }

    #[doc="Sets the PTCO26 field."]
    #[inline] pub fn set_ptco26<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco27(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PTCO27 != 0"]
    #[inline] pub fn test_ptco27(&self) -> bool {
        self.ptco27() != 0
    }

    #[doc="Sets the PTCO27 field."]
    #[inline] pub fn set_ptco27<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco28(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PTCO28 != 0"]
    #[inline] pub fn test_ptco28(&self) -> bool {
        self.ptco28() != 0
    }

    #[doc="Sets the PTCO28 field."]
    #[inline] pub fn set_ptco28<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco29(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if PTCO29 != 0"]
    #[inline] pub fn test_ptco29(&self) -> bool {
        self.ptco29() != 0
    }

    #[doc="Sets the PTCO29 field."]
    #[inline] pub fn set_ptco29<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco30(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if PTCO30 != 0"]
    #[inline] pub fn test_ptco30(&self) -> bool {
        self.ptco30() != 0
    }

    #[doc="Sets the PTCO30 field."]
    #[inline] pub fn set_ptco30<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Port Clear Output"]
    #[inline] pub fn ptco31(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PTCO31 != 0"]
    #[inline] pub fn test_ptco31(&self) -> bool {
        self.ptco31() != 0
    }

    #[doc="Sets the PTCO31 field."]
    #[inline] pub fn set_ptco31<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.ptco0() != 0 { try!(write!(f, " ptco0"))}
        if self.ptco1() != 0 { try!(write!(f, " ptco1"))}
        if self.ptco2() != 0 { try!(write!(f, " ptco2"))}
        if self.ptco3() != 0 { try!(write!(f, " ptco3"))}
        if self.ptco4() != 0 { try!(write!(f, " ptco4"))}
        if self.ptco5() != 0 { try!(write!(f, " ptco5"))}
        if self.ptco6() != 0 { try!(write!(f, " ptco6"))}
        if self.ptco7() != 0 { try!(write!(f, " ptco7"))}
        if self.ptco8() != 0 { try!(write!(f, " ptco8"))}
        if self.ptco9() != 0 { try!(write!(f, " ptco9"))}
        if self.ptco10() != 0 { try!(write!(f, " ptco10"))}
        if self.ptco11() != 0 { try!(write!(f, " ptco11"))}
        if self.ptco12() != 0 { try!(write!(f, " ptco12"))}
        if self.ptco13() != 0 { try!(write!(f, " ptco13"))}
        if self.ptco14() != 0 { try!(write!(f, " ptco14"))}
        if self.ptco15() != 0 { try!(write!(f, " ptco15"))}
        if self.ptco16() != 0 { try!(write!(f, " ptco16"))}
        if self.ptco17() != 0 { try!(write!(f, " ptco17"))}
        if self.ptco18() != 0 { try!(write!(f, " ptco18"))}
        if self.ptco19() != 0 { try!(write!(f, " ptco19"))}
        if self.ptco20() != 0 { try!(write!(f, " ptco20"))}
        if self.ptco21() != 0 { try!(write!(f, " ptco21"))}
        if self.ptco22() != 0 { try!(write!(f, " ptco22"))}
        if self.ptco23() != 0 { try!(write!(f, " ptco23"))}
        if self.ptco24() != 0 { try!(write!(f, " ptco24"))}
        if self.ptco25() != 0 { try!(write!(f, " ptco25"))}
        if self.ptco26() != 0 { try!(write!(f, " ptco26"))}
        if self.ptco27() != 0 { try!(write!(f, " ptco27"))}
        if self.ptco28() != 0 { try!(write!(f, " ptco28"))}
        if self.ptco29() != 0 { try!(write!(f, " ptco29"))}
        if self.ptco30() != 0 { try!(write!(f, " ptco30"))}
        if self.ptco31() != 0 { try!(write!(f, " ptco31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Toggle Output Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ptor(pub u32);
impl Ptor {
    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PTTO0 != 0"]
    #[inline] pub fn test_ptto0(&self) -> bool {
        self.ptto0() != 0
    }

    #[doc="Sets the PTTO0 field."]
    #[inline] pub fn set_ptto0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PTTO1 != 0"]
    #[inline] pub fn test_ptto1(&self) -> bool {
        self.ptto1() != 0
    }

    #[doc="Sets the PTTO1 field."]
    #[inline] pub fn set_ptto1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PTTO2 != 0"]
    #[inline] pub fn test_ptto2(&self) -> bool {
        self.ptto2() != 0
    }

    #[doc="Sets the PTTO2 field."]
    #[inline] pub fn set_ptto2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PTTO3 != 0"]
    #[inline] pub fn test_ptto3(&self) -> bool {
        self.ptto3() != 0
    }

    #[doc="Sets the PTTO3 field."]
    #[inline] pub fn set_ptto3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PTTO4 != 0"]
    #[inline] pub fn test_ptto4(&self) -> bool {
        self.ptto4() != 0
    }

    #[doc="Sets the PTTO4 field."]
    #[inline] pub fn set_ptto4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PTTO5 != 0"]
    #[inline] pub fn test_ptto5(&self) -> bool {
        self.ptto5() != 0
    }

    #[doc="Sets the PTTO5 field."]
    #[inline] pub fn set_ptto5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PTTO6 != 0"]
    #[inline] pub fn test_ptto6(&self) -> bool {
        self.ptto6() != 0
    }

    #[doc="Sets the PTTO6 field."]
    #[inline] pub fn set_ptto6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PTTO7 != 0"]
    #[inline] pub fn test_ptto7(&self) -> bool {
        self.ptto7() != 0
    }

    #[doc="Sets the PTTO7 field."]
    #[inline] pub fn set_ptto7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PTTO8 != 0"]
    #[inline] pub fn test_ptto8(&self) -> bool {
        self.ptto8() != 0
    }

    #[doc="Sets the PTTO8 field."]
    #[inline] pub fn set_ptto8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PTTO9 != 0"]
    #[inline] pub fn test_ptto9(&self) -> bool {
        self.ptto9() != 0
    }

    #[doc="Sets the PTTO9 field."]
    #[inline] pub fn set_ptto9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PTTO10 != 0"]
    #[inline] pub fn test_ptto10(&self) -> bool {
        self.ptto10() != 0
    }

    #[doc="Sets the PTTO10 field."]
    #[inline] pub fn set_ptto10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PTTO11 != 0"]
    #[inline] pub fn test_ptto11(&self) -> bool {
        self.ptto11() != 0
    }

    #[doc="Sets the PTTO11 field."]
    #[inline] pub fn set_ptto11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PTTO12 != 0"]
    #[inline] pub fn test_ptto12(&self) -> bool {
        self.ptto12() != 0
    }

    #[doc="Sets the PTTO12 field."]
    #[inline] pub fn set_ptto12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PTTO13 != 0"]
    #[inline] pub fn test_ptto13(&self) -> bool {
        self.ptto13() != 0
    }

    #[doc="Sets the PTTO13 field."]
    #[inline] pub fn set_ptto13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PTTO14 != 0"]
    #[inline] pub fn test_ptto14(&self) -> bool {
        self.ptto14() != 0
    }

    #[doc="Sets the PTTO14 field."]
    #[inline] pub fn set_ptto14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PTTO15 != 0"]
    #[inline] pub fn test_ptto15(&self) -> bool {
        self.ptto15() != 0
    }

    #[doc="Sets the PTTO15 field."]
    #[inline] pub fn set_ptto15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto16(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PTTO16 != 0"]
    #[inline] pub fn test_ptto16(&self) -> bool {
        self.ptto16() != 0
    }

    #[doc="Sets the PTTO16 field."]
    #[inline] pub fn set_ptto16<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto17(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PTTO17 != 0"]
    #[inline] pub fn test_ptto17(&self) -> bool {
        self.ptto17() != 0
    }

    #[doc="Sets the PTTO17 field."]
    #[inline] pub fn set_ptto17<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto18(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PTTO18 != 0"]
    #[inline] pub fn test_ptto18(&self) -> bool {
        self.ptto18() != 0
    }

    #[doc="Sets the PTTO18 field."]
    #[inline] pub fn set_ptto18<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto19(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PTTO19 != 0"]
    #[inline] pub fn test_ptto19(&self) -> bool {
        self.ptto19() != 0
    }

    #[doc="Sets the PTTO19 field."]
    #[inline] pub fn set_ptto19<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto20(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PTTO20 != 0"]
    #[inline] pub fn test_ptto20(&self) -> bool {
        self.ptto20() != 0
    }

    #[doc="Sets the PTTO20 field."]
    #[inline] pub fn set_ptto20<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto21(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PTTO21 != 0"]
    #[inline] pub fn test_ptto21(&self) -> bool {
        self.ptto21() != 0
    }

    #[doc="Sets the PTTO21 field."]
    #[inline] pub fn set_ptto21<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto22(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PTTO22 != 0"]
    #[inline] pub fn test_ptto22(&self) -> bool {
        self.ptto22() != 0
    }

    #[doc="Sets the PTTO22 field."]
    #[inline] pub fn set_ptto22<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto23(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PTTO23 != 0"]
    #[inline] pub fn test_ptto23(&self) -> bool {
        self.ptto23() != 0
    }

    #[doc="Sets the PTTO23 field."]
    #[inline] pub fn set_ptto23<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto24(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PTTO24 != 0"]
    #[inline] pub fn test_ptto24(&self) -> bool {
        self.ptto24() != 0
    }

    #[doc="Sets the PTTO24 field."]
    #[inline] pub fn set_ptto24<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto25(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PTTO25 != 0"]
    #[inline] pub fn test_ptto25(&self) -> bool {
        self.ptto25() != 0
    }

    #[doc="Sets the PTTO25 field."]
    #[inline] pub fn set_ptto25<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto26(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PTTO26 != 0"]
    #[inline] pub fn test_ptto26(&self) -> bool {
        self.ptto26() != 0
    }

    #[doc="Sets the PTTO26 field."]
    #[inline] pub fn set_ptto26<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto27(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PTTO27 != 0"]
    #[inline] pub fn test_ptto27(&self) -> bool {
        self.ptto27() != 0
    }

    #[doc="Sets the PTTO27 field."]
    #[inline] pub fn set_ptto27<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto28(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PTTO28 != 0"]
    #[inline] pub fn test_ptto28(&self) -> bool {
        self.ptto28() != 0
    }

    #[doc="Sets the PTTO28 field."]
    #[inline] pub fn set_ptto28<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto29(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if PTTO29 != 0"]
    #[inline] pub fn test_ptto29(&self) -> bool {
        self.ptto29() != 0
    }

    #[doc="Sets the PTTO29 field."]
    #[inline] pub fn set_ptto29<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto30(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if PTTO30 != 0"]
    #[inline] pub fn test_ptto30(&self) -> bool {
        self.ptto30() != 0
    }

    #[doc="Sets the PTTO30 field."]
    #[inline] pub fn set_ptto30<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Port Toggle Output"]
    #[inline] pub fn ptto31(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PTTO31 != 0"]
    #[inline] pub fn test_ptto31(&self) -> bool {
        self.ptto31() != 0
    }

    #[doc="Sets the PTTO31 field."]
    #[inline] pub fn set_ptto31<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.ptto0() != 0 { try!(write!(f, " ptto0"))}
        if self.ptto1() != 0 { try!(write!(f, " ptto1"))}
        if self.ptto2() != 0 { try!(write!(f, " ptto2"))}
        if self.ptto3() != 0 { try!(write!(f, " ptto3"))}
        if self.ptto4() != 0 { try!(write!(f, " ptto4"))}
        if self.ptto5() != 0 { try!(write!(f, " ptto5"))}
        if self.ptto6() != 0 { try!(write!(f, " ptto6"))}
        if self.ptto7() != 0 { try!(write!(f, " ptto7"))}
        if self.ptto8() != 0 { try!(write!(f, " ptto8"))}
        if self.ptto9() != 0 { try!(write!(f, " ptto9"))}
        if self.ptto10() != 0 { try!(write!(f, " ptto10"))}
        if self.ptto11() != 0 { try!(write!(f, " ptto11"))}
        if self.ptto12() != 0 { try!(write!(f, " ptto12"))}
        if self.ptto13() != 0 { try!(write!(f, " ptto13"))}
        if self.ptto14() != 0 { try!(write!(f, " ptto14"))}
        if self.ptto15() != 0 { try!(write!(f, " ptto15"))}
        if self.ptto16() != 0 { try!(write!(f, " ptto16"))}
        if self.ptto17() != 0 { try!(write!(f, " ptto17"))}
        if self.ptto18() != 0 { try!(write!(f, " ptto18"))}
        if self.ptto19() != 0 { try!(write!(f, " ptto19"))}
        if self.ptto20() != 0 { try!(write!(f, " ptto20"))}
        if self.ptto21() != 0 { try!(write!(f, " ptto21"))}
        if self.ptto22() != 0 { try!(write!(f, " ptto22"))}
        if self.ptto23() != 0 { try!(write!(f, " ptto23"))}
        if self.ptto24() != 0 { try!(write!(f, " ptto24"))}
        if self.ptto25() != 0 { try!(write!(f, " ptto25"))}
        if self.ptto26() != 0 { try!(write!(f, " ptto26"))}
        if self.ptto27() != 0 { try!(write!(f, " ptto27"))}
        if self.ptto28() != 0 { try!(write!(f, " ptto28"))}
        if self.ptto29() != 0 { try!(write!(f, " ptto29"))}
        if self.ptto30() != 0 { try!(write!(f, " ptto30"))}
        if self.ptto31() != 0 { try!(write!(f, " ptto31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Data Input Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pdir(pub u32);
impl Pdir {
    #[doc="Port Data Input"]
    #[inline] pub fn pdi0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PDI0 != 0"]
    #[inline] pub fn test_pdi0(&self) -> bool {
        self.pdi0() != 0
    }

    #[doc="Sets the PDI0 field."]
    #[inline] pub fn set_pdi0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PDI1 != 0"]
    #[inline] pub fn test_pdi1(&self) -> bool {
        self.pdi1() != 0
    }

    #[doc="Sets the PDI1 field."]
    #[inline] pub fn set_pdi1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PDI2 != 0"]
    #[inline] pub fn test_pdi2(&self) -> bool {
        self.pdi2() != 0
    }

    #[doc="Sets the PDI2 field."]
    #[inline] pub fn set_pdi2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PDI3 != 0"]
    #[inline] pub fn test_pdi3(&self) -> bool {
        self.pdi3() != 0
    }

    #[doc="Sets the PDI3 field."]
    #[inline] pub fn set_pdi3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PDI4 != 0"]
    #[inline] pub fn test_pdi4(&self) -> bool {
        self.pdi4() != 0
    }

    #[doc="Sets the PDI4 field."]
    #[inline] pub fn set_pdi4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PDI5 != 0"]
    #[inline] pub fn test_pdi5(&self) -> bool {
        self.pdi5() != 0
    }

    #[doc="Sets the PDI5 field."]
    #[inline] pub fn set_pdi5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PDI6 != 0"]
    #[inline] pub fn test_pdi6(&self) -> bool {
        self.pdi6() != 0
    }

    #[doc="Sets the PDI6 field."]
    #[inline] pub fn set_pdi6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PDI7 != 0"]
    #[inline] pub fn test_pdi7(&self) -> bool {
        self.pdi7() != 0
    }

    #[doc="Sets the PDI7 field."]
    #[inline] pub fn set_pdi7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PDI8 != 0"]
    #[inline] pub fn test_pdi8(&self) -> bool {
        self.pdi8() != 0
    }

    #[doc="Sets the PDI8 field."]
    #[inline] pub fn set_pdi8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PDI9 != 0"]
    #[inline] pub fn test_pdi9(&self) -> bool {
        self.pdi9() != 0
    }

    #[doc="Sets the PDI9 field."]
    #[inline] pub fn set_pdi9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PDI10 != 0"]
    #[inline] pub fn test_pdi10(&self) -> bool {
        self.pdi10() != 0
    }

    #[doc="Sets the PDI10 field."]
    #[inline] pub fn set_pdi10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PDI11 != 0"]
    #[inline] pub fn test_pdi11(&self) -> bool {
        self.pdi11() != 0
    }

    #[doc="Sets the PDI11 field."]
    #[inline] pub fn set_pdi11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PDI12 != 0"]
    #[inline] pub fn test_pdi12(&self) -> bool {
        self.pdi12() != 0
    }

    #[doc="Sets the PDI12 field."]
    #[inline] pub fn set_pdi12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PDI13 != 0"]
    #[inline] pub fn test_pdi13(&self) -> bool {
        self.pdi13() != 0
    }

    #[doc="Sets the PDI13 field."]
    #[inline] pub fn set_pdi13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PDI14 != 0"]
    #[inline] pub fn test_pdi14(&self) -> bool {
        self.pdi14() != 0
    }

    #[doc="Sets the PDI14 field."]
    #[inline] pub fn set_pdi14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PDI15 != 0"]
    #[inline] pub fn test_pdi15(&self) -> bool {
        self.pdi15() != 0
    }

    #[doc="Sets the PDI15 field."]
    #[inline] pub fn set_pdi15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi16(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PDI16 != 0"]
    #[inline] pub fn test_pdi16(&self) -> bool {
        self.pdi16() != 0
    }

    #[doc="Sets the PDI16 field."]
    #[inline] pub fn set_pdi16<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi17(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PDI17 != 0"]
    #[inline] pub fn test_pdi17(&self) -> bool {
        self.pdi17() != 0
    }

    #[doc="Sets the PDI17 field."]
    #[inline] pub fn set_pdi17<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi18(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PDI18 != 0"]
    #[inline] pub fn test_pdi18(&self) -> bool {
        self.pdi18() != 0
    }

    #[doc="Sets the PDI18 field."]
    #[inline] pub fn set_pdi18<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi19(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PDI19 != 0"]
    #[inline] pub fn test_pdi19(&self) -> bool {
        self.pdi19() != 0
    }

    #[doc="Sets the PDI19 field."]
    #[inline] pub fn set_pdi19<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi20(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PDI20 != 0"]
    #[inline] pub fn test_pdi20(&self) -> bool {
        self.pdi20() != 0
    }

    #[doc="Sets the PDI20 field."]
    #[inline] pub fn set_pdi20<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi21(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PDI21 != 0"]
    #[inline] pub fn test_pdi21(&self) -> bool {
        self.pdi21() != 0
    }

    #[doc="Sets the PDI21 field."]
    #[inline] pub fn set_pdi21<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi22(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PDI22 != 0"]
    #[inline] pub fn test_pdi22(&self) -> bool {
        self.pdi22() != 0
    }

    #[doc="Sets the PDI22 field."]
    #[inline] pub fn set_pdi22<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi23(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PDI23 != 0"]
    #[inline] pub fn test_pdi23(&self) -> bool {
        self.pdi23() != 0
    }

    #[doc="Sets the PDI23 field."]
    #[inline] pub fn set_pdi23<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi24(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PDI24 != 0"]
    #[inline] pub fn test_pdi24(&self) -> bool {
        self.pdi24() != 0
    }

    #[doc="Sets the PDI24 field."]
    #[inline] pub fn set_pdi24<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi25(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PDI25 != 0"]
    #[inline] pub fn test_pdi25(&self) -> bool {
        self.pdi25() != 0
    }

    #[doc="Sets the PDI25 field."]
    #[inline] pub fn set_pdi25<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi26(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PDI26 != 0"]
    #[inline] pub fn test_pdi26(&self) -> bool {
        self.pdi26() != 0
    }

    #[doc="Sets the PDI26 field."]
    #[inline] pub fn set_pdi26<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi27(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PDI27 != 0"]
    #[inline] pub fn test_pdi27(&self) -> bool {
        self.pdi27() != 0
    }

    #[doc="Sets the PDI27 field."]
    #[inline] pub fn set_pdi27<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi28(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PDI28 != 0"]
    #[inline] pub fn test_pdi28(&self) -> bool {
        self.pdi28() != 0
    }

    #[doc="Sets the PDI28 field."]
    #[inline] pub fn set_pdi28<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi29(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if PDI29 != 0"]
    #[inline] pub fn test_pdi29(&self) -> bool {
        self.pdi29() != 0
    }

    #[doc="Sets the PDI29 field."]
    #[inline] pub fn set_pdi29<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi30(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if PDI30 != 0"]
    #[inline] pub fn test_pdi30(&self) -> bool {
        self.pdi30() != 0
    }

    #[doc="Sets the PDI30 field."]
    #[inline] pub fn set_pdi30<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Port Data Input"]
    #[inline] pub fn pdi31(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PDI31 != 0"]
    #[inline] pub fn test_pdi31(&self) -> bool {
        self.pdi31() != 0
    }

    #[doc="Sets the PDI31 field."]
    #[inline] pub fn set_pdi31<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.pdi0() != 0 { try!(write!(f, " pdi0"))}
        if self.pdi1() != 0 { try!(write!(f, " pdi1"))}
        if self.pdi2() != 0 { try!(write!(f, " pdi2"))}
        if self.pdi3() != 0 { try!(write!(f, " pdi3"))}
        if self.pdi4() != 0 { try!(write!(f, " pdi4"))}
        if self.pdi5() != 0 { try!(write!(f, " pdi5"))}
        if self.pdi6() != 0 { try!(write!(f, " pdi6"))}
        if self.pdi7() != 0 { try!(write!(f, " pdi7"))}
        if self.pdi8() != 0 { try!(write!(f, " pdi8"))}
        if self.pdi9() != 0 { try!(write!(f, " pdi9"))}
        if self.pdi10() != 0 { try!(write!(f, " pdi10"))}
        if self.pdi11() != 0 { try!(write!(f, " pdi11"))}
        if self.pdi12() != 0 { try!(write!(f, " pdi12"))}
        if self.pdi13() != 0 { try!(write!(f, " pdi13"))}
        if self.pdi14() != 0 { try!(write!(f, " pdi14"))}
        if self.pdi15() != 0 { try!(write!(f, " pdi15"))}
        if self.pdi16() != 0 { try!(write!(f, " pdi16"))}
        if self.pdi17() != 0 { try!(write!(f, " pdi17"))}
        if self.pdi18() != 0 { try!(write!(f, " pdi18"))}
        if self.pdi19() != 0 { try!(write!(f, " pdi19"))}
        if self.pdi20() != 0 { try!(write!(f, " pdi20"))}
        if self.pdi21() != 0 { try!(write!(f, " pdi21"))}
        if self.pdi22() != 0 { try!(write!(f, " pdi22"))}
        if self.pdi23() != 0 { try!(write!(f, " pdi23"))}
        if self.pdi24() != 0 { try!(write!(f, " pdi24"))}
        if self.pdi25() != 0 { try!(write!(f, " pdi25"))}
        if self.pdi26() != 0 { try!(write!(f, " pdi26"))}
        if self.pdi27() != 0 { try!(write!(f, " pdi27"))}
        if self.pdi28() != 0 { try!(write!(f, " pdi28"))}
        if self.pdi29() != 0 { try!(write!(f, " pdi29"))}
        if self.pdi30() != 0 { try!(write!(f, " pdi30"))}
        if self.pdi31() != 0 { try!(write!(f, " pdi31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Data Direction Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pddr(pub u32);
impl Pddr {
    #[doc="Port Data Direction"]
    #[inline] pub fn pdd0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PDD0 != 0"]
    #[inline] pub fn test_pdd0(&self) -> bool {
        self.pdd0() != 0
    }

    #[doc="Sets the PDD0 field."]
    #[inline] pub fn set_pdd0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PDD1 != 0"]
    #[inline] pub fn test_pdd1(&self) -> bool {
        self.pdd1() != 0
    }

    #[doc="Sets the PDD1 field."]
    #[inline] pub fn set_pdd1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PDD2 != 0"]
    #[inline] pub fn test_pdd2(&self) -> bool {
        self.pdd2() != 0
    }

    #[doc="Sets the PDD2 field."]
    #[inline] pub fn set_pdd2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PDD3 != 0"]
    #[inline] pub fn test_pdd3(&self) -> bool {
        self.pdd3() != 0
    }

    #[doc="Sets the PDD3 field."]
    #[inline] pub fn set_pdd3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PDD4 != 0"]
    #[inline] pub fn test_pdd4(&self) -> bool {
        self.pdd4() != 0
    }

    #[doc="Sets the PDD4 field."]
    #[inline] pub fn set_pdd4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PDD5 != 0"]
    #[inline] pub fn test_pdd5(&self) -> bool {
        self.pdd5() != 0
    }

    #[doc="Sets the PDD5 field."]
    #[inline] pub fn set_pdd5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PDD6 != 0"]
    #[inline] pub fn test_pdd6(&self) -> bool {
        self.pdd6() != 0
    }

    #[doc="Sets the PDD6 field."]
    #[inline] pub fn set_pdd6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PDD7 != 0"]
    #[inline] pub fn test_pdd7(&self) -> bool {
        self.pdd7() != 0
    }

    #[doc="Sets the PDD7 field."]
    #[inline] pub fn set_pdd7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PDD8 != 0"]
    #[inline] pub fn test_pdd8(&self) -> bool {
        self.pdd8() != 0
    }

    #[doc="Sets the PDD8 field."]
    #[inline] pub fn set_pdd8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PDD9 != 0"]
    #[inline] pub fn test_pdd9(&self) -> bool {
        self.pdd9() != 0
    }

    #[doc="Sets the PDD9 field."]
    #[inline] pub fn set_pdd9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PDD10 != 0"]
    #[inline] pub fn test_pdd10(&self) -> bool {
        self.pdd10() != 0
    }

    #[doc="Sets the PDD10 field."]
    #[inline] pub fn set_pdd10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PDD11 != 0"]
    #[inline] pub fn test_pdd11(&self) -> bool {
        self.pdd11() != 0
    }

    #[doc="Sets the PDD11 field."]
    #[inline] pub fn set_pdd11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PDD12 != 0"]
    #[inline] pub fn test_pdd12(&self) -> bool {
        self.pdd12() != 0
    }

    #[doc="Sets the PDD12 field."]
    #[inline] pub fn set_pdd12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PDD13 != 0"]
    #[inline] pub fn test_pdd13(&self) -> bool {
        self.pdd13() != 0
    }

    #[doc="Sets the PDD13 field."]
    #[inline] pub fn set_pdd13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PDD14 != 0"]
    #[inline] pub fn test_pdd14(&self) -> bool {
        self.pdd14() != 0
    }

    #[doc="Sets the PDD14 field."]
    #[inline] pub fn set_pdd14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PDD15 != 0"]
    #[inline] pub fn test_pdd15(&self) -> bool {
        self.pdd15() != 0
    }

    #[doc="Sets the PDD15 field."]
    #[inline] pub fn set_pdd15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd16(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PDD16 != 0"]
    #[inline] pub fn test_pdd16(&self) -> bool {
        self.pdd16() != 0
    }

    #[doc="Sets the PDD16 field."]
    #[inline] pub fn set_pdd16<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd17(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PDD17 != 0"]
    #[inline] pub fn test_pdd17(&self) -> bool {
        self.pdd17() != 0
    }

    #[doc="Sets the PDD17 field."]
    #[inline] pub fn set_pdd17<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd18(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PDD18 != 0"]
    #[inline] pub fn test_pdd18(&self) -> bool {
        self.pdd18() != 0
    }

    #[doc="Sets the PDD18 field."]
    #[inline] pub fn set_pdd18<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd19(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PDD19 != 0"]
    #[inline] pub fn test_pdd19(&self) -> bool {
        self.pdd19() != 0
    }

    #[doc="Sets the PDD19 field."]
    #[inline] pub fn set_pdd19<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd20(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PDD20 != 0"]
    #[inline] pub fn test_pdd20(&self) -> bool {
        self.pdd20() != 0
    }

    #[doc="Sets the PDD20 field."]
    #[inline] pub fn set_pdd20<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd21(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PDD21 != 0"]
    #[inline] pub fn test_pdd21(&self) -> bool {
        self.pdd21() != 0
    }

    #[doc="Sets the PDD21 field."]
    #[inline] pub fn set_pdd21<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd22(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PDD22 != 0"]
    #[inline] pub fn test_pdd22(&self) -> bool {
        self.pdd22() != 0
    }

    #[doc="Sets the PDD22 field."]
    #[inline] pub fn set_pdd22<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd23(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PDD23 != 0"]
    #[inline] pub fn test_pdd23(&self) -> bool {
        self.pdd23() != 0
    }

    #[doc="Sets the PDD23 field."]
    #[inline] pub fn set_pdd23<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd24(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PDD24 != 0"]
    #[inline] pub fn test_pdd24(&self) -> bool {
        self.pdd24() != 0
    }

    #[doc="Sets the PDD24 field."]
    #[inline] pub fn set_pdd24<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd25(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PDD25 != 0"]
    #[inline] pub fn test_pdd25(&self) -> bool {
        self.pdd25() != 0
    }

    #[doc="Sets the PDD25 field."]
    #[inline] pub fn set_pdd25<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd26(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PDD26 != 0"]
    #[inline] pub fn test_pdd26(&self) -> bool {
        self.pdd26() != 0
    }

    #[doc="Sets the PDD26 field."]
    #[inline] pub fn set_pdd26<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd27(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PDD27 != 0"]
    #[inline] pub fn test_pdd27(&self) -> bool {
        self.pdd27() != 0
    }

    #[doc="Sets the PDD27 field."]
    #[inline] pub fn set_pdd27<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd28(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PDD28 != 0"]
    #[inline] pub fn test_pdd28(&self) -> bool {
        self.pdd28() != 0
    }

    #[doc="Sets the PDD28 field."]
    #[inline] pub fn set_pdd28<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd29(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if PDD29 != 0"]
    #[inline] pub fn test_pdd29(&self) -> bool {
        self.pdd29() != 0
    }

    #[doc="Sets the PDD29 field."]
    #[inline] pub fn set_pdd29<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd30(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if PDD30 != 0"]
    #[inline] pub fn test_pdd30(&self) -> bool {
        self.pdd30() != 0
    }

    #[doc="Sets the PDD30 field."]
    #[inline] pub fn set_pdd30<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Port Data Direction"]
    #[inline] pub fn pdd31(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PDD31 != 0"]
    #[inline] pub fn test_pdd31(&self) -> bool {
        self.pdd31() != 0
    }

    #[doc="Sets the PDD31 field."]
    #[inline] pub fn set_pdd31<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.pdd0() != 0 { try!(write!(f, " pdd0"))}
        if self.pdd1() != 0 { try!(write!(f, " pdd1"))}
        if self.pdd2() != 0 { try!(write!(f, " pdd2"))}
        if self.pdd3() != 0 { try!(write!(f, " pdd3"))}
        if self.pdd4() != 0 { try!(write!(f, " pdd4"))}
        if self.pdd5() != 0 { try!(write!(f, " pdd5"))}
        if self.pdd6() != 0 { try!(write!(f, " pdd6"))}
        if self.pdd7() != 0 { try!(write!(f, " pdd7"))}
        if self.pdd8() != 0 { try!(write!(f, " pdd8"))}
        if self.pdd9() != 0 { try!(write!(f, " pdd9"))}
        if self.pdd10() != 0 { try!(write!(f, " pdd10"))}
        if self.pdd11() != 0 { try!(write!(f, " pdd11"))}
        if self.pdd12() != 0 { try!(write!(f, " pdd12"))}
        if self.pdd13() != 0 { try!(write!(f, " pdd13"))}
        if self.pdd14() != 0 { try!(write!(f, " pdd14"))}
        if self.pdd15() != 0 { try!(write!(f, " pdd15"))}
        if self.pdd16() != 0 { try!(write!(f, " pdd16"))}
        if self.pdd17() != 0 { try!(write!(f, " pdd17"))}
        if self.pdd18() != 0 { try!(write!(f, " pdd18"))}
        if self.pdd19() != 0 { try!(write!(f, " pdd19"))}
        if self.pdd20() != 0 { try!(write!(f, " pdd20"))}
        if self.pdd21() != 0 { try!(write!(f, " pdd21"))}
        if self.pdd22() != 0 { try!(write!(f, " pdd22"))}
        if self.pdd23() != 0 { try!(write!(f, " pdd23"))}
        if self.pdd24() != 0 { try!(write!(f, " pdd24"))}
        if self.pdd25() != 0 { try!(write!(f, " pdd25"))}
        if self.pdd26() != 0 { try!(write!(f, " pdd26"))}
        if self.pdd27() != 0 { try!(write!(f, " pdd27"))}
        if self.pdd28() != 0 { try!(write!(f, " pdd28"))}
        if self.pdd29() != 0 { try!(write!(f, " pdd29"))}
        if self.pdd30() != 0 { try!(write!(f, " pdd30"))}
        if self.pdd31() != 0 { try!(write!(f, " pdd31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Port Input Disable Register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pidr(pub u32);
impl Pidr {
    #[doc="Port Input Disable"]
    #[inline] pub fn pid0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if PID0 != 0"]
    #[inline] pub fn test_pid0(&self) -> bool {
        self.pid0() != 0
    }

    #[doc="Sets the PID0 field."]
    #[inline] pub fn set_pid0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if PID1 != 0"]
    #[inline] pub fn test_pid1(&self) -> bool {
        self.pid1() != 0
    }

    #[doc="Sets the PID1 field."]
    #[inline] pub fn set_pid1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if PID2 != 0"]
    #[inline] pub fn test_pid2(&self) -> bool {
        self.pid2() != 0
    }

    #[doc="Sets the PID2 field."]
    #[inline] pub fn set_pid2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PID3 != 0"]
    #[inline] pub fn test_pid3(&self) -> bool {
        self.pid3() != 0
    }

    #[doc="Sets the PID3 field."]
    #[inline] pub fn set_pid3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if PID4 != 0"]
    #[inline] pub fn test_pid4(&self) -> bool {
        self.pid4() != 0
    }

    #[doc="Sets the PID4 field."]
    #[inline] pub fn set_pid4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if PID5 != 0"]
    #[inline] pub fn test_pid5(&self) -> bool {
        self.pid5() != 0
    }

    #[doc="Sets the PID5 field."]
    #[inline] pub fn set_pid5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if PID6 != 0"]
    #[inline] pub fn test_pid6(&self) -> bool {
        self.pid6() != 0
    }

    #[doc="Sets the PID6 field."]
    #[inline] pub fn set_pid6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if PID7 != 0"]
    #[inline] pub fn test_pid7(&self) -> bool {
        self.pid7() != 0
    }

    #[doc="Sets the PID7 field."]
    #[inline] pub fn set_pid7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid8(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if PID8 != 0"]
    #[inline] pub fn test_pid8(&self) -> bool {
        self.pid8() != 0
    }

    #[doc="Sets the PID8 field."]
    #[inline] pub fn set_pid8<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid9(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if PID9 != 0"]
    #[inline] pub fn test_pid9(&self) -> bool {
        self.pid9() != 0
    }

    #[doc="Sets the PID9 field."]
    #[inline] pub fn set_pid9<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid10(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if PID10 != 0"]
    #[inline] pub fn test_pid10(&self) -> bool {
        self.pid10() != 0
    }

    #[doc="Sets the PID10 field."]
    #[inline] pub fn set_pid10<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid11(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if PID11 != 0"]
    #[inline] pub fn test_pid11(&self) -> bool {
        self.pid11() != 0
    }

    #[doc="Sets the PID11 field."]
    #[inline] pub fn set_pid11<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid12(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if PID12 != 0"]
    #[inline] pub fn test_pid12(&self) -> bool {
        self.pid12() != 0
    }

    #[doc="Sets the PID12 field."]
    #[inline] pub fn set_pid12<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid13(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if PID13 != 0"]
    #[inline] pub fn test_pid13(&self) -> bool {
        self.pid13() != 0
    }

    #[doc="Sets the PID13 field."]
    #[inline] pub fn set_pid13<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid14(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if PID14 != 0"]
    #[inline] pub fn test_pid14(&self) -> bool {
        self.pid14() != 0
    }

    #[doc="Sets the PID14 field."]
    #[inline] pub fn set_pid14<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid15(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if PID15 != 0"]
    #[inline] pub fn test_pid15(&self) -> bool {
        self.pid15() != 0
    }

    #[doc="Sets the PID15 field."]
    #[inline] pub fn set_pid15<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid16(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if PID16 != 0"]
    #[inline] pub fn test_pid16(&self) -> bool {
        self.pid16() != 0
    }

    #[doc="Sets the PID16 field."]
    #[inline] pub fn set_pid16<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid17(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if PID17 != 0"]
    #[inline] pub fn test_pid17(&self) -> bool {
        self.pid17() != 0
    }

    #[doc="Sets the PID17 field."]
    #[inline] pub fn set_pid17<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid18(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if PID18 != 0"]
    #[inline] pub fn test_pid18(&self) -> bool {
        self.pid18() != 0
    }

    #[doc="Sets the PID18 field."]
    #[inline] pub fn set_pid18<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid19(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if PID19 != 0"]
    #[inline] pub fn test_pid19(&self) -> bool {
        self.pid19() != 0
    }

    #[doc="Sets the PID19 field."]
    #[inline] pub fn set_pid19<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid20(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if PID20 != 0"]
    #[inline] pub fn test_pid20(&self) -> bool {
        self.pid20() != 0
    }

    #[doc="Sets the PID20 field."]
    #[inline] pub fn set_pid20<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid21(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if PID21 != 0"]
    #[inline] pub fn test_pid21(&self) -> bool {
        self.pid21() != 0
    }

    #[doc="Sets the PID21 field."]
    #[inline] pub fn set_pid21<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid22(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if PID22 != 0"]
    #[inline] pub fn test_pid22(&self) -> bool {
        self.pid22() != 0
    }

    #[doc="Sets the PID22 field."]
    #[inline] pub fn set_pid22<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid23(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if PID23 != 0"]
    #[inline] pub fn test_pid23(&self) -> bool {
        self.pid23() != 0
    }

    #[doc="Sets the PID23 field."]
    #[inline] pub fn set_pid23<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid24(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if PID24 != 0"]
    #[inline] pub fn test_pid24(&self) -> bool {
        self.pid24() != 0
    }

    #[doc="Sets the PID24 field."]
    #[inline] pub fn set_pid24<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid25(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if PID25 != 0"]
    #[inline] pub fn test_pid25(&self) -> bool {
        self.pid25() != 0
    }

    #[doc="Sets the PID25 field."]
    #[inline] pub fn set_pid25<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid26(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if PID26 != 0"]
    #[inline] pub fn test_pid26(&self) -> bool {
        self.pid26() != 0
    }

    #[doc="Sets the PID26 field."]
    #[inline] pub fn set_pid26<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid27(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if PID27 != 0"]
    #[inline] pub fn test_pid27(&self) -> bool {
        self.pid27() != 0
    }

    #[doc="Sets the PID27 field."]
    #[inline] pub fn set_pid27<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid28(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if PID28 != 0"]
    #[inline] pub fn test_pid28(&self) -> bool {
        self.pid28() != 0
    }

    #[doc="Sets the PID28 field."]
    #[inline] pub fn set_pid28<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid29(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if PID29 != 0"]
    #[inline] pub fn test_pid29(&self) -> bool {
        self.pid29() != 0
    }

    #[doc="Sets the PID29 field."]
    #[inline] pub fn set_pid29<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid30(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if PID30 != 0"]
    #[inline] pub fn test_pid30(&self) -> bool {
        self.pid30() != 0
    }

    #[doc="Sets the PID30 field."]
    #[inline] pub fn set_pid30<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Port Input Disable"]
    #[inline] pub fn pid31(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if PID31 != 0"]
    #[inline] pub fn test_pid31(&self) -> bool {
        self.pid31() != 0
    }

    #[doc="Sets the PID31 field."]
    #[inline] pub fn set_pid31<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
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
        if self.pid0() != 0 { try!(write!(f, " pid0"))}
        if self.pid1() != 0 { try!(write!(f, " pid1"))}
        if self.pid2() != 0 { try!(write!(f, " pid2"))}
        if self.pid3() != 0 { try!(write!(f, " pid3"))}
        if self.pid4() != 0 { try!(write!(f, " pid4"))}
        if self.pid5() != 0 { try!(write!(f, " pid5"))}
        if self.pid6() != 0 { try!(write!(f, " pid6"))}
        if self.pid7() != 0 { try!(write!(f, " pid7"))}
        if self.pid8() != 0 { try!(write!(f, " pid8"))}
        if self.pid9() != 0 { try!(write!(f, " pid9"))}
        if self.pid10() != 0 { try!(write!(f, " pid10"))}
        if self.pid11() != 0 { try!(write!(f, " pid11"))}
        if self.pid12() != 0 { try!(write!(f, " pid12"))}
        if self.pid13() != 0 { try!(write!(f, " pid13"))}
        if self.pid14() != 0 { try!(write!(f, " pid14"))}
        if self.pid15() != 0 { try!(write!(f, " pid15"))}
        if self.pid16() != 0 { try!(write!(f, " pid16"))}
        if self.pid17() != 0 { try!(write!(f, " pid17"))}
        if self.pid18() != 0 { try!(write!(f, " pid18"))}
        if self.pid19() != 0 { try!(write!(f, " pid19"))}
        if self.pid20() != 0 { try!(write!(f, " pid20"))}
        if self.pid21() != 0 { try!(write!(f, " pid21"))}
        if self.pid22() != 0 { try!(write!(f, " pid22"))}
        if self.pid23() != 0 { try!(write!(f, " pid23"))}
        if self.pid24() != 0 { try!(write!(f, " pid24"))}
        if self.pid25() != 0 { try!(write!(f, " pid25"))}
        if self.pid26() != 0 { try!(write!(f, " pid26"))}
        if self.pid27() != 0 { try!(write!(f, " pid27"))}
        if self.pid28() != 0 { try!(write!(f, " pid28"))}
        if self.pid29() != 0 { try!(write!(f, " pid29"))}
        if self.pid30() != 0 { try!(write!(f, " pid30"))}
        if self.pid31() != 0 { try!(write!(f, " pid31"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

