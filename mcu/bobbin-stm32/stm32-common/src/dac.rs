
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="DAC Peripheral"]
pub struct DacPeriph(pub usize); 

pub struct DacCh { pub periph: DacPeriph, pub index: usize }

impl DacPeriph {
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

    #[doc="Get the SWTRIGR Register."]
    #[inline] pub fn swtrigr_reg(&self) -> ::bobbin_mcu::register::Register<Swtrigr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Swtrigr, 0x4)
    }

    #[doc="Get the *mut pointer for the SWTRIGR register."]
    #[inline] pub fn swtrigr_mut(&self) -> *mut Swtrigr { 
        self.swtrigr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SWTRIGR register."]
    #[inline] pub fn swtrigr_ptr(&self) -> *const Swtrigr { 
        self.swtrigr_reg().ptr()
    }

    #[doc="Write the SWTRIGR register."]
    #[inline] pub fn write_swtrigr(&self, value: Swtrigr) -> &Self { 
        self.swtrigr_reg().write(value);
        self
    }

    #[doc="Set the SWTRIGR register."]
    #[inline] pub fn set_swtrigr<F: FnOnce(Swtrigr) -> Swtrigr>(&self, f: F) -> &Self {
        self.swtrigr_reg().set(f);
        self
    }

    #[doc="Get the DHR12R Register."]
    #[inline] pub fn dhr12r_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dhr12r, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dhr12r, 0x8, 0xc)
    }

    #[doc="Get the *mut pointer for the DHR12R register."]
    #[inline] pub fn dhr12r_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dhr12r { 
        self.dhr12r_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DHR12R register."]
    #[inline] pub fn dhr12r_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dhr12r { 
        self.dhr12r_reg().ptr(index.into())
    }

    #[doc="Read the DHR12R register."]
    #[inline] pub fn dhr12r<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dhr12r { 
        self.dhr12r_reg().read(index.into())
    }

    #[doc="Write the DHR12R register."]
    #[inline] pub fn write_dhr12r<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dhr12r) -> &Self {
        self.dhr12r_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DHR12R register."]
    #[inline] pub fn set_dhr12r<I: Into<::bobbin_bits::R2>, F: FnOnce(Dhr12r) -> Dhr12r>(&self, index: I, f: F) -> &Self {
        self.dhr12r_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DHR12R register."]
    #[inline] pub fn with_dhr12r<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dhr12r) -> Dhr12r>(&self, index: I, f: F) -> &Self {
        self.dhr12r_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DHR12L Register."]
    #[inline] pub fn dhr12l_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dhr12l, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dhr12l, 0xc, 0xc)
    }

    #[doc="Get the *mut pointer for the DHR12L register."]
    #[inline] pub fn dhr12l_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dhr12l { 
        self.dhr12l_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DHR12L register."]
    #[inline] pub fn dhr12l_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dhr12l { 
        self.dhr12l_reg().ptr(index.into())
    }

    #[doc="Read the DHR12L register."]
    #[inline] pub fn dhr12l<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dhr12l { 
        self.dhr12l_reg().read(index.into())
    }

    #[doc="Write the DHR12L register."]
    #[inline] pub fn write_dhr12l<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dhr12l) -> &Self {
        self.dhr12l_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DHR12L register."]
    #[inline] pub fn set_dhr12l<I: Into<::bobbin_bits::R2>, F: FnOnce(Dhr12l) -> Dhr12l>(&self, index: I, f: F) -> &Self {
        self.dhr12l_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DHR12L register."]
    #[inline] pub fn with_dhr12l<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dhr12l) -> Dhr12l>(&self, index: I, f: F) -> &Self {
        self.dhr12l_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DHR8R Register."]
    #[inline] pub fn dhr8r_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dhr8r, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dhr8r, 0x10, 0xc)
    }

    #[doc="Get the *mut pointer for the DHR8R register."]
    #[inline] pub fn dhr8r_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dhr8r { 
        self.dhr8r_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DHR8R register."]
    #[inline] pub fn dhr8r_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dhr8r { 
        self.dhr8r_reg().ptr(index.into())
    }

    #[doc="Read the DHR8R register."]
    #[inline] pub fn dhr8r<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dhr8r { 
        self.dhr8r_reg().read(index.into())
    }

    #[doc="Write the DHR8R register."]
    #[inline] pub fn write_dhr8r<I: Into<::bobbin_bits::R2>>(&self, index: I, value: Dhr8r) -> &Self {
        self.dhr8r_reg().write(index.into(), value);
        self
    }

    #[doc="Set the DHR8R register."]
    #[inline] pub fn set_dhr8r<I: Into<::bobbin_bits::R2>, F: FnOnce(Dhr8r) -> Dhr8r>(&self, index: I, f: F) -> &Self {
        self.dhr8r_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the DHR8R register."]
    #[inline] pub fn with_dhr8r<I: Into<::bobbin_bits::R2> + Copy, F: FnOnce(Dhr8r) -> Dhr8r>(&self, index: I, f: F) -> &Self {
        self.dhr8r_reg().with(index.into(), f);
        self
    }

    #[doc="Get the DHR12RD Register."]
    #[inline] pub fn dhr12rd_reg(&self) -> ::bobbin_mcu::register::Register<Dhr12rd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhr12rd, 0x20)
    }

    #[doc="Get the *mut pointer for the DHR12RD register."]
    #[inline] pub fn dhr12rd_mut(&self) -> *mut Dhr12rd { 
        self.dhr12rd_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHR12RD register."]
    #[inline] pub fn dhr12rd_ptr(&self) -> *const Dhr12rd { 
        self.dhr12rd_reg().ptr()
    }

    #[doc="Read the DHR12RD register."]
    #[inline] pub fn dhr12rd(&self) -> Dhr12rd { 
        self.dhr12rd_reg().read()
    }

    #[doc="Write the DHR12RD register."]
    #[inline] pub fn write_dhr12rd(&self, value: Dhr12rd) -> &Self { 
        self.dhr12rd_reg().write(value);
        self
    }

    #[doc="Set the DHR12RD register."]
    #[inline] pub fn set_dhr12rd<F: FnOnce(Dhr12rd) -> Dhr12rd>(&self, f: F) -> &Self {
        self.dhr12rd_reg().set(f);
        self
    }

    #[doc="Modify the DHR12RD register."]
    #[inline] pub fn with_dhr12rd<F: FnOnce(Dhr12rd) -> Dhr12rd>(&self, f: F) -> &Self {
        self.dhr12rd_reg().with(f);
        self
    }

    #[doc="Get the DHR12LD Register."]
    #[inline] pub fn dhr12ld_reg(&self) -> ::bobbin_mcu::register::Register<Dhr12ld> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhr12ld, 0x24)
    }

    #[doc="Get the *mut pointer for the DHR12LD register."]
    #[inline] pub fn dhr12ld_mut(&self) -> *mut Dhr12ld { 
        self.dhr12ld_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHR12LD register."]
    #[inline] pub fn dhr12ld_ptr(&self) -> *const Dhr12ld { 
        self.dhr12ld_reg().ptr()
    }

    #[doc="Read the DHR12LD register."]
    #[inline] pub fn dhr12ld(&self) -> Dhr12ld { 
        self.dhr12ld_reg().read()
    }

    #[doc="Write the DHR12LD register."]
    #[inline] pub fn write_dhr12ld(&self, value: Dhr12ld) -> &Self { 
        self.dhr12ld_reg().write(value);
        self
    }

    #[doc="Set the DHR12LD register."]
    #[inline] pub fn set_dhr12ld<F: FnOnce(Dhr12ld) -> Dhr12ld>(&self, f: F) -> &Self {
        self.dhr12ld_reg().set(f);
        self
    }

    #[doc="Modify the DHR12LD register."]
    #[inline] pub fn with_dhr12ld<F: FnOnce(Dhr12ld) -> Dhr12ld>(&self, f: F) -> &Self {
        self.dhr12ld_reg().with(f);
        self
    }

    #[doc="Get the DHR8RD Register."]
    #[inline] pub fn dhr8rd_reg(&self) -> ::bobbin_mcu::register::Register<Dhr8rd> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Dhr8rd, 0x28)
    }

    #[doc="Get the *mut pointer for the DHR8RD register."]
    #[inline] pub fn dhr8rd_mut(&self) -> *mut Dhr8rd { 
        self.dhr8rd_reg().ptr()
    }

    #[doc="Get the *const pointer for the DHR8RD register."]
    #[inline] pub fn dhr8rd_ptr(&self) -> *const Dhr8rd { 
        self.dhr8rd_reg().ptr()
    }

    #[doc="Read the DHR8RD register."]
    #[inline] pub fn dhr8rd(&self) -> Dhr8rd { 
        self.dhr8rd_reg().read()
    }

    #[doc="Write the DHR8RD register."]
    #[inline] pub fn write_dhr8rd(&self, value: Dhr8rd) -> &Self { 
        self.dhr8rd_reg().write(value);
        self
    }

    #[doc="Set the DHR8RD register."]
    #[inline] pub fn set_dhr8rd<F: FnOnce(Dhr8rd) -> Dhr8rd>(&self, f: F) -> &Self {
        self.dhr8rd_reg().set(f);
        self
    }

    #[doc="Modify the DHR8RD register."]
    #[inline] pub fn with_dhr8rd<F: FnOnce(Dhr8rd) -> Dhr8rd>(&self, f: F) -> &Self {
        self.dhr8rd_reg().with(f);
        self
    }

    #[doc="Get the DOR Register."]
    #[inline] pub fn dor_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Dor, ::bobbin_bits::R2> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Dor, 0x2c, 0x4)
    }

    #[doc="Get the *mut pointer for the DOR register."]
    #[inline] pub fn dor_mut<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *mut Dor { 
        self.dor_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the DOR register."]
    #[inline] pub fn dor_ptr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> *const Dor { 
        self.dor_reg().ptr(index.into())
    }

    #[doc="Read the DOR register."]
    #[inline] pub fn dor<I: Into<::bobbin_bits::R2>>(&self, index: I) -> Dor { 
        self.dor_reg().read(index.into())
    }

    #[doc="Get the SR Register."]
    #[inline] pub fn sr_reg(&self) -> ::bobbin_mcu::register::Register<Sr> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sr, 0x34)
    }

    #[doc="Get the *mut pointer for the SR register."]
    #[inline] pub fn sr_mut(&self) -> *mut Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Get the *const pointer for the SR register."]
    #[inline] pub fn sr_ptr(&self) -> *const Sr { 
        self.sr_reg().ptr()
    }

    #[doc="Read the SR register."]
    #[inline] pub fn sr(&self) -> Sr { 
        self.sr_reg().read()
    }

    #[doc="Write the SR register."]
    #[inline] pub fn write_sr(&self, value: Sr) -> &Self { 
        self.sr_reg().write(value);
        self
    }

    #[doc="Set the SR register."]
    #[inline] pub fn set_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().set(f);
        self
    }

    #[doc="Modify the SR register."]
    #[inline] pub fn with_sr<F: FnOnce(Sr) -> Sr>(&self, f: F) -> &Self {
        self.sr_reg().with(f);
        self
    }

}

#[doc="control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc="DAC channel DMA Underrun Interrupt enable"]
    #[inline] pub fn dmaudrie<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 13 + (index * 16);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DMAUDRIE != 0"]
    #[inline] pub fn test_dmaudrie<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.dmaudrie(index) != 0
    }

    #[doc="Sets the DMAUDRIE field."]
    #[inline] pub fn set_dmaudrie<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 13 + (index * 16);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="DAC channel DMA enable"]
    #[inline] pub fn dmaen<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 12 + (index * 16);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DMAEN != 0"]
    #[inline] pub fn test_dmaen<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.dmaen(index) != 0
    }

    #[doc="Sets the DMAEN field."]
    #[inline] pub fn set_dmaen<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 12 + (index * 16);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="DAC channel mask/amplitude selector"]
    #[inline] pub fn mamp<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U4 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 8 + (index * 16);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if MAMP != 0"]
    #[inline] pub fn test_mamp<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.mamp(index) != 0
    }

    #[doc="Sets the MAMP field."]
    #[inline] pub fn set_mamp<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U4>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        let shift: usize = 8 + (index * 16);
        self.0 &= !(0xf << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="DAC channel noise/triangle wave generation enable"]
    #[inline] pub fn wave<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U2 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 6 + (index * 16);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if WAVE != 0"]
    #[inline] pub fn test_wave<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.wave(index) != 0
    }

    #[doc="Sets the WAVE field."]
    #[inline] pub fn set_wave<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U2>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        let shift: usize = 6 + (index * 16);
        self.0 &= !(0x3 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="DAC channel trigger selection"]
    #[inline] pub fn tsel<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U3 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 3 + (index * 16);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x7) as u8) } // [5:3]
    }

    #[doc="Returns true if TSEL != 0"]
    #[inline] pub fn test_tsel<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.tsel(index) != 0
    }

    #[doc="Sets the TSEL field."]
    #[inline] pub fn set_tsel<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U3>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U3 = value.into();
        let value: u32 = value.into();
        let shift: usize = 3 + (index * 16);
        self.0 &= !(0x7 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="DAC channel trigger enable"]
    #[inline] pub fn ten<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 2 + (index * 16);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TEN != 0"]
    #[inline] pub fn test_ten<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.ten(index) != 0
    }

    #[doc="Sets the TEN field."]
    #[inline] pub fn set_ten<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 2 + (index * 16);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="DAC channel output buffer disable"]
    #[inline] pub fn boff<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 1 + (index * 16);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if BOFF != 0"]
    #[inline] pub fn test_boff<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.boff(index) != 0
    }

    #[doc="Sets the BOFF field."]
    #[inline] pub fn set_boff<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 1 + (index * 16);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

    #[doc="DAC channel enable"]
    #[inline] pub fn en<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index * 16);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EN != 0"]
    #[inline] pub fn test_en<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.en(index) != 0
    }

    #[doc="Sets the EN field."]
    #[inline] pub fn set_en<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index * 16);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
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
        if self.dmaudrie(0) != 0 { try!(write!(f, " dmaudrie[0]"))}
        if self.dmaudrie(1) != 0 { try!(write!(f, " dmaudrie[1]"))}
        if self.dmaen(0) != 0 { try!(write!(f, " dmaen[0]"))}
        if self.dmaen(1) != 0 { try!(write!(f, " dmaen[1]"))}
        if self.mamp(0) != 0 { try!(write!(f, " mamp[0]=0x{:x}", self.mamp(0)))}
        if self.mamp(1) != 0 { try!(write!(f, " mamp[1]=0x{:x}", self.mamp(1)))}
        if self.wave(0) != 0 { try!(write!(f, " wave[0]=0x{:x}", self.wave(0)))}
        if self.wave(1) != 0 { try!(write!(f, " wave[1]=0x{:x}", self.wave(1)))}
        if self.tsel(0) != 0 { try!(write!(f, " tsel[0]=0x{:x}", self.tsel(0)))}
        if self.tsel(1) != 0 { try!(write!(f, " tsel[1]=0x{:x}", self.tsel(1)))}
        if self.ten(0) != 0 { try!(write!(f, " ten[0]"))}
        if self.ten(1) != 0 { try!(write!(f, " ten[1]"))}
        if self.boff(0) != 0 { try!(write!(f, " boff[0]"))}
        if self.boff(1) != 0 { try!(write!(f, " boff[1]"))}
        if self.en(0) != 0 { try!(write!(f, " en[0]"))}
        if self.en(1) != 0 { try!(write!(f, " en[1]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="software trigger register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swtrigr(pub u32);
impl Swtrigr {
    #[doc="DAC channel software trigger"]
    #[inline] pub fn swtrig<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if SWTRIG != 0"]
    #[inline] pub fn test_swtrig<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.swtrig(index) != 0
    }

    #[doc="Sets the SWTRIG field."]
    #[inline] pub fn set_swtrig<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Swtrigr {
    #[inline]
    fn from(other: u32) -> Self {
         Swtrigr(other)
    }
}

impl ::core::fmt::Display for Swtrigr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swtrigr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.swtrig(0) != 0 { try!(write!(f, " swtrig[0]"))}
        if self.swtrig(1) != 0 { try!(write!(f, " swtrig[1]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel 12-bit right-aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr12r(pub u32);
impl Dhr12r {
    #[doc="DAC channel 12-bit right-aligned data"]
    #[inline] pub fn daccdhr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DACCDHR != 0"]
    #[inline] pub fn test_daccdhr(&self) -> bool {
        self.daccdhr() != 0
    }

    #[doc="Sets the DACCDHR field."]
    #[inline] pub fn set_daccdhr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dhr12r {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr12r(other)
    }
}

impl ::core::fmt::Display for Dhr12r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr12r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.daccdhr() != 0 { try!(write!(f, " daccdhr=0x{:x}", self.daccdhr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel 12-bit left aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr12l(pub u32);
impl Dhr12l {
    #[doc="DAC channel 12-bit left-aligned data"]
    #[inline] pub fn daccdhr(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xfff) as u16) } // [15:4]
    }

    #[doc="Returns true if DACCDHR != 0"]
    #[inline] pub fn test_daccdhr(&self) -> bool {
        self.daccdhr() != 0
    }

    #[doc="Sets the DACCDHR field."]
    #[inline] pub fn set_daccdhr<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 4);
        self.0 |= value << 4;
        self
    }

}

impl From<u32> for Dhr12l {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr12l(other)
    }
}

impl ::core::fmt::Display for Dhr12l {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr12l {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.daccdhr() != 0 { try!(write!(f, " daccdhr=0x{:x}", self.daccdhr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel 8-bit right aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr8r(pub u32);
impl Dhr8r {
    #[doc="DAC channel 8-bit right-aligned data"]
    #[inline] pub fn daccdhr(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DACCDHR != 0"]
    #[inline] pub fn test_daccdhr(&self) -> bool {
        self.daccdhr() != 0
    }

    #[doc="Sets the DACCDHR field."]
    #[inline] pub fn set_daccdhr<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dhr8r {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr8r(other)
    }
}

impl ::core::fmt::Display for Dhr8r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr8r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.daccdhr() != 0 { try!(write!(f, " daccdhr=0x{:x}", self.daccdhr()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Dual DAC 12-bit right-aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr12rd(pub u32);
impl Dhr12rd {
    #[doc="DAC channel 12-bit right-aligned data"]
    #[inline] pub fn daccdhr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U12 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index * 16);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DACCDHR != 0"]
    #[inline] pub fn test_daccdhr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.daccdhr(index) != 0
    }

    #[doc="Sets the DACCDHR field."]
    #[inline] pub fn set_daccdhr<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U12>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index * 16);
        self.0 &= !(0xfff << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dhr12rd {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr12rd(other)
    }
}

impl ::core::fmt::Display for Dhr12rd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr12rd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.daccdhr(0) != 0 { try!(write!(f, " daccdhr[0]=0x{:x}", self.daccdhr(0)))}
        if self.daccdhr(1) != 0 { try!(write!(f, " daccdhr[1]=0x{:x}", self.daccdhr(1)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DUAL DAC 12-bit left aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr12ld(pub u32);
impl Dhr12ld {
    #[doc="DAC channel 12-bit left-aligned data"]
    #[inline] pub fn daccdhr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U12 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 4 + (index * 16);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xfff) as u16) } // [15:4]
    }

    #[doc="Returns true if DACCDHR != 0"]
    #[inline] pub fn test_daccdhr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.daccdhr(index) != 0
    }

    #[doc="Sets the DACCDHR field."]
    #[inline] pub fn set_daccdhr<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U12>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        let shift: usize = 4 + (index * 16);
        self.0 &= !(0xfff << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dhr12ld {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr12ld(other)
    }
}

impl ::core::fmt::Display for Dhr12ld {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr12ld {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.daccdhr(0) != 0 { try!(write!(f, " daccdhr[0]=0x{:x}", self.daccdhr(0)))}
        if self.daccdhr(1) != 0 { try!(write!(f, " daccdhr[1]=0x{:x}", self.daccdhr(1)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="DUAL DAC 8-bit right aligned data holding register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dhr8rd(pub u32);
impl Dhr8rd {
    #[doc="DAC channel 8-bit right-aligned data"]
    #[inline] pub fn daccdhr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U8 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + (index << 3);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DACCDHR != 0"]
    #[inline] pub fn test_daccdhr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.daccdhr(index) != 0
    }

    #[doc="Sets the DACCDHR field."]
    #[inline] pub fn set_daccdhr<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U8>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + (index << 3);
        self.0 &= !(0xff << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Dhr8rd {
    #[inline]
    fn from(other: u32) -> Self {
         Dhr8rd(other)
    }
}

impl ::core::fmt::Display for Dhr8rd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dhr8rd {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.daccdhr(0) != 0 { try!(write!(f, " daccdhr[0]=0x{:x}", self.daccdhr(0)))}
        if self.daccdhr(1) != 0 { try!(write!(f, " daccdhr[1]=0x{:x}", self.daccdhr(1)))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="channel data output register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Dor(pub u32);
impl Dor {
    #[doc="DAC channel data output"]
    #[inline] pub fn daccdor(&self) -> ::bobbin_bits::U12 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xfff) as u16) } // [11:0]
    }

    #[doc="Returns true if DACCDOR != 0"]
    #[inline] pub fn test_daccdor(&self) -> bool {
        self.daccdor() != 0
    }

    #[doc="Sets the DACCDOR field."]
    #[inline] pub fn set_daccdor<V: Into<::bobbin_bits::U12>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U12 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xfff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Dor {
    #[inline]
    fn from(other: u32) -> Self {
         Dor(other)
    }
}

impl ::core::fmt::Display for Dor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Dor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.daccdor() != 0 { try!(write!(f, " daccdor=0x{:x}", self.daccdor()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc="DAC channel DMA underrun flag"]
    #[inline] pub fn dmaudr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> ::bobbin_bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 13 + (index * 16);
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if DMAUDR != 0"]
    #[inline] pub fn test_dmaudr<I: Into<::bobbin_bits::R2>>(&self, index: I) -> bool{
        self.dmaudr(index) != 0
    }

    #[doc="Sets the DMAUDR field."]
    #[inline] pub fn set_dmaudr<I: Into<::bobbin_bits::R2>, V: Into<::bobbin_bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 13 + (index * 16);
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Sr {
    #[inline]
    fn from(other: u32) -> Self {
         Sr(other)
    }
}

impl ::core::fmt::Display for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dmaudr(0) != 0 { try!(write!(f, " dmaudr[0]"))}
        if self.dmaudr(1) != 0 { try!(write!(f, " dmaudr[1]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

