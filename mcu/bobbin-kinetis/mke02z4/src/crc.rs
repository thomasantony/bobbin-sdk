::bobbin_mcu::periph!( CRC, Crc, CRC_PERIPH, CrcPeriph, CRC_OWNED, CRC_REF_COUNT, 0x40032000, 0x00, 0x02);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="CRC Peripheral"]
pub struct CrcPeriph(pub usize); 

impl CrcPeriph {
    #[doc="Get the DATA Register."]
    #[inline] pub fn data_reg(&self) -> ::bobbin_mcu::register::Register<Data> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Data, 0x0)
    }

    #[doc="Get the *mut pointer for the DATA register."]
    #[inline] pub fn data_mut(&self) -> *mut Data { 
        self.data_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATA register."]
    #[inline] pub fn data_ptr(&self) -> *const Data { 
        self.data_reg().ptr()
    }

    #[doc="Read the DATA register."]
    #[inline] pub fn data(&self) -> Data { 
        self.data_reg().read()
    }

    #[doc="Write the DATA register."]
    #[inline] pub fn write_data(&self, value: Data) -> &Self { 
        self.data_reg().write(value);
        self
    }

    #[doc="Set the DATA register."]
    #[inline] pub fn set_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().set(f);
        self
    }

    #[doc="Modify the DATA register."]
    #[inline] pub fn with_data<F: FnOnce(Data) -> Data>(&self, f: F) -> &Self {
        self.data_reg().with(f);
        self
    }

    #[doc="Get the DATAL Register."]
    #[inline] pub fn datal_reg(&self) -> ::bobbin_mcu::register::Register<Datal> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Datal, 0x0)
    }

    #[doc="Get the *mut pointer for the DATAL register."]
    #[inline] pub fn datal_mut(&self) -> *mut Datal { 
        self.datal_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATAL register."]
    #[inline] pub fn datal_ptr(&self) -> *const Datal { 
        self.datal_reg().ptr()
    }

    #[doc="Read the DATAL register."]
    #[inline] pub fn datal(&self) -> Datal { 
        self.datal_reg().read()
    }

    #[doc="Write the DATAL register."]
    #[inline] pub fn write_datal(&self, value: Datal) -> &Self { 
        self.datal_reg().write(value);
        self
    }

    #[doc="Set the DATAL register."]
    #[inline] pub fn set_datal<F: FnOnce(Datal) -> Datal>(&self, f: F) -> &Self {
        self.datal_reg().set(f);
        self
    }

    #[doc="Modify the DATAL register."]
    #[inline] pub fn with_datal<F: FnOnce(Datal) -> Datal>(&self, f: F) -> &Self {
        self.datal_reg().with(f);
        self
    }

    #[doc="Get the DATALL Register."]
    #[inline] pub fn datall_reg(&self) -> ::bobbin_mcu::register::Register<Datall> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Datall, 0x0)
    }

    #[doc="Get the *mut pointer for the DATALL register."]
    #[inline] pub fn datall_mut(&self) -> *mut Datall { 
        self.datall_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATALL register."]
    #[inline] pub fn datall_ptr(&self) -> *const Datall { 
        self.datall_reg().ptr()
    }

    #[doc="Read the DATALL register."]
    #[inline] pub fn datall(&self) -> Datall { 
        self.datall_reg().read()
    }

    #[doc="Write the DATALL register."]
    #[inline] pub fn write_datall(&self, value: Datall) -> &Self { 
        self.datall_reg().write(value);
        self
    }

    #[doc="Set the DATALL register."]
    #[inline] pub fn set_datall<F: FnOnce(Datall) -> Datall>(&self, f: F) -> &Self {
        self.datall_reg().set(f);
        self
    }

    #[doc="Modify the DATALL register."]
    #[inline] pub fn with_datall<F: FnOnce(Datall) -> Datall>(&self, f: F) -> &Self {
        self.datall_reg().with(f);
        self
    }

    #[doc="Get the DATALU Register."]
    #[inline] pub fn datalu_reg(&self) -> ::bobbin_mcu::register::Register<Datalu> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Datalu, 0x1)
    }

    #[doc="Get the *mut pointer for the DATALU register."]
    #[inline] pub fn datalu_mut(&self) -> *mut Datalu { 
        self.datalu_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATALU register."]
    #[inline] pub fn datalu_ptr(&self) -> *const Datalu { 
        self.datalu_reg().ptr()
    }

    #[doc="Read the DATALU register."]
    #[inline] pub fn datalu(&self) -> Datalu { 
        self.datalu_reg().read()
    }

    #[doc="Write the DATALU register."]
    #[inline] pub fn write_datalu(&self, value: Datalu) -> &Self { 
        self.datalu_reg().write(value);
        self
    }

    #[doc="Set the DATALU register."]
    #[inline] pub fn set_datalu<F: FnOnce(Datalu) -> Datalu>(&self, f: F) -> &Self {
        self.datalu_reg().set(f);
        self
    }

    #[doc="Modify the DATALU register."]
    #[inline] pub fn with_datalu<F: FnOnce(Datalu) -> Datalu>(&self, f: F) -> &Self {
        self.datalu_reg().with(f);
        self
    }

    #[doc="Get the DATAH Register."]
    #[inline] pub fn datah_reg(&self) -> ::bobbin_mcu::register::Register<Datah> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Datah, 0x2)
    }

    #[doc="Get the *mut pointer for the DATAH register."]
    #[inline] pub fn datah_mut(&self) -> *mut Datah { 
        self.datah_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATAH register."]
    #[inline] pub fn datah_ptr(&self) -> *const Datah { 
        self.datah_reg().ptr()
    }

    #[doc="Read the DATAH register."]
    #[inline] pub fn datah(&self) -> Datah { 
        self.datah_reg().read()
    }

    #[doc="Write the DATAH register."]
    #[inline] pub fn write_datah(&self, value: Datah) -> &Self { 
        self.datah_reg().write(value);
        self
    }

    #[doc="Set the DATAH register."]
    #[inline] pub fn set_datah<F: FnOnce(Datah) -> Datah>(&self, f: F) -> &Self {
        self.datah_reg().set(f);
        self
    }

    #[doc="Modify the DATAH register."]
    #[inline] pub fn with_datah<F: FnOnce(Datah) -> Datah>(&self, f: F) -> &Self {
        self.datah_reg().with(f);
        self
    }

    #[doc="Get the DATAHL Register."]
    #[inline] pub fn datahl_reg(&self) -> ::bobbin_mcu::register::Register<Datahl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Datahl, 0x2)
    }

    #[doc="Get the *mut pointer for the DATAHL register."]
    #[inline] pub fn datahl_mut(&self) -> *mut Datahl { 
        self.datahl_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATAHL register."]
    #[inline] pub fn datahl_ptr(&self) -> *const Datahl { 
        self.datahl_reg().ptr()
    }

    #[doc="Read the DATAHL register."]
    #[inline] pub fn datahl(&self) -> Datahl { 
        self.datahl_reg().read()
    }

    #[doc="Write the DATAHL register."]
    #[inline] pub fn write_datahl(&self, value: Datahl) -> &Self { 
        self.datahl_reg().write(value);
        self
    }

    #[doc="Set the DATAHL register."]
    #[inline] pub fn set_datahl<F: FnOnce(Datahl) -> Datahl>(&self, f: F) -> &Self {
        self.datahl_reg().set(f);
        self
    }

    #[doc="Modify the DATAHL register."]
    #[inline] pub fn with_datahl<F: FnOnce(Datahl) -> Datahl>(&self, f: F) -> &Self {
        self.datahl_reg().with(f);
        self
    }

    #[doc="Get the DATAHU Register."]
    #[inline] pub fn datahu_reg(&self) -> ::bobbin_mcu::register::Register<Datahu> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Datahu, 0x3)
    }

    #[doc="Get the *mut pointer for the DATAHU register."]
    #[inline] pub fn datahu_mut(&self) -> *mut Datahu { 
        self.datahu_reg().ptr()
    }

    #[doc="Get the *const pointer for the DATAHU register."]
    #[inline] pub fn datahu_ptr(&self) -> *const Datahu { 
        self.datahu_reg().ptr()
    }

    #[doc="Read the DATAHU register."]
    #[inline] pub fn datahu(&self) -> Datahu { 
        self.datahu_reg().read()
    }

    #[doc="Write the DATAHU register."]
    #[inline] pub fn write_datahu(&self, value: Datahu) -> &Self { 
        self.datahu_reg().write(value);
        self
    }

    #[doc="Set the DATAHU register."]
    #[inline] pub fn set_datahu<F: FnOnce(Datahu) -> Datahu>(&self, f: F) -> &Self {
        self.datahu_reg().set(f);
        self
    }

    #[doc="Modify the DATAHU register."]
    #[inline] pub fn with_datahu<F: FnOnce(Datahu) -> Datahu>(&self, f: F) -> &Self {
        self.datahu_reg().with(f);
        self
    }

    #[doc="Get the GPOLY Register."]
    #[inline] pub fn gpoly_reg(&self) -> ::bobbin_mcu::register::Register<Gpoly> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gpoly, 0x4)
    }

    #[doc="Get the *mut pointer for the GPOLY register."]
    #[inline] pub fn gpoly_mut(&self) -> *mut Gpoly { 
        self.gpoly_reg().ptr()
    }

    #[doc="Get the *const pointer for the GPOLY register."]
    #[inline] pub fn gpoly_ptr(&self) -> *const Gpoly { 
        self.gpoly_reg().ptr()
    }

    #[doc="Read the GPOLY register."]
    #[inline] pub fn gpoly(&self) -> Gpoly { 
        self.gpoly_reg().read()
    }

    #[doc="Write the GPOLY register."]
    #[inline] pub fn write_gpoly(&self, value: Gpoly) -> &Self { 
        self.gpoly_reg().write(value);
        self
    }

    #[doc="Set the GPOLY register."]
    #[inline] pub fn set_gpoly<F: FnOnce(Gpoly) -> Gpoly>(&self, f: F) -> &Self {
        self.gpoly_reg().set(f);
        self
    }

    #[doc="Modify the GPOLY register."]
    #[inline] pub fn with_gpoly<F: FnOnce(Gpoly) -> Gpoly>(&self, f: F) -> &Self {
        self.gpoly_reg().with(f);
        self
    }

    #[doc="Get the GPOLYL Register."]
    #[inline] pub fn gpolyl_reg(&self) -> ::bobbin_mcu::register::Register<Gpolyl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gpolyl, 0x4)
    }

    #[doc="Get the *mut pointer for the GPOLYL register."]
    #[inline] pub fn gpolyl_mut(&self) -> *mut Gpolyl { 
        self.gpolyl_reg().ptr()
    }

    #[doc="Get the *const pointer for the GPOLYL register."]
    #[inline] pub fn gpolyl_ptr(&self) -> *const Gpolyl { 
        self.gpolyl_reg().ptr()
    }

    #[doc="Read the GPOLYL register."]
    #[inline] pub fn gpolyl(&self) -> Gpolyl { 
        self.gpolyl_reg().read()
    }

    #[doc="Write the GPOLYL register."]
    #[inline] pub fn write_gpolyl(&self, value: Gpolyl) -> &Self { 
        self.gpolyl_reg().write(value);
        self
    }

    #[doc="Set the GPOLYL register."]
    #[inline] pub fn set_gpolyl<F: FnOnce(Gpolyl) -> Gpolyl>(&self, f: F) -> &Self {
        self.gpolyl_reg().set(f);
        self
    }

    #[doc="Modify the GPOLYL register."]
    #[inline] pub fn with_gpolyl<F: FnOnce(Gpolyl) -> Gpolyl>(&self, f: F) -> &Self {
        self.gpolyl_reg().with(f);
        self
    }

    #[doc="Get the GPOLYLL Register."]
    #[inline] pub fn gpolyll_reg(&self) -> ::bobbin_mcu::register::Register<Gpolyll> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gpolyll, 0x4)
    }

    #[doc="Get the *mut pointer for the GPOLYLL register."]
    #[inline] pub fn gpolyll_mut(&self) -> *mut Gpolyll { 
        self.gpolyll_reg().ptr()
    }

    #[doc="Get the *const pointer for the GPOLYLL register."]
    #[inline] pub fn gpolyll_ptr(&self) -> *const Gpolyll { 
        self.gpolyll_reg().ptr()
    }

    #[doc="Read the GPOLYLL register."]
    #[inline] pub fn gpolyll(&self) -> Gpolyll { 
        self.gpolyll_reg().read()
    }

    #[doc="Write the GPOLYLL register."]
    #[inline] pub fn write_gpolyll(&self, value: Gpolyll) -> &Self { 
        self.gpolyll_reg().write(value);
        self
    }

    #[doc="Set the GPOLYLL register."]
    #[inline] pub fn set_gpolyll<F: FnOnce(Gpolyll) -> Gpolyll>(&self, f: F) -> &Self {
        self.gpolyll_reg().set(f);
        self
    }

    #[doc="Modify the GPOLYLL register."]
    #[inline] pub fn with_gpolyll<F: FnOnce(Gpolyll) -> Gpolyll>(&self, f: F) -> &Self {
        self.gpolyll_reg().with(f);
        self
    }

    #[doc="Get the GPOLYLU Register."]
    #[inline] pub fn gpolylu_reg(&self) -> ::bobbin_mcu::register::Register<Gpolylu> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gpolylu, 0x5)
    }

    #[doc="Get the *mut pointer for the GPOLYLU register."]
    #[inline] pub fn gpolylu_mut(&self) -> *mut Gpolylu { 
        self.gpolylu_reg().ptr()
    }

    #[doc="Get the *const pointer for the GPOLYLU register."]
    #[inline] pub fn gpolylu_ptr(&self) -> *const Gpolylu { 
        self.gpolylu_reg().ptr()
    }

    #[doc="Read the GPOLYLU register."]
    #[inline] pub fn gpolylu(&self) -> Gpolylu { 
        self.gpolylu_reg().read()
    }

    #[doc="Write the GPOLYLU register."]
    #[inline] pub fn write_gpolylu(&self, value: Gpolylu) -> &Self { 
        self.gpolylu_reg().write(value);
        self
    }

    #[doc="Set the GPOLYLU register."]
    #[inline] pub fn set_gpolylu<F: FnOnce(Gpolylu) -> Gpolylu>(&self, f: F) -> &Self {
        self.gpolylu_reg().set(f);
        self
    }

    #[doc="Modify the GPOLYLU register."]
    #[inline] pub fn with_gpolylu<F: FnOnce(Gpolylu) -> Gpolylu>(&self, f: F) -> &Self {
        self.gpolylu_reg().with(f);
        self
    }

    #[doc="Get the GPOLYH Register."]
    #[inline] pub fn gpolyh_reg(&self) -> ::bobbin_mcu::register::Register<Gpolyh> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gpolyh, 0x6)
    }

    #[doc="Get the *mut pointer for the GPOLYH register."]
    #[inline] pub fn gpolyh_mut(&self) -> *mut Gpolyh { 
        self.gpolyh_reg().ptr()
    }

    #[doc="Get the *const pointer for the GPOLYH register."]
    #[inline] pub fn gpolyh_ptr(&self) -> *const Gpolyh { 
        self.gpolyh_reg().ptr()
    }

    #[doc="Read the GPOLYH register."]
    #[inline] pub fn gpolyh(&self) -> Gpolyh { 
        self.gpolyh_reg().read()
    }

    #[doc="Write the GPOLYH register."]
    #[inline] pub fn write_gpolyh(&self, value: Gpolyh) -> &Self { 
        self.gpolyh_reg().write(value);
        self
    }

    #[doc="Set the GPOLYH register."]
    #[inline] pub fn set_gpolyh<F: FnOnce(Gpolyh) -> Gpolyh>(&self, f: F) -> &Self {
        self.gpolyh_reg().set(f);
        self
    }

    #[doc="Modify the GPOLYH register."]
    #[inline] pub fn with_gpolyh<F: FnOnce(Gpolyh) -> Gpolyh>(&self, f: F) -> &Self {
        self.gpolyh_reg().with(f);
        self
    }

    #[doc="Get the GPOLYHL Register."]
    #[inline] pub fn gpolyhl_reg(&self) -> ::bobbin_mcu::register::Register<Gpolyhl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gpolyhl, 0x6)
    }

    #[doc="Get the *mut pointer for the GPOLYHL register."]
    #[inline] pub fn gpolyhl_mut(&self) -> *mut Gpolyhl { 
        self.gpolyhl_reg().ptr()
    }

    #[doc="Get the *const pointer for the GPOLYHL register."]
    #[inline] pub fn gpolyhl_ptr(&self) -> *const Gpolyhl { 
        self.gpolyhl_reg().ptr()
    }

    #[doc="Read the GPOLYHL register."]
    #[inline] pub fn gpolyhl(&self) -> Gpolyhl { 
        self.gpolyhl_reg().read()
    }

    #[doc="Write the GPOLYHL register."]
    #[inline] pub fn write_gpolyhl(&self, value: Gpolyhl) -> &Self { 
        self.gpolyhl_reg().write(value);
        self
    }

    #[doc="Set the GPOLYHL register."]
    #[inline] pub fn set_gpolyhl<F: FnOnce(Gpolyhl) -> Gpolyhl>(&self, f: F) -> &Self {
        self.gpolyhl_reg().set(f);
        self
    }

    #[doc="Modify the GPOLYHL register."]
    #[inline] pub fn with_gpolyhl<F: FnOnce(Gpolyhl) -> Gpolyhl>(&self, f: F) -> &Self {
        self.gpolyhl_reg().with(f);
        self
    }

    #[doc="Get the GPOLYHU Register."]
    #[inline] pub fn gpolyhu_reg(&self) -> ::bobbin_mcu::register::Register<Gpolyhu> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Gpolyhu, 0x7)
    }

    #[doc="Get the *mut pointer for the GPOLYHU register."]
    #[inline] pub fn gpolyhu_mut(&self) -> *mut Gpolyhu { 
        self.gpolyhu_reg().ptr()
    }

    #[doc="Get the *const pointer for the GPOLYHU register."]
    #[inline] pub fn gpolyhu_ptr(&self) -> *const Gpolyhu { 
        self.gpolyhu_reg().ptr()
    }

    #[doc="Read the GPOLYHU register."]
    #[inline] pub fn gpolyhu(&self) -> Gpolyhu { 
        self.gpolyhu_reg().read()
    }

    #[doc="Write the GPOLYHU register."]
    #[inline] pub fn write_gpolyhu(&self, value: Gpolyhu) -> &Self { 
        self.gpolyhu_reg().write(value);
        self
    }

    #[doc="Set the GPOLYHU register."]
    #[inline] pub fn set_gpolyhu<F: FnOnce(Gpolyhu) -> Gpolyhu>(&self, f: F) -> &Self {
        self.gpolyhu_reg().set(f);
        self
    }

    #[doc="Modify the GPOLYHU register."]
    #[inline] pub fn with_gpolyhu<F: FnOnce(Gpolyhu) -> Gpolyhu>(&self, f: F) -> &Self {
        self.gpolyhu_reg().with(f);
        self
    }

    #[doc="Get the CTRL Register."]
    #[inline] pub fn ctrl_reg(&self) -> ::bobbin_mcu::register::Register<Ctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrl, 0x8)
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

    #[doc="Get the CTRLHU Register."]
    #[inline] pub fn ctrlhu_reg(&self) -> ::bobbin_mcu::register::Register<Ctrlhu> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Ctrlhu, 0xb)
    }

    #[doc="Get the *mut pointer for the CTRLHU register."]
    #[inline] pub fn ctrlhu_mut(&self) -> *mut Ctrlhu { 
        self.ctrlhu_reg().ptr()
    }

    #[doc="Get the *const pointer for the CTRLHU register."]
    #[inline] pub fn ctrlhu_ptr(&self) -> *const Ctrlhu { 
        self.ctrlhu_reg().ptr()
    }

    #[doc="Read the CTRLHU register."]
    #[inline] pub fn ctrlhu(&self) -> Ctrlhu { 
        self.ctrlhu_reg().read()
    }

    #[doc="Write the CTRLHU register."]
    #[inline] pub fn write_ctrlhu(&self, value: Ctrlhu) -> &Self { 
        self.ctrlhu_reg().write(value);
        self
    }

    #[doc="Set the CTRLHU register."]
    #[inline] pub fn set_ctrlhu<F: FnOnce(Ctrlhu) -> Ctrlhu>(&self, f: F) -> &Self {
        self.ctrlhu_reg().set(f);
        self
    }

    #[doc="Modify the CTRLHU register."]
    #[inline] pub fn with_ctrlhu<F: FnOnce(Ctrlhu) -> Ctrlhu>(&self, f: F) -> &Self {
        self.ctrlhu_reg().with(f);
        self
    }

}

#[doc="CRC Data register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Data(pub u32);
impl Data {
    #[doc="CRC Low Lower Byte"]
    #[inline] pub fn ll(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if LL != 0"]
    #[inline] pub fn test_ll(&self) -> bool {
        self.ll() != 0
    }

    #[doc="Sets the LL field."]
    #[inline] pub fn set_ll<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CRC Low Upper Byte"]
    #[inline] pub fn lu(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if LU != 0"]
    #[inline] pub fn test_lu(&self) -> bool {
        self.lu() != 0
    }

    #[doc="Sets the LU field."]
    #[inline] pub fn set_lu<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="CRC High Lower Byte"]
    #[inline] pub fn hl(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if HL != 0"]
    #[inline] pub fn test_hl(&self) -> bool {
        self.hl() != 0
    }

    #[doc="Sets the HL field."]
    #[inline] pub fn set_hl<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="CRC High Upper Byte"]
    #[inline] pub fn hu(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if HU != 0"]
    #[inline] pub fn test_hu(&self) -> bool {
        self.hu() != 0
    }

    #[doc="Sets the HU field."]
    #[inline] pub fn set_hu<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

}

impl From<u32> for Data {
    #[inline]
    fn from(other: u32) -> Self {
         Data(other)
    }
}

impl ::core::fmt::Display for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Data {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ll() != 0 { try!(write!(f, " ll=0x{:x}", self.ll()))}
        if self.lu() != 0 { try!(write!(f, " lu=0x{:x}", self.lu()))}
        if self.hl() != 0 { try!(write!(f, " hl=0x{:x}", self.hl()))}
        if self.hu() != 0 { try!(write!(f, " hu=0x{:x}", self.hu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_DATAL register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datal(pub u16);
impl Datal {
    #[doc="DATAL stores the lower 16 bits of the 16/32 bit CRC"]
    #[inline] pub fn datal(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DATAL != 0"]
    #[inline] pub fn test_datal(&self) -> bool {
        self.datal() != 0
    }

    #[doc="Sets the DATAL field."]
    #[inline] pub fn set_datal<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Datal {
    #[inline]
    fn from(other: u16) -> Self {
         Datal(other)
    }
}

impl ::core::fmt::Display for Datal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datal() != 0 { try!(write!(f, " datal=0x{:x}", self.datal()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_DATALL register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datall(pub u8);
impl Datall {
    #[doc="CRCLL stores the first 8 bits of the 32 bit DATA"]
    #[inline] pub fn datall(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATALL != 0"]
    #[inline] pub fn test_datall(&self) -> bool {
        self.datall() != 0
    }

    #[doc="Sets the DATALL field."]
    #[inline] pub fn set_datall<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Datall {
    #[inline]
    fn from(other: u8) -> Self {
         Datall(other)
    }
}

impl ::core::fmt::Display for Datall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datall {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datall() != 0 { try!(write!(f, " datall=0x{:x}", self.datall()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_DATALU register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datalu(pub u8);
impl Datalu {
    #[doc="DATALL stores the second 8 bits of the 32 bit CRC"]
    #[inline] pub fn datalu(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATALU != 0"]
    #[inline] pub fn test_datalu(&self) -> bool {
        self.datalu() != 0
    }

    #[doc="Sets the DATALU field."]
    #[inline] pub fn set_datalu<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Datalu {
    #[inline]
    fn from(other: u8) -> Self {
         Datalu(other)
    }
}

impl ::core::fmt::Display for Datalu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datalu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datalu() != 0 { try!(write!(f, " datalu=0x{:x}", self.datalu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_DATAH register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datah(pub u16);
impl Datah {
    #[doc="DATAH stores the high 16 bits of the 16/32 bit CRC"]
    #[inline] pub fn datah(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if DATAH != 0"]
    #[inline] pub fn test_datah(&self) -> bool {
        self.datah() != 0
    }

    #[doc="Sets the DATAH field."]
    #[inline] pub fn set_datah<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Datah {
    #[inline]
    fn from(other: u16) -> Self {
         Datah(other)
    }
}

impl ::core::fmt::Display for Datah {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datah {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datah() != 0 { try!(write!(f, " datah=0x{:x}", self.datah()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_DATAHL register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datahl(pub u8);
impl Datahl {
    #[doc="DATAHL stores the third 8 bits of the 32 bit CRC"]
    #[inline] pub fn datahl(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATAHL != 0"]
    #[inline] pub fn test_datahl(&self) -> bool {
        self.datahl() != 0
    }

    #[doc="Sets the DATAHL field."]
    #[inline] pub fn set_datahl<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Datahl {
    #[inline]
    fn from(other: u8) -> Self {
         Datahl(other)
    }
}

impl ::core::fmt::Display for Datahl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datahl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datahl() != 0 { try!(write!(f, " datahl=0x{:x}", self.datahl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_DATAHU register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Datahu(pub u8);
impl Datahu {
    #[doc="DATAHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline] pub fn datahu(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATAHU != 0"]
    #[inline] pub fn test_datahu(&self) -> bool {
        self.datahu() != 0
    }

    #[doc="Sets the DATAHU field."]
    #[inline] pub fn set_datahu<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Datahu {
    #[inline]
    fn from(other: u8) -> Self {
         Datahu(other)
    }
}

impl ::core::fmt::Display for Datahu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Datahu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.datahu() != 0 { try!(write!(f, " datahu=0x{:x}", self.datahu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC Polynomial register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpoly(pub u32);
impl Gpoly {
    #[doc="Low Polynominal Half-word"]
    #[inline] pub fn low(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if LOW != 0"]
    #[inline] pub fn test_low(&self) -> bool {
        self.low() != 0
    }

    #[doc="Sets the LOW field."]
    #[inline] pub fn set_low<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="High Polynominal Half-word"]
    #[inline] pub fn high(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if HIGH != 0"]
    #[inline] pub fn test_high(&self) -> bool {
        self.high() != 0
    }

    #[doc="Sets the HIGH field."]
    #[inline] pub fn set_high<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

}

impl From<u32> for Gpoly {
    #[inline]
    fn from(other: u32) -> Self {
         Gpoly(other)
    }
}

impl ::core::fmt::Display for Gpoly {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gpoly {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.low() != 0 { try!(write!(f, " low=0x{:x}", self.low()))}
        if self.high() != 0 { try!(write!(f, " high=0x{:x}", self.high()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_GPOLYL register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpolyl(pub u16);
impl Gpolyl {
    #[doc="POLYL stores the lower 16 bits of the 16/32 bit CRC polynomial value"]
    #[inline] pub fn gpolyl(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if GPOLYL != 0"]
    #[inline] pub fn test_gpolyl(&self) -> bool {
        self.gpolyl() != 0
    }

    #[doc="Sets the GPOLYL field."]
    #[inline] pub fn set_gpolyl<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Gpolyl {
    #[inline]
    fn from(other: u16) -> Self {
         Gpolyl(other)
    }
}

impl ::core::fmt::Display for Gpolyl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gpolyl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpolyl() != 0 { try!(write!(f, " gpolyl=0x{:x}", self.gpolyl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_GPOLYLL register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpolyll(pub u8);
impl Gpolyll {
    #[doc="POLYLL stores the first 8 bits of the 32 bit CRC"]
    #[inline] pub fn gpolyll(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if GPOLYLL != 0"]
    #[inline] pub fn test_gpolyll(&self) -> bool {
        self.gpolyll() != 0
    }

    #[doc="Sets the GPOLYLL field."]
    #[inline] pub fn set_gpolyll<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Gpolyll {
    #[inline]
    fn from(other: u8) -> Self {
         Gpolyll(other)
    }
}

impl ::core::fmt::Display for Gpolyll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gpolyll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpolyll() != 0 { try!(write!(f, " gpolyll=0x{:x}", self.gpolyll()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_GPOLYLU register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpolylu(pub u8);
impl Gpolylu {
    #[doc="POLYLL stores the second 8 bits of the 32 bit CRC"]
    #[inline] pub fn gpolylu(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if GPOLYLU != 0"]
    #[inline] pub fn test_gpolylu(&self) -> bool {
        self.gpolylu() != 0
    }

    #[doc="Sets the GPOLYLU field."]
    #[inline] pub fn set_gpolylu<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Gpolylu {
    #[inline]
    fn from(other: u8) -> Self {
         Gpolylu(other)
    }
}

impl ::core::fmt::Display for Gpolylu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gpolylu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpolylu() != 0 { try!(write!(f, " gpolylu=0x{:x}", self.gpolylu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_GPOLYH register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpolyh(pub u16);
impl Gpolyh {
    #[doc="POLYH stores the high 16 bits of the 16/32 bit CRC polynomial value"]
    #[inline] pub fn gpolyh(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if GPOLYH != 0"]
    #[inline] pub fn test_gpolyh(&self) -> bool {
        self.gpolyh() != 0
    }

    #[doc="Sets the GPOLYH field."]
    #[inline] pub fn set_gpolyh<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u16 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u16> for Gpolyh {
    #[inline]
    fn from(other: u16) -> Self {
         Gpolyh(other)
    }
}

impl ::core::fmt::Display for Gpolyh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gpolyh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpolyh() != 0 { try!(write!(f, " gpolyh=0x{:x}", self.gpolyh()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_GPOLYHL register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpolyhl(pub u8);
impl Gpolyhl {
    #[doc="POLYHL stores the third 8 bits of the 32 bit CRC"]
    #[inline] pub fn gpolyhl(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if GPOLYHL != 0"]
    #[inline] pub fn test_gpolyhl(&self) -> bool {
        self.gpolyhl() != 0
    }

    #[doc="Sets the GPOLYHL field."]
    #[inline] pub fn set_gpolyhl<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Gpolyhl {
    #[inline]
    fn from(other: u8) -> Self {
         Gpolyhl(other)
    }
}

impl ::core::fmt::Display for Gpolyhl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gpolyhl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpolyhl() != 0 { try!(write!(f, " gpolyhl=0x{:x}", self.gpolyhl()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_GPOLYHU register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Gpolyhu(pub u8);
impl Gpolyhu {
    #[doc="POLYHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline] pub fn gpolyhu(&self) -> ::bobbin_bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if GPOLYHU != 0"]
    #[inline] pub fn test_gpolyhu(&self) -> bool {
        self.gpolyhu() != 0
    }

    #[doc="Sets the GPOLYHU field."]
    #[inline] pub fn set_gpolyhu<V: Into<::bobbin_bits::U8>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U8 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u8> for Gpolyhu {
    #[inline]
    fn from(other: u8) -> Self {
         Gpolyhu(other)
    }
}

impl ::core::fmt::Display for Gpolyhu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Gpolyhu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.gpolyhu() != 0 { try!(write!(f, " gpolyhu=0x{:x}", self.gpolyhu()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC Control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc="Width of CRC protocol."]
    #[inline] pub fn tcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if TCRC != 0"]
    #[inline] pub fn test_tcrc(&self) -> bool {
        self.tcrc() != 0
    }

    #[doc="Sets the TCRC field."]
    #[inline] pub fn set_tcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Write CRC Data Register As Seed"]
    #[inline] pub fn was(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if WAS != 0"]
    #[inline] pub fn test_was(&self) -> bool {
        self.was() != 0
    }

    #[doc="Sets the WAS field."]
    #[inline] pub fn set_was<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Complement Read Of CRC Data Register"]
    #[inline] pub fn fxor(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if FXOR != 0"]
    #[inline] pub fn test_fxor(&self) -> bool {
        self.fxor() != 0
    }

    #[doc="Sets the FXOR field."]
    #[inline] pub fn set_fxor<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Type Of Transpose For Read"]
    #[inline] pub fn totr(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x3) as u8) } // [29:28]
    }

    #[doc="Returns true if TOTR != 0"]
    #[inline] pub fn test_totr(&self) -> bool {
        self.totr() != 0
    }

    #[doc="Sets the TOTR field."]
    #[inline] pub fn set_totr<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Type Of Transpose For Writes"]
    #[inline] pub fn tot(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x3) as u8) } // [31:30]
    }

    #[doc="Returns true if TOT != 0"]
    #[inline] pub fn test_tot(&self) -> bool {
        self.tot() != 0
    }

    #[doc="Sets the TOT field."]
    #[inline] pub fn set_tot<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Ctrl {
    #[inline]
    fn from(other: u32) -> Self {
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
        if self.tcrc() != 0 { try!(write!(f, " tcrc"))}
        if self.was() != 0 { try!(write!(f, " was"))}
        if self.fxor() != 0 { try!(write!(f, " fxor"))}
        if self.totr() != 0 { try!(write!(f, " totr=0x{:x}", self.totr()))}
        if self.tot() != 0 { try!(write!(f, " tot=0x{:x}", self.tot()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="CRC_CTRLHU register."]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ctrlhu(pub u8);
impl Ctrlhu {
    #[doc="no description available"]
    #[inline] pub fn tcrc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TCRC != 0"]
    #[inline] pub fn test_tcrc(&self) -> bool {
        self.tcrc() != 0
    }

    #[doc="Sets the TCRC field."]
    #[inline] pub fn set_tcrc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn was(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if WAS != 0"]
    #[inline] pub fn test_was(&self) -> bool {
        self.was() != 0
    }

    #[doc="Sets the WAS field."]
    #[inline] pub fn set_was<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn fxor(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FXOR != 0"]
    #[inline] pub fn test_fxor(&self) -> bool {
        self.fxor() != 0
    }

    #[doc="Sets the FXOR field."]
    #[inline] pub fn set_fxor<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn totr(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x3) as u8) } // [5:4]
    }

    #[doc="Returns true if TOTR != 0"]
    #[inline] pub fn test_totr(&self) -> bool {
        self.totr() != 0
    }

    #[doc="Sets the TOTR field."]
    #[inline] pub fn set_totr<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="no description available"]
    #[inline] pub fn tot(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if TOT != 0"]
    #[inline] pub fn test_tot(&self) -> bool {
        self.tot() != 0
    }

    #[doc="Sets the TOT field."]
    #[inline] pub fn set_tot<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u8 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u8> for Ctrlhu {
    #[inline]
    fn from(other: u8) -> Self {
         Ctrlhu(other)
    }
}

impl ::core::fmt::Display for Ctrlhu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ctrlhu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.tcrc() != 0 { try!(write!(f, " tcrc"))}
        if self.was() != 0 { try!(write!(f, " was"))}
        if self.fxor() != 0 { try!(write!(f, " fxor"))}
        if self.totr() != 0 { try!(write!(f, " totr=0x{:x}", self.totr()))}
        if self.tot() != 0 { try!(write!(f, " tot=0x{:x}", self.tot()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

