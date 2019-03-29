::bobbin_mcu::periph!( FTM0, Ftm0, FTM0_PERIPH, FtmPeriph, FTM0_OWNED, FTM0_REF_COUNT, 0x40038000, 0x00, 0x04);
::bobbin_mcu::periph!( FTM1, Ftm1, FTM1_PERIPH, FtmPeriph, FTM1_OWNED, FTM1_REF_COUNT, 0x40039000, 0x01, 0x05);
::bobbin_mcu::periph!( FTM2, Ftm2, FTM2_PERIPH, FtmPeriph, FTM2_OWNED, FTM2_REF_COUNT, 0x4003a000, 0x02, 0x06);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="FTM Peripheral"]
pub struct FtmPeriph(pub usize); 

impl FtmPeriph {
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
    #[inline] pub fn csc_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Csc, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Csc, 0xc, 0x8)
    }

    #[doc="Get the *mut pointer for the CSC register."]
    #[inline] pub fn csc_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Csc { 
        self.csc_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CSC register."]
    #[inline] pub fn csc_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Csc { 
        self.csc_reg().ptr(index.into())
    }

    #[doc="Read the CSC register."]
    #[inline] pub fn csc<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Csc { 
        self.csc_reg().read(index.into())
    }

    #[doc="Write the CSC register."]
    #[inline] pub fn write_csc<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Csc) -> &Self {
        self.csc_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CSC register."]
    #[inline] pub fn set_csc<I: Into<::bobbin_bits::R8>, F: FnOnce(Csc) -> Csc>(&self, index: I, f: F) -> &Self {
        self.csc_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CSC register."]
    #[inline] pub fn with_csc<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Csc) -> Csc>(&self, index: I, f: F) -> &Self {
        self.csc_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CV Register."]
    #[inline] pub fn cv_reg(&self) -> ::bobbin_mcu::register::RegisterArray<Cv, ::bobbin_bits::R8> { 
        ::bobbin_mcu::register::RegisterArray::new(self.0 as *mut Cv, 0x10, 0x8)
    }

    #[doc="Get the *mut pointer for the CV register."]
    #[inline] pub fn cv_mut<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *mut Cv { 
        self.cv_reg().ptr(index.into())
    }

    #[doc="Get the *const pointer for the CV register."]
    #[inline] pub fn cv_ptr<I: Into<::bobbin_bits::R8>>(&self, index: I) -> *const Cv { 
        self.cv_reg().ptr(index.into())
    }

    #[doc="Read the CV register."]
    #[inline] pub fn cv<I: Into<::bobbin_bits::R8>>(&self, index: I) -> Cv { 
        self.cv_reg().read(index.into())
    }

    #[doc="Write the CV register."]
    #[inline] pub fn write_cv<I: Into<::bobbin_bits::R8>>(&self, index: I, value: Cv) -> &Self {
        self.cv_reg().write(index.into(), value);
        self
    }

    #[doc="Set the CV register."]
    #[inline] pub fn set_cv<I: Into<::bobbin_bits::R8>, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
        self.cv_reg().set(index.into(), f);
        self
    }

    #[doc="Modify the CV register."]
    #[inline] pub fn with_cv<I: Into<::bobbin_bits::R8> + Copy, F: FnOnce(Cv) -> Cv>(&self, index: I, f: F) -> &Self {
        self.cv_reg().with(index.into(), f);
        self
    }

    #[doc="Get the CNTIN Register."]
    #[inline] pub fn cntin_reg(&self) -> ::bobbin_mcu::register::Register<Cntin> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Cntin, 0x4c)
    }

    #[doc="Get the *mut pointer for the CNTIN register."]
    #[inline] pub fn cntin_mut(&self) -> *mut Cntin { 
        self.cntin_reg().ptr()
    }

    #[doc="Get the *const pointer for the CNTIN register."]
    #[inline] pub fn cntin_ptr(&self) -> *const Cntin { 
        self.cntin_reg().ptr()
    }

    #[doc="Read the CNTIN register."]
    #[inline] pub fn cntin(&self) -> Cntin { 
        self.cntin_reg().read()
    }

    #[doc="Write the CNTIN register."]
    #[inline] pub fn write_cntin(&self, value: Cntin) -> &Self { 
        self.cntin_reg().write(value);
        self
    }

    #[doc="Set the CNTIN register."]
    #[inline] pub fn set_cntin<F: FnOnce(Cntin) -> Cntin>(&self, f: F) -> &Self {
        self.cntin_reg().set(f);
        self
    }

    #[doc="Modify the CNTIN register."]
    #[inline] pub fn with_cntin<F: FnOnce(Cntin) -> Cntin>(&self, f: F) -> &Self {
        self.cntin_reg().with(f);
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

    #[doc="Get the MODE Register."]
    #[inline] pub fn mode_reg(&self) -> ::bobbin_mcu::register::Register<Mode> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Mode, 0x54)
    }

    #[doc="Get the *mut pointer for the MODE register."]
    #[inline] pub fn mode_mut(&self) -> *mut Mode { 
        self.mode_reg().ptr()
    }

    #[doc="Get the *const pointer for the MODE register."]
    #[inline] pub fn mode_ptr(&self) -> *const Mode { 
        self.mode_reg().ptr()
    }

    #[doc="Read the MODE register."]
    #[inline] pub fn mode(&self) -> Mode { 
        self.mode_reg().read()
    }

    #[doc="Write the MODE register."]
    #[inline] pub fn write_mode(&self, value: Mode) -> &Self { 
        self.mode_reg().write(value);
        self
    }

    #[doc="Set the MODE register."]
    #[inline] pub fn set_mode<F: FnOnce(Mode) -> Mode>(&self, f: F) -> &Self {
        self.mode_reg().set(f);
        self
    }

    #[doc="Modify the MODE register."]
    #[inline] pub fn with_mode<F: FnOnce(Mode) -> Mode>(&self, f: F) -> &Self {
        self.mode_reg().with(f);
        self
    }

    #[doc="Get the SYNC Register."]
    #[inline] pub fn sync_reg(&self) -> ::bobbin_mcu::register::Register<Sync> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Sync, 0x58)
    }

    #[doc="Get the *mut pointer for the SYNC register."]
    #[inline] pub fn sync_mut(&self) -> *mut Sync { 
        self.sync_reg().ptr()
    }

    #[doc="Get the *const pointer for the SYNC register."]
    #[inline] pub fn sync_ptr(&self) -> *const Sync { 
        self.sync_reg().ptr()
    }

    #[doc="Read the SYNC register."]
    #[inline] pub fn sync(&self) -> Sync { 
        self.sync_reg().read()
    }

    #[doc="Write the SYNC register."]
    #[inline] pub fn write_sync(&self, value: Sync) -> &Self { 
        self.sync_reg().write(value);
        self
    }

    #[doc="Set the SYNC register."]
    #[inline] pub fn set_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
        self.sync_reg().set(f);
        self
    }

    #[doc="Modify the SYNC register."]
    #[inline] pub fn with_sync<F: FnOnce(Sync) -> Sync>(&self, f: F) -> &Self {
        self.sync_reg().with(f);
        self
    }

    #[doc="Get the OUTINIT Register."]
    #[inline] pub fn outinit_reg(&self) -> ::bobbin_mcu::register::Register<Outinit> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Outinit, 0x5c)
    }

    #[doc="Get the *mut pointer for the OUTINIT register."]
    #[inline] pub fn outinit_mut(&self) -> *mut Outinit { 
        self.outinit_reg().ptr()
    }

    #[doc="Get the *const pointer for the OUTINIT register."]
    #[inline] pub fn outinit_ptr(&self) -> *const Outinit { 
        self.outinit_reg().ptr()
    }

    #[doc="Read the OUTINIT register."]
    #[inline] pub fn outinit(&self) -> Outinit { 
        self.outinit_reg().read()
    }

    #[doc="Write the OUTINIT register."]
    #[inline] pub fn write_outinit(&self, value: Outinit) -> &Self { 
        self.outinit_reg().write(value);
        self
    }

    #[doc="Set the OUTINIT register."]
    #[inline] pub fn set_outinit<F: FnOnce(Outinit) -> Outinit>(&self, f: F) -> &Self {
        self.outinit_reg().set(f);
        self
    }

    #[doc="Modify the OUTINIT register."]
    #[inline] pub fn with_outinit<F: FnOnce(Outinit) -> Outinit>(&self, f: F) -> &Self {
        self.outinit_reg().with(f);
        self
    }

    #[doc="Get the OUTMASK Register."]
    #[inline] pub fn outmask_reg(&self) -> ::bobbin_mcu::register::Register<Outmask> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Outmask, 0x60)
    }

    #[doc="Get the *mut pointer for the OUTMASK register."]
    #[inline] pub fn outmask_mut(&self) -> *mut Outmask { 
        self.outmask_reg().ptr()
    }

    #[doc="Get the *const pointer for the OUTMASK register."]
    #[inline] pub fn outmask_ptr(&self) -> *const Outmask { 
        self.outmask_reg().ptr()
    }

    #[doc="Read the OUTMASK register."]
    #[inline] pub fn outmask(&self) -> Outmask { 
        self.outmask_reg().read()
    }

    #[doc="Write the OUTMASK register."]
    #[inline] pub fn write_outmask(&self, value: Outmask) -> &Self { 
        self.outmask_reg().write(value);
        self
    }

    #[doc="Set the OUTMASK register."]
    #[inline] pub fn set_outmask<F: FnOnce(Outmask) -> Outmask>(&self, f: F) -> &Self {
        self.outmask_reg().set(f);
        self
    }

    #[doc="Modify the OUTMASK register."]
    #[inline] pub fn with_outmask<F: FnOnce(Outmask) -> Outmask>(&self, f: F) -> &Self {
        self.outmask_reg().with(f);
        self
    }

    #[doc="Get the COMBINE Register."]
    #[inline] pub fn combine_reg(&self) -> ::bobbin_mcu::register::Register<Combine> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Combine, 0x64)
    }

    #[doc="Get the *mut pointer for the COMBINE register."]
    #[inline] pub fn combine_mut(&self) -> *mut Combine { 
        self.combine_reg().ptr()
    }

    #[doc="Get the *const pointer for the COMBINE register."]
    #[inline] pub fn combine_ptr(&self) -> *const Combine { 
        self.combine_reg().ptr()
    }

    #[doc="Read the COMBINE register."]
    #[inline] pub fn combine(&self) -> Combine { 
        self.combine_reg().read()
    }

    #[doc="Write the COMBINE register."]
    #[inline] pub fn write_combine(&self, value: Combine) -> &Self { 
        self.combine_reg().write(value);
        self
    }

    #[doc="Set the COMBINE register."]
    #[inline] pub fn set_combine<F: FnOnce(Combine) -> Combine>(&self, f: F) -> &Self {
        self.combine_reg().set(f);
        self
    }

    #[doc="Modify the COMBINE register."]
    #[inline] pub fn with_combine<F: FnOnce(Combine) -> Combine>(&self, f: F) -> &Self {
        self.combine_reg().with(f);
        self
    }

    #[doc="Get the DEADTIME Register."]
    #[inline] pub fn deadtime_reg(&self) -> ::bobbin_mcu::register::Register<Deadtime> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Deadtime, 0x68)
    }

    #[doc="Get the *mut pointer for the DEADTIME register."]
    #[inline] pub fn deadtime_mut(&self) -> *mut Deadtime { 
        self.deadtime_reg().ptr()
    }

    #[doc="Get the *const pointer for the DEADTIME register."]
    #[inline] pub fn deadtime_ptr(&self) -> *const Deadtime { 
        self.deadtime_reg().ptr()
    }

    #[doc="Read the DEADTIME register."]
    #[inline] pub fn deadtime(&self) -> Deadtime { 
        self.deadtime_reg().read()
    }

    #[doc="Write the DEADTIME register."]
    #[inline] pub fn write_deadtime(&self, value: Deadtime) -> &Self { 
        self.deadtime_reg().write(value);
        self
    }

    #[doc="Set the DEADTIME register."]
    #[inline] pub fn set_deadtime<F: FnOnce(Deadtime) -> Deadtime>(&self, f: F) -> &Self {
        self.deadtime_reg().set(f);
        self
    }

    #[doc="Modify the DEADTIME register."]
    #[inline] pub fn with_deadtime<F: FnOnce(Deadtime) -> Deadtime>(&self, f: F) -> &Self {
        self.deadtime_reg().with(f);
        self
    }

    #[doc="Get the EXTTRIG Register."]
    #[inline] pub fn exttrig_reg(&self) -> ::bobbin_mcu::register::Register<Exttrig> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Exttrig, 0x6c)
    }

    #[doc="Get the *mut pointer for the EXTTRIG register."]
    #[inline] pub fn exttrig_mut(&self) -> *mut Exttrig { 
        self.exttrig_reg().ptr()
    }

    #[doc="Get the *const pointer for the EXTTRIG register."]
    #[inline] pub fn exttrig_ptr(&self) -> *const Exttrig { 
        self.exttrig_reg().ptr()
    }

    #[doc="Read the EXTTRIG register."]
    #[inline] pub fn exttrig(&self) -> Exttrig { 
        self.exttrig_reg().read()
    }

    #[doc="Write the EXTTRIG register."]
    #[inline] pub fn write_exttrig(&self, value: Exttrig) -> &Self { 
        self.exttrig_reg().write(value);
        self
    }

    #[doc="Set the EXTTRIG register."]
    #[inline] pub fn set_exttrig<F: FnOnce(Exttrig) -> Exttrig>(&self, f: F) -> &Self {
        self.exttrig_reg().set(f);
        self
    }

    #[doc="Modify the EXTTRIG register."]
    #[inline] pub fn with_exttrig<F: FnOnce(Exttrig) -> Exttrig>(&self, f: F) -> &Self {
        self.exttrig_reg().with(f);
        self
    }

    #[doc="Get the POL Register."]
    #[inline] pub fn pol_reg(&self) -> ::bobbin_mcu::register::Register<Pol> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pol, 0x70)
    }

    #[doc="Get the *mut pointer for the POL register."]
    #[inline] pub fn pol_mut(&self) -> *mut Pol { 
        self.pol_reg().ptr()
    }

    #[doc="Get the *const pointer for the POL register."]
    #[inline] pub fn pol_ptr(&self) -> *const Pol { 
        self.pol_reg().ptr()
    }

    #[doc="Read the POL register."]
    #[inline] pub fn pol(&self) -> Pol { 
        self.pol_reg().read()
    }

    #[doc="Write the POL register."]
    #[inline] pub fn write_pol(&self, value: Pol) -> &Self { 
        self.pol_reg().write(value);
        self
    }

    #[doc="Set the POL register."]
    #[inline] pub fn set_pol<F: FnOnce(Pol) -> Pol>(&self, f: F) -> &Self {
        self.pol_reg().set(f);
        self
    }

    #[doc="Modify the POL register."]
    #[inline] pub fn with_pol<F: FnOnce(Pol) -> Pol>(&self, f: F) -> &Self {
        self.pol_reg().with(f);
        self
    }

    #[doc="Get the FMS Register."]
    #[inline] pub fn fms_reg(&self) -> ::bobbin_mcu::register::Register<Fms> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fms, 0x74)
    }

    #[doc="Get the *mut pointer for the FMS register."]
    #[inline] pub fn fms_mut(&self) -> *mut Fms { 
        self.fms_reg().ptr()
    }

    #[doc="Get the *const pointer for the FMS register."]
    #[inline] pub fn fms_ptr(&self) -> *const Fms { 
        self.fms_reg().ptr()
    }

    #[doc="Read the FMS register."]
    #[inline] pub fn fms(&self) -> Fms { 
        self.fms_reg().read()
    }

    #[doc="Write the FMS register."]
    #[inline] pub fn write_fms(&self, value: Fms) -> &Self { 
        self.fms_reg().write(value);
        self
    }

    #[doc="Set the FMS register."]
    #[inline] pub fn set_fms<F: FnOnce(Fms) -> Fms>(&self, f: F) -> &Self {
        self.fms_reg().set(f);
        self
    }

    #[doc="Modify the FMS register."]
    #[inline] pub fn with_fms<F: FnOnce(Fms) -> Fms>(&self, f: F) -> &Self {
        self.fms_reg().with(f);
        self
    }

    #[doc="Get the FILTER Register."]
    #[inline] pub fn filter_reg(&self) -> ::bobbin_mcu::register::Register<Filter> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Filter, 0x78)
    }

    #[doc="Get the *mut pointer for the FILTER register."]
    #[inline] pub fn filter_mut(&self) -> *mut Filter { 
        self.filter_reg().ptr()
    }

    #[doc="Get the *const pointer for the FILTER register."]
    #[inline] pub fn filter_ptr(&self) -> *const Filter { 
        self.filter_reg().ptr()
    }

    #[doc="Read the FILTER register."]
    #[inline] pub fn filter(&self) -> Filter { 
        self.filter_reg().read()
    }

    #[doc="Write the FILTER register."]
    #[inline] pub fn write_filter(&self, value: Filter) -> &Self { 
        self.filter_reg().write(value);
        self
    }

    #[doc="Set the FILTER register."]
    #[inline] pub fn set_filter<F: FnOnce(Filter) -> Filter>(&self, f: F) -> &Self {
        self.filter_reg().set(f);
        self
    }

    #[doc="Modify the FILTER register."]
    #[inline] pub fn with_filter<F: FnOnce(Filter) -> Filter>(&self, f: F) -> &Self {
        self.filter_reg().with(f);
        self
    }

    #[doc="Get the FLTCTRL Register."]
    #[inline] pub fn fltctrl_reg(&self) -> ::bobbin_mcu::register::Register<Fltctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fltctrl, 0x7c)
    }

    #[doc="Get the *mut pointer for the FLTCTRL register."]
    #[inline] pub fn fltctrl_mut(&self) -> *mut Fltctrl { 
        self.fltctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the FLTCTRL register."]
    #[inline] pub fn fltctrl_ptr(&self) -> *const Fltctrl { 
        self.fltctrl_reg().ptr()
    }

    #[doc="Read the FLTCTRL register."]
    #[inline] pub fn fltctrl(&self) -> Fltctrl { 
        self.fltctrl_reg().read()
    }

    #[doc="Write the FLTCTRL register."]
    #[inline] pub fn write_fltctrl(&self, value: Fltctrl) -> &Self { 
        self.fltctrl_reg().write(value);
        self
    }

    #[doc="Set the FLTCTRL register."]
    #[inline] pub fn set_fltctrl<F: FnOnce(Fltctrl) -> Fltctrl>(&self, f: F) -> &Self {
        self.fltctrl_reg().set(f);
        self
    }

    #[doc="Modify the FLTCTRL register."]
    #[inline] pub fn with_fltctrl<F: FnOnce(Fltctrl) -> Fltctrl>(&self, f: F) -> &Self {
        self.fltctrl_reg().with(f);
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

    #[doc="Get the FLTPOL Register."]
    #[inline] pub fn fltpol_reg(&self) -> ::bobbin_mcu::register::Register<Fltpol> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Fltpol, 0x88)
    }

    #[doc="Get the *mut pointer for the FLTPOL register."]
    #[inline] pub fn fltpol_mut(&self) -> *mut Fltpol { 
        self.fltpol_reg().ptr()
    }

    #[doc="Get the *const pointer for the FLTPOL register."]
    #[inline] pub fn fltpol_ptr(&self) -> *const Fltpol { 
        self.fltpol_reg().ptr()
    }

    #[doc="Read the FLTPOL register."]
    #[inline] pub fn fltpol(&self) -> Fltpol { 
        self.fltpol_reg().read()
    }

    #[doc="Write the FLTPOL register."]
    #[inline] pub fn write_fltpol(&self, value: Fltpol) -> &Self { 
        self.fltpol_reg().write(value);
        self
    }

    #[doc="Set the FLTPOL register."]
    #[inline] pub fn set_fltpol<F: FnOnce(Fltpol) -> Fltpol>(&self, f: F) -> &Self {
        self.fltpol_reg().set(f);
        self
    }

    #[doc="Modify the FLTPOL register."]
    #[inline] pub fn with_fltpol<F: FnOnce(Fltpol) -> Fltpol>(&self, f: F) -> &Self {
        self.fltpol_reg().with(f);
        self
    }

    #[doc="Get the SYNCONF Register."]
    #[inline] pub fn synconf_reg(&self) -> ::bobbin_mcu::register::Register<Synconf> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Synconf, 0x8c)
    }

    #[doc="Get the *mut pointer for the SYNCONF register."]
    #[inline] pub fn synconf_mut(&self) -> *mut Synconf { 
        self.synconf_reg().ptr()
    }

    #[doc="Get the *const pointer for the SYNCONF register."]
    #[inline] pub fn synconf_ptr(&self) -> *const Synconf { 
        self.synconf_reg().ptr()
    }

    #[doc="Read the SYNCONF register."]
    #[inline] pub fn synconf(&self) -> Synconf { 
        self.synconf_reg().read()
    }

    #[doc="Write the SYNCONF register."]
    #[inline] pub fn write_synconf(&self, value: Synconf) -> &Self { 
        self.synconf_reg().write(value);
        self
    }

    #[doc="Set the SYNCONF register."]
    #[inline] pub fn set_synconf<F: FnOnce(Synconf) -> Synconf>(&self, f: F) -> &Self {
        self.synconf_reg().set(f);
        self
    }

    #[doc="Modify the SYNCONF register."]
    #[inline] pub fn with_synconf<F: FnOnce(Synconf) -> Synconf>(&self, f: F) -> &Self {
        self.synconf_reg().with(f);
        self
    }

    #[doc="Get the INVCTRL Register."]
    #[inline] pub fn invctrl_reg(&self) -> ::bobbin_mcu::register::Register<Invctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Invctrl, 0x90)
    }

    #[doc="Get the *mut pointer for the INVCTRL register."]
    #[inline] pub fn invctrl_mut(&self) -> *mut Invctrl { 
        self.invctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the INVCTRL register."]
    #[inline] pub fn invctrl_ptr(&self) -> *const Invctrl { 
        self.invctrl_reg().ptr()
    }

    #[doc="Read the INVCTRL register."]
    #[inline] pub fn invctrl(&self) -> Invctrl { 
        self.invctrl_reg().read()
    }

    #[doc="Write the INVCTRL register."]
    #[inline] pub fn write_invctrl(&self, value: Invctrl) -> &Self { 
        self.invctrl_reg().write(value);
        self
    }

    #[doc="Set the INVCTRL register."]
    #[inline] pub fn set_invctrl<F: FnOnce(Invctrl) -> Invctrl>(&self, f: F) -> &Self {
        self.invctrl_reg().set(f);
        self
    }

    #[doc="Modify the INVCTRL register."]
    #[inline] pub fn with_invctrl<F: FnOnce(Invctrl) -> Invctrl>(&self, f: F) -> &Self {
        self.invctrl_reg().with(f);
        self
    }

    #[doc="Get the SWOCTRL Register."]
    #[inline] pub fn swoctrl_reg(&self) -> ::bobbin_mcu::register::Register<Swoctrl> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Swoctrl, 0x94)
    }

    #[doc="Get the *mut pointer for the SWOCTRL register."]
    #[inline] pub fn swoctrl_mut(&self) -> *mut Swoctrl { 
        self.swoctrl_reg().ptr()
    }

    #[doc="Get the *const pointer for the SWOCTRL register."]
    #[inline] pub fn swoctrl_ptr(&self) -> *const Swoctrl { 
        self.swoctrl_reg().ptr()
    }

    #[doc="Read the SWOCTRL register."]
    #[inline] pub fn swoctrl(&self) -> Swoctrl { 
        self.swoctrl_reg().read()
    }

    #[doc="Write the SWOCTRL register."]
    #[inline] pub fn write_swoctrl(&self, value: Swoctrl) -> &Self { 
        self.swoctrl_reg().write(value);
        self
    }

    #[doc="Set the SWOCTRL register."]
    #[inline] pub fn set_swoctrl<F: FnOnce(Swoctrl) -> Swoctrl>(&self, f: F) -> &Self {
        self.swoctrl_reg().set(f);
        self
    }

    #[doc="Modify the SWOCTRL register."]
    #[inline] pub fn with_swoctrl<F: FnOnce(Swoctrl) -> Swoctrl>(&self, f: F) -> &Self {
        self.swoctrl_reg().with(f);
        self
    }

    #[doc="Get the PWMLOAD Register."]
    #[inline] pub fn pwmload_reg(&self) -> ::bobbin_mcu::register::Register<Pwmload> { 
        ::bobbin_mcu::register::Register::new(self.0 as *mut Pwmload, 0x98)
    }

    #[doc="Get the *mut pointer for the PWMLOAD register."]
    #[inline] pub fn pwmload_mut(&self) -> *mut Pwmload { 
        self.pwmload_reg().ptr()
    }

    #[doc="Get the *const pointer for the PWMLOAD register."]
    #[inline] pub fn pwmload_ptr(&self) -> *const Pwmload { 
        self.pwmload_reg().ptr()
    }

    #[doc="Read the PWMLOAD register."]
    #[inline] pub fn pwmload(&self) -> Pwmload { 
        self.pwmload_reg().read()
    }

    #[doc="Write the PWMLOAD register."]
    #[inline] pub fn write_pwmload(&self, value: Pwmload) -> &Self { 
        self.pwmload_reg().write(value);
        self
    }

    #[doc="Set the PWMLOAD register."]
    #[inline] pub fn set_pwmload<F: FnOnce(Pwmload) -> Pwmload>(&self, f: F) -> &Self {
        self.pwmload_reg().set(f);
        self
    }

    #[doc="Modify the PWMLOAD register."]
    #[inline] pub fn with_pwmload<F: FnOnce(Pwmload) -> Pwmload>(&self, f: F) -> &Self {
        self.pwmload_reg().with(f);
        self
    }

}

#[doc="Status And Control"]
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

    #[doc="Clock Source Selection"]
    #[inline] pub fn clks(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3) as u8) } // [4:3]
    }

    #[doc="Returns true if CLKS != 0"]
    #[inline] pub fn test_clks(&self) -> bool {
        self.clks() != 0
    }

    #[doc="Sets the CLKS field."]
    #[inline] pub fn set_clks<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
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
        if self.clks() != 0 { try!(write!(f, " clks=0x{:x}", self.clks()))}
        if self.cpwms() != 0 { try!(write!(f, " cpwms"))}
        if self.toie() != 0 { try!(write!(f, " toie"))}
        if self.tof() != 0 { try!(write!(f, " tof"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Counter"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc="Counter Value"]
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
    #[doc="Modulo Value"]
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

#[doc="Channel (n) Status And Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Csc(pub u32);
impl Csc {
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

#[doc="Counter Initial Value"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Cntin(pub u32);
impl Cntin {
    #[doc="Initial Value Of The FTM Counter"]
    #[inline] pub fn init(&self) -> ::bobbin_bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffff) as u16) } // [15:0]
    }

    #[doc="Returns true if INIT != 0"]
    #[inline] pub fn test_init(&self) -> bool {
        self.init() != 0
    }

    #[doc="Sets the INIT field."]
    #[inline] pub fn set_init<V: Into<::bobbin_bits::U16>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Cntin {
    #[inline]
    fn from(other: u32) -> Self {
         Cntin(other)
    }
}

impl ::core::fmt::Display for Cntin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Cntin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.init() != 0 { try!(write!(f, " init=0x{:x}", self.init()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Capture And Compare Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Status(pub u32);
impl Status {
    #[doc="Channel 0 Flag"]
    #[inline] pub fn ch0f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH0F != 0"]
    #[inline] pub fn test_ch0f(&self) -> bool {
        self.ch0f() != 0
    }

    #[doc="Sets the CH0F field."]
    #[inline] pub fn set_ch0f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 1 Flag"]
    #[inline] pub fn ch1f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH1F != 0"]
    #[inline] pub fn test_ch1f(&self) -> bool {
        self.ch1f() != 0
    }

    #[doc="Sets the CH1F field."]
    #[inline] pub fn set_ch1f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel 2 Flag"]
    #[inline] pub fn ch2f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH2F != 0"]
    #[inline] pub fn test_ch2f(&self) -> bool {
        self.ch2f() != 0
    }

    #[doc="Sets the CH2F field."]
    #[inline] pub fn set_ch2f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Channel 3 Flag"]
    #[inline] pub fn ch3f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH3F != 0"]
    #[inline] pub fn test_ch3f(&self) -> bool {
        self.ch3f() != 0
    }

    #[doc="Sets the CH3F field."]
    #[inline] pub fn set_ch3f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel 4 Flag"]
    #[inline] pub fn ch4f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH4F != 0"]
    #[inline] pub fn test_ch4f(&self) -> bool {
        self.ch4f() != 0
    }

    #[doc="Sets the CH4F field."]
    #[inline] pub fn set_ch4f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 5 Flag"]
    #[inline] pub fn ch5f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH5F != 0"]
    #[inline] pub fn test_ch5f(&self) -> bool {
        self.ch5f() != 0
    }

    #[doc="Sets the CH5F field."]
    #[inline] pub fn set_ch5f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel 6 Flag"]
    #[inline] pub fn ch6f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CH6F != 0"]
    #[inline] pub fn test_ch6f(&self) -> bool {
        self.ch6f() != 0
    }

    #[doc="Sets the CH6F field."]
    #[inline] pub fn set_ch6f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel 7 Flag"]
    #[inline] pub fn ch7f(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CH7F != 0"]
    #[inline] pub fn test_ch7f(&self) -> bool {
        self.ch7f() != 0
    }

    #[doc="Sets the CH7F field."]
    #[inline] pub fn set_ch7f<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
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
        if self.ch0f() != 0 { try!(write!(f, " ch0f"))}
        if self.ch1f() != 0 { try!(write!(f, " ch1f"))}
        if self.ch2f() != 0 { try!(write!(f, " ch2f"))}
        if self.ch3f() != 0 { try!(write!(f, " ch3f"))}
        if self.ch4f() != 0 { try!(write!(f, " ch4f"))}
        if self.ch5f() != 0 { try!(write!(f, " ch5f"))}
        if self.ch6f() != 0 { try!(write!(f, " ch6f"))}
        if self.ch7f() != 0 { try!(write!(f, " ch7f"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Features Mode Selection"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mode(pub u32);
impl Mode {
    #[doc="FTM Enable"]
    #[inline] pub fn ftmen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FTMEN != 0"]
    #[inline] pub fn test_ftmen(&self) -> bool {
        self.ftmen() != 0
    }

    #[doc="Sets the FTMEN field."]
    #[inline] pub fn set_ftmen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Initialize The Channels Output"]
    #[inline] pub fn init(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if INIT != 0"]
    #[inline] pub fn test_init(&self) -> bool {
        self.init() != 0
    }

    #[doc="Sets the INIT field."]
    #[inline] pub fn set_init<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Write Protection Disable"]
    #[inline] pub fn wpdis(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if WPDIS != 0"]
    #[inline] pub fn test_wpdis(&self) -> bool {
        self.wpdis() != 0
    }

    #[doc="Sets the WPDIS field."]
    #[inline] pub fn set_wpdis<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="PWM Synchronization Mode"]
    #[inline] pub fn pwmsync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if PWMSYNC != 0"]
    #[inline] pub fn test_pwmsync(&self) -> bool {
        self.pwmsync() != 0
    }

    #[doc="Sets the PWMSYNC field."]
    #[inline] pub fn set_pwmsync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Capture Test Mode Enable"]
    #[inline] pub fn captest(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CAPTEST != 0"]
    #[inline] pub fn test_captest(&self) -> bool {
        self.captest() != 0
    }

    #[doc="Sets the CAPTEST field."]
    #[inline] pub fn set_captest<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Fault Control Mode"]
    #[inline] pub fn faultm(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x3) as u8) } // [6:5]
    }

    #[doc="Returns true if FAULTM != 0"]
    #[inline] pub fn test_faultm(&self) -> bool {
        self.faultm() != 0
    }

    #[doc="Sets the FAULTM field."]
    #[inline] pub fn set_faultm<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Fault Interrupt Enable"]
    #[inline] pub fn faultie(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FAULTIE != 0"]
    #[inline] pub fn test_faultie(&self) -> bool {
        self.faultie() != 0
    }

    #[doc="Sets the FAULTIE field."]
    #[inline] pub fn set_faultie<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Mode {
    #[inline]
    fn from(other: u32) -> Self {
         Mode(other)
    }
}

impl ::core::fmt::Display for Mode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ftmen() != 0 { try!(write!(f, " ftmen"))}
        if self.init() != 0 { try!(write!(f, " init"))}
        if self.wpdis() != 0 { try!(write!(f, " wpdis"))}
        if self.pwmsync() != 0 { try!(write!(f, " pwmsync"))}
        if self.captest() != 0 { try!(write!(f, " captest"))}
        if self.faultm() != 0 { try!(write!(f, " faultm=0x{:x}", self.faultm()))}
        if self.faultie() != 0 { try!(write!(f, " faultie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Sync(pub u32);
impl Sync {
    #[doc="Minimum Loading Point Enable"]
    #[inline] pub fn cntmin(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CNTMIN != 0"]
    #[inline] pub fn test_cntmin(&self) -> bool {
        self.cntmin() != 0
    }

    #[doc="Sets the CNTMIN field."]
    #[inline] pub fn set_cntmin<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Maximum Loading Point Enable"]
    #[inline] pub fn cntmax(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CNTMAX != 0"]
    #[inline] pub fn test_cntmax(&self) -> bool {
        self.cntmax() != 0
    }

    #[doc="Sets the CNTMAX field."]
    #[inline] pub fn set_cntmax<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="FTM Counter Reinitialization By Synchronization (FTM counter synchronization)"]
    #[inline] pub fn reinit(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if REINIT != 0"]
    #[inline] pub fn test_reinit(&self) -> bool {
        self.reinit() != 0
    }

    #[doc="Sets the REINIT field."]
    #[inline] pub fn set_reinit<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Output Mask Synchronization"]
    #[inline] pub fn synchom(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if SYNCHOM != 0"]
    #[inline] pub fn test_synchom(&self) -> bool {
        self.synchom() != 0
    }

    #[doc="Sets the SYNCHOM field."]
    #[inline] pub fn set_synchom<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="PWM Synchronization Hardware Trigger 0"]
    #[inline] pub fn trig0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if TRIG0 != 0"]
    #[inline] pub fn test_trig0(&self) -> bool {
        self.trig0() != 0
    }

    #[doc="Sets the TRIG0 field."]
    #[inline] pub fn set_trig0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="PWM Synchronization Hardware Trigger 1"]
    #[inline] pub fn trig1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if TRIG1 != 0"]
    #[inline] pub fn test_trig1(&self) -> bool {
        self.trig1() != 0
    }

    #[doc="Sets the TRIG1 field."]
    #[inline] pub fn set_trig1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="PWM Synchronization Hardware Trigger 2"]
    #[inline] pub fn trig2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if TRIG2 != 0"]
    #[inline] pub fn test_trig2(&self) -> bool {
        self.trig2() != 0
    }

    #[doc="Sets the TRIG2 field."]
    #[inline] pub fn set_trig2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="PWM Synchronization Software Trigger"]
    #[inline] pub fn swsync(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SWSYNC != 0"]
    #[inline] pub fn test_swsync(&self) -> bool {
        self.swsync() != 0
    }

    #[doc="Sets the SWSYNC field."]
    #[inline] pub fn set_swsync<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Sync {
    #[inline]
    fn from(other: u32) -> Self {
         Sync(other)
    }
}

impl ::core::fmt::Display for Sync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Sync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.cntmin() != 0 { try!(write!(f, " cntmin"))}
        if self.cntmax() != 0 { try!(write!(f, " cntmax"))}
        if self.reinit() != 0 { try!(write!(f, " reinit"))}
        if self.synchom() != 0 { try!(write!(f, " synchom"))}
        if self.trig0() != 0 { try!(write!(f, " trig0"))}
        if self.trig1() != 0 { try!(write!(f, " trig1"))}
        if self.trig2() != 0 { try!(write!(f, " trig2"))}
        if self.swsync() != 0 { try!(write!(f, " swsync"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Initial State For Channels Output"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Outinit(pub u32);
impl Outinit {
    #[doc="Channel 0 Output Initialization Value"]
    #[inline] pub fn ch0oi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH0OI != 0"]
    #[inline] pub fn test_ch0oi(&self) -> bool {
        self.ch0oi() != 0
    }

    #[doc="Sets the CH0OI field."]
    #[inline] pub fn set_ch0oi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 1 Output Initialization Value"]
    #[inline] pub fn ch1oi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH1OI != 0"]
    #[inline] pub fn test_ch1oi(&self) -> bool {
        self.ch1oi() != 0
    }

    #[doc="Sets the CH1OI field."]
    #[inline] pub fn set_ch1oi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel 2 Output Initialization Value"]
    #[inline] pub fn ch2oi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH2OI != 0"]
    #[inline] pub fn test_ch2oi(&self) -> bool {
        self.ch2oi() != 0
    }

    #[doc="Sets the CH2OI field."]
    #[inline] pub fn set_ch2oi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Channel 3 Output Initialization Value"]
    #[inline] pub fn ch3oi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH3OI != 0"]
    #[inline] pub fn test_ch3oi(&self) -> bool {
        self.ch3oi() != 0
    }

    #[doc="Sets the CH3OI field."]
    #[inline] pub fn set_ch3oi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel 4 Output Initialization Value"]
    #[inline] pub fn ch4oi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH4OI != 0"]
    #[inline] pub fn test_ch4oi(&self) -> bool {
        self.ch4oi() != 0
    }

    #[doc="Sets the CH4OI field."]
    #[inline] pub fn set_ch4oi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 5 Output Initialization Value"]
    #[inline] pub fn ch5oi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH5OI != 0"]
    #[inline] pub fn test_ch5oi(&self) -> bool {
        self.ch5oi() != 0
    }

    #[doc="Sets the CH5OI field."]
    #[inline] pub fn set_ch5oi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel 6 Output Initialization Value"]
    #[inline] pub fn ch6oi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CH6OI != 0"]
    #[inline] pub fn test_ch6oi(&self) -> bool {
        self.ch6oi() != 0
    }

    #[doc="Sets the CH6OI field."]
    #[inline] pub fn set_ch6oi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel 7 Output Initialization Value"]
    #[inline] pub fn ch7oi(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CH7OI != 0"]
    #[inline] pub fn test_ch7oi(&self) -> bool {
        self.ch7oi() != 0
    }

    #[doc="Sets the CH7OI field."]
    #[inline] pub fn set_ch7oi<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Outinit {
    #[inline]
    fn from(other: u32) -> Self {
         Outinit(other)
    }
}

impl ::core::fmt::Display for Outinit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Outinit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch0oi() != 0 { try!(write!(f, " ch0oi"))}
        if self.ch1oi() != 0 { try!(write!(f, " ch1oi"))}
        if self.ch2oi() != 0 { try!(write!(f, " ch2oi"))}
        if self.ch3oi() != 0 { try!(write!(f, " ch3oi"))}
        if self.ch4oi() != 0 { try!(write!(f, " ch4oi"))}
        if self.ch5oi() != 0 { try!(write!(f, " ch5oi"))}
        if self.ch6oi() != 0 { try!(write!(f, " ch6oi"))}
        if self.ch7oi() != 0 { try!(write!(f, " ch7oi"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Output Mask"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Outmask(pub u32);
impl Outmask {
    #[doc="Channel 0 Output Mask"]
    #[inline] pub fn ch0om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH0OM != 0"]
    #[inline] pub fn test_ch0om(&self) -> bool {
        self.ch0om() != 0
    }

    #[doc="Sets the CH0OM field."]
    #[inline] pub fn set_ch0om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 1 Output Mask"]
    #[inline] pub fn ch1om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH1OM != 0"]
    #[inline] pub fn test_ch1om(&self) -> bool {
        self.ch1om() != 0
    }

    #[doc="Sets the CH1OM field."]
    #[inline] pub fn set_ch1om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel 2 Output Mask"]
    #[inline] pub fn ch2om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH2OM != 0"]
    #[inline] pub fn test_ch2om(&self) -> bool {
        self.ch2om() != 0
    }

    #[doc="Sets the CH2OM field."]
    #[inline] pub fn set_ch2om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Channel 3 Output Mask"]
    #[inline] pub fn ch3om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH3OM != 0"]
    #[inline] pub fn test_ch3om(&self) -> bool {
        self.ch3om() != 0
    }

    #[doc="Sets the CH3OM field."]
    #[inline] pub fn set_ch3om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel 4 Output Mask"]
    #[inline] pub fn ch4om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH4OM != 0"]
    #[inline] pub fn test_ch4om(&self) -> bool {
        self.ch4om() != 0
    }

    #[doc="Sets the CH4OM field."]
    #[inline] pub fn set_ch4om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 5 Output Mask"]
    #[inline] pub fn ch5om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH5OM != 0"]
    #[inline] pub fn test_ch5om(&self) -> bool {
        self.ch5om() != 0
    }

    #[doc="Sets the CH5OM field."]
    #[inline] pub fn set_ch5om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel 6 Output Mask"]
    #[inline] pub fn ch6om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CH6OM != 0"]
    #[inline] pub fn test_ch6om(&self) -> bool {
        self.ch6om() != 0
    }

    #[doc="Sets the CH6OM field."]
    #[inline] pub fn set_ch6om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel 7 Output Mask"]
    #[inline] pub fn ch7om(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CH7OM != 0"]
    #[inline] pub fn test_ch7om(&self) -> bool {
        self.ch7om() != 0
    }

    #[doc="Sets the CH7OM field."]
    #[inline] pub fn set_ch7om<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Outmask {
    #[inline]
    fn from(other: u32) -> Self {
         Outmask(other)
    }
}

impl ::core::fmt::Display for Outmask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Outmask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch0om() != 0 { try!(write!(f, " ch0om"))}
        if self.ch1om() != 0 { try!(write!(f, " ch1om"))}
        if self.ch2om() != 0 { try!(write!(f, " ch2om"))}
        if self.ch3om() != 0 { try!(write!(f, " ch3om"))}
        if self.ch4om() != 0 { try!(write!(f, " ch4om"))}
        if self.ch5om() != 0 { try!(write!(f, " ch5om"))}
        if self.ch6om() != 0 { try!(write!(f, " ch6om"))}
        if self.ch7om() != 0 { try!(write!(f, " ch7om"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Function For Linked Channels"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Combine(pub u32);
impl Combine {
    #[doc="Combine Channels For n = 0"]
    #[inline] pub fn combine0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if COMBINE0 != 0"]
    #[inline] pub fn test_combine0(&self) -> bool {
        self.combine0() != 0
    }

    #[doc="Sets the COMBINE0 field."]
    #[inline] pub fn set_combine0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Complement Of Channel (n) For n = 0"]
    #[inline] pub fn comp0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if COMP0 != 0"]
    #[inline] pub fn test_comp0(&self) -> bool {
        self.comp0() != 0
    }

    #[doc="Sets the COMP0 field."]
    #[inline] pub fn set_comp0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Dual Edge Capture Mode Enable For n = 0"]
    #[inline] pub fn decapen0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if DECAPEN0 != 0"]
    #[inline] pub fn test_decapen0(&self) -> bool {
        self.decapen0() != 0
    }

    #[doc="Sets the DECAPEN0 field."]
    #[inline] pub fn set_decapen0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Dual Edge Capture Mode Captures For n = 0"]
    #[inline] pub fn decap0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if DECAP0 != 0"]
    #[inline] pub fn test_decap0(&self) -> bool {
        self.decap0() != 0
    }

    #[doc="Sets the DECAP0 field."]
    #[inline] pub fn set_decap0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Deadtime Enable For n = 0"]
    #[inline] pub fn dten0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if DTEN0 != 0"]
    #[inline] pub fn test_dten0(&self) -> bool {
        self.dten0() != 0
    }

    #[doc="Sets the DTEN0 field."]
    #[inline] pub fn set_dten0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Synchronization Enable For n = 0"]
    #[inline] pub fn syncen0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SYNCEN0 != 0"]
    #[inline] pub fn test_syncen0(&self) -> bool {
        self.syncen0() != 0
    }

    #[doc="Sets the SYNCEN0 field."]
    #[inline] pub fn set_syncen0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Fault Control Enable For n = 0"]
    #[inline] pub fn faulten0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FAULTEN0 != 0"]
    #[inline] pub fn test_faulten0(&self) -> bool {
        self.faulten0() != 0
    }

    #[doc="Sets the FAULTEN0 field."]
    #[inline] pub fn set_faulten0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Combine Channels For n = 2"]
    #[inline] pub fn combine1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if COMBINE1 != 0"]
    #[inline] pub fn test_combine1(&self) -> bool {
        self.combine1() != 0
    }

    #[doc="Sets the COMBINE1 field."]
    #[inline] pub fn set_combine1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Complement Of Channel (n) For n = 2"]
    #[inline] pub fn comp1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if COMP1 != 0"]
    #[inline] pub fn test_comp1(&self) -> bool {
        self.comp1() != 0
    }

    #[doc="Sets the COMP1 field."]
    #[inline] pub fn set_comp1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Dual Edge Capture Mode Enable For n = 2"]
    #[inline] pub fn decapen1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if DECAPEN1 != 0"]
    #[inline] pub fn test_decapen1(&self) -> bool {
        self.decapen1() != 0
    }

    #[doc="Sets the DECAPEN1 field."]
    #[inline] pub fn set_decapen1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Dual Edge Capture Mode Captures For n = 2"]
    #[inline] pub fn decap1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if DECAP1 != 0"]
    #[inline] pub fn test_decap1(&self) -> bool {
        self.decap1() != 0
    }

    #[doc="Sets the DECAP1 field."]
    #[inline] pub fn set_decap1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Deadtime Enable For n = 2"]
    #[inline] pub fn dten1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if DTEN1 != 0"]
    #[inline] pub fn test_dten1(&self) -> bool {
        self.dten1() != 0
    }

    #[doc="Sets the DTEN1 field."]
    #[inline] pub fn set_dten1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Synchronization Enable For n = 2"]
    #[inline] pub fn syncen1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if SYNCEN1 != 0"]
    #[inline] pub fn test_syncen1(&self) -> bool {
        self.syncen1() != 0
    }

    #[doc="Sets the SYNCEN1 field."]
    #[inline] pub fn set_syncen1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Fault Control Enable For n = 2"]
    #[inline] pub fn faulten1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if FAULTEN1 != 0"]
    #[inline] pub fn test_faulten1(&self) -> bool {
        self.faulten1() != 0
    }

    #[doc="Sets the FAULTEN1 field."]
    #[inline] pub fn set_faulten1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Combine Channels For n = 4"]
    #[inline] pub fn combine2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if COMBINE2 != 0"]
    #[inline] pub fn test_combine2(&self) -> bool {
        self.combine2() != 0
    }

    #[doc="Sets the COMBINE2 field."]
    #[inline] pub fn set_combine2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="Complement Of Channel (n) For n = 4"]
    #[inline] pub fn comp2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if COMP2 != 0"]
    #[inline] pub fn test_comp2(&self) -> bool {
        self.comp2() != 0
    }

    #[doc="Sets the COMP2 field."]
    #[inline] pub fn set_comp2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Dual Edge Capture Mode Enable For n = 4"]
    #[inline] pub fn decapen2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if DECAPEN2 != 0"]
    #[inline] pub fn test_decapen2(&self) -> bool {
        self.decapen2() != 0
    }

    #[doc="Sets the DECAPEN2 field."]
    #[inline] pub fn set_decapen2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Dual Edge Capture Mode Captures For n = 4"]
    #[inline] pub fn decap2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if DECAP2 != 0"]
    #[inline] pub fn test_decap2(&self) -> bool {
        self.decap2() != 0
    }

    #[doc="Sets the DECAP2 field."]
    #[inline] pub fn set_decap2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Deadtime Enable For n = 4"]
    #[inline] pub fn dten2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if DTEN2 != 0"]
    #[inline] pub fn test_dten2(&self) -> bool {
        self.dten2() != 0
    }

    #[doc="Sets the DTEN2 field."]
    #[inline] pub fn set_dten2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="Synchronization Enable For n = 4"]
    #[inline] pub fn syncen2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x1) as u8) } // [21]
    }

    #[doc="Returns true if SYNCEN2 != 0"]
    #[inline] pub fn test_syncen2(&self) -> bool {
        self.syncen2() != 0
    }

    #[doc="Sets the SYNCEN2 field."]
    #[inline] pub fn set_syncen2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="Fault Control Enable For n = 4"]
    #[inline] pub fn faulten2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 22) & 0x1) as u8) } // [22]
    }

    #[doc="Returns true if FAULTEN2 != 0"]
    #[inline] pub fn test_faulten2(&self) -> bool {
        self.faulten2() != 0
    }

    #[doc="Sets the FAULTEN2 field."]
    #[inline] pub fn set_faulten2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 22);
        self.0 |= value << 22;
        self
    }

    #[doc="Combine Channels For n = 6"]
    #[inline] pub fn combine3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x1) as u8) } // [24]
    }

    #[doc="Returns true if COMBINE3 != 0"]
    #[inline] pub fn test_combine3(&self) -> bool {
        self.combine3() != 0
    }

    #[doc="Sets the COMBINE3 field."]
    #[inline] pub fn set_combine3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="Complement Of Channel (n) for n = 6"]
    #[inline] pub fn comp3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 25) & 0x1) as u8) } // [25]
    }

    #[doc="Returns true if COMP3 != 0"]
    #[inline] pub fn test_comp3(&self) -> bool {
        self.comp3() != 0
    }

    #[doc="Sets the COMP3 field."]
    #[inline] pub fn set_comp3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 25);
        self.0 |= value << 25;
        self
    }

    #[doc="Dual Edge Capture Mode Enable For n = 6"]
    #[inline] pub fn decapen3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if DECAPEN3 != 0"]
    #[inline] pub fn test_decapen3(&self) -> bool {
        self.decapen3() != 0
    }

    #[doc="Sets the DECAPEN3 field."]
    #[inline] pub fn set_decapen3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="Dual Edge Capture Mode Captures For n = 6"]
    #[inline] pub fn decap3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if DECAP3 != 0"]
    #[inline] pub fn test_decap3(&self) -> bool {
        self.decap3() != 0
    }

    #[doc="Sets the DECAP3 field."]
    #[inline] pub fn set_decap3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Deadtime Enable For n = 6"]
    #[inline] pub fn dten3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if DTEN3 != 0"]
    #[inline] pub fn test_dten3(&self) -> bool {
        self.dten3() != 0
    }

    #[doc="Sets the DTEN3 field."]
    #[inline] pub fn set_dten3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Synchronization Enable For n = 6"]
    #[inline] pub fn syncen3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if SYNCEN3 != 0"]
    #[inline] pub fn test_syncen3(&self) -> bool {
        self.syncen3() != 0
    }

    #[doc="Sets the SYNCEN3 field."]
    #[inline] pub fn set_syncen3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Fault Control Enable For n = 6"]
    #[inline] pub fn faulten3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if FAULTEN3 != 0"]
    #[inline] pub fn test_faulten3(&self) -> bool {
        self.faulten3() != 0
    }

    #[doc="Sets the FAULTEN3 field."]
    #[inline] pub fn set_faulten3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

}

impl From<u32> for Combine {
    #[inline]
    fn from(other: u32) -> Self {
         Combine(other)
    }
}

impl ::core::fmt::Display for Combine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Combine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.combine0() != 0 { try!(write!(f, " combine0"))}
        if self.comp0() != 0 { try!(write!(f, " comp0"))}
        if self.decapen0() != 0 { try!(write!(f, " decapen0"))}
        if self.decap0() != 0 { try!(write!(f, " decap0"))}
        if self.dten0() != 0 { try!(write!(f, " dten0"))}
        if self.syncen0() != 0 { try!(write!(f, " syncen0"))}
        if self.faulten0() != 0 { try!(write!(f, " faulten0"))}
        if self.combine1() != 0 { try!(write!(f, " combine1"))}
        if self.comp1() != 0 { try!(write!(f, " comp1"))}
        if self.decapen1() != 0 { try!(write!(f, " decapen1"))}
        if self.decap1() != 0 { try!(write!(f, " decap1"))}
        if self.dten1() != 0 { try!(write!(f, " dten1"))}
        if self.syncen1() != 0 { try!(write!(f, " syncen1"))}
        if self.faulten1() != 0 { try!(write!(f, " faulten1"))}
        if self.combine2() != 0 { try!(write!(f, " combine2"))}
        if self.comp2() != 0 { try!(write!(f, " comp2"))}
        if self.decapen2() != 0 { try!(write!(f, " decapen2"))}
        if self.decap2() != 0 { try!(write!(f, " decap2"))}
        if self.dten2() != 0 { try!(write!(f, " dten2"))}
        if self.syncen2() != 0 { try!(write!(f, " syncen2"))}
        if self.faulten2() != 0 { try!(write!(f, " faulten2"))}
        if self.combine3() != 0 { try!(write!(f, " combine3"))}
        if self.comp3() != 0 { try!(write!(f, " comp3"))}
        if self.decapen3() != 0 { try!(write!(f, " decapen3"))}
        if self.decap3() != 0 { try!(write!(f, " decap3"))}
        if self.dten3() != 0 { try!(write!(f, " dten3"))}
        if self.syncen3() != 0 { try!(write!(f, " syncen3"))}
        if self.faulten3() != 0 { try!(write!(f, " faulten3"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Deadtime Insertion Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Deadtime(pub u32);
impl Deadtime {
    #[doc="Deadtime Value"]
    #[inline] pub fn dtval(&self) -> ::bobbin_bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3f) as u8) } // [5:0]
    }

    #[doc="Returns true if DTVAL != 0"]
    #[inline] pub fn test_dtval(&self) -> bool {
        self.dtval() != 0
    }

    #[doc="Sets the DTVAL field."]
    #[inline] pub fn set_dtval<V: Into<::bobbin_bits::U6>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Deadtime Prescaler Value"]
    #[inline] pub fn dtps(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if DTPS != 0"]
    #[inline] pub fn test_dtps(&self) -> bool {
        self.dtps() != 0
    }

    #[doc="Sets the DTPS field."]
    #[inline] pub fn set_dtps<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

}

impl From<u32> for Deadtime {
    #[inline]
    fn from(other: u32) -> Self {
         Deadtime(other)
    }
}

impl ::core::fmt::Display for Deadtime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Deadtime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dtval() != 0 { try!(write!(f, " dtval=0x{:x}", self.dtval()))}
        if self.dtps() != 0 { try!(write!(f, " dtps=0x{:x}", self.dtps()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FTM External Trigger"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Exttrig(pub u32);
impl Exttrig {
    #[doc="Channel 2 Trigger Enable"]
    #[inline] pub fn ch2trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH2TRIG != 0"]
    #[inline] pub fn test_ch2trig(&self) -> bool {
        self.ch2trig() != 0
    }

    #[doc="Sets the CH2TRIG field."]
    #[inline] pub fn set_ch2trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 3 Trigger Enable"]
    #[inline] pub fn ch3trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH3TRIG != 0"]
    #[inline] pub fn test_ch3trig(&self) -> bool {
        self.ch3trig() != 0
    }

    #[doc="Sets the CH3TRIG field."]
    #[inline] pub fn set_ch3trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel 4 Trigger Enable"]
    #[inline] pub fn ch4trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH4TRIG != 0"]
    #[inline] pub fn test_ch4trig(&self) -> bool {
        self.ch4trig() != 0
    }

    #[doc="Sets the CH4TRIG field."]
    #[inline] pub fn set_ch4trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Channel 5 Trigger Enable"]
    #[inline] pub fn ch5trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH5TRIG != 0"]
    #[inline] pub fn test_ch5trig(&self) -> bool {
        self.ch5trig() != 0
    }

    #[doc="Sets the CH5TRIG field."]
    #[inline] pub fn set_ch5trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel 0 Trigger Enable"]
    #[inline] pub fn ch0trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH0TRIG != 0"]
    #[inline] pub fn test_ch0trig(&self) -> bool {
        self.ch0trig() != 0
    }

    #[doc="Sets the CH0TRIG field."]
    #[inline] pub fn set_ch0trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 1 Trigger Enable"]
    #[inline] pub fn ch1trig(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH1TRIG != 0"]
    #[inline] pub fn test_ch1trig(&self) -> bool {
        self.ch1trig() != 0
    }

    #[doc="Sets the CH1TRIG field."]
    #[inline] pub fn set_ch1trig<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Initialization Trigger Enable"]
    #[inline] pub fn inittrigen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if INITTRIGEN != 0"]
    #[inline] pub fn test_inittrigen(&self) -> bool {
        self.inittrigen() != 0
    }

    #[doc="Sets the INITTRIGEN field."]
    #[inline] pub fn set_inittrigen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel Trigger Flag"]
    #[inline] pub fn trigf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TRIGF != 0"]
    #[inline] pub fn test_trigf(&self) -> bool {
        self.trigf() != 0
    }

    #[doc="Sets the TRIGF field."]
    #[inline] pub fn set_trigf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Exttrig {
    #[inline]
    fn from(other: u32) -> Self {
         Exttrig(other)
    }
}

impl ::core::fmt::Display for Exttrig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Exttrig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch2trig() != 0 { try!(write!(f, " ch2trig"))}
        if self.ch3trig() != 0 { try!(write!(f, " ch3trig"))}
        if self.ch4trig() != 0 { try!(write!(f, " ch4trig"))}
        if self.ch5trig() != 0 { try!(write!(f, " ch5trig"))}
        if self.ch0trig() != 0 { try!(write!(f, " ch0trig"))}
        if self.ch1trig() != 0 { try!(write!(f, " ch1trig"))}
        if self.inittrigen() != 0 { try!(write!(f, " inittrigen"))}
        if self.trigf() != 0 { try!(write!(f, " trigf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Channels Polarity"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pol(pub u32);
impl Pol {
    #[doc="Channel 0 Polarity"]
    #[inline] pub fn pol0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if POL0 != 0"]
    #[inline] pub fn test_pol0(&self) -> bool {
        self.pol0() != 0
    }

    #[doc="Sets the POL0 field."]
    #[inline] pub fn set_pol0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 1 Polarity"]
    #[inline] pub fn pol1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if POL1 != 0"]
    #[inline] pub fn test_pol1(&self) -> bool {
        self.pol1() != 0
    }

    #[doc="Sets the POL1 field."]
    #[inline] pub fn set_pol1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel 2 Polarity"]
    #[inline] pub fn pol2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if POL2 != 0"]
    #[inline] pub fn test_pol2(&self) -> bool {
        self.pol2() != 0
    }

    #[doc="Sets the POL2 field."]
    #[inline] pub fn set_pol2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Channel 3 Polarity"]
    #[inline] pub fn pol3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if POL3 != 0"]
    #[inline] pub fn test_pol3(&self) -> bool {
        self.pol3() != 0
    }

    #[doc="Sets the POL3 field."]
    #[inline] pub fn set_pol3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel 4 Polarity"]
    #[inline] pub fn pol4(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if POL4 != 0"]
    #[inline] pub fn test_pol4(&self) -> bool {
        self.pol4() != 0
    }

    #[doc="Sets the POL4 field."]
    #[inline] pub fn set_pol4<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 5 Polarity"]
    #[inline] pub fn pol5(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if POL5 != 0"]
    #[inline] pub fn test_pol5(&self) -> bool {
        self.pol5() != 0
    }

    #[doc="Sets the POL5 field."]
    #[inline] pub fn set_pol5<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel 6 Polarity"]
    #[inline] pub fn pol6(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if POL6 != 0"]
    #[inline] pub fn test_pol6(&self) -> bool {
        self.pol6() != 0
    }

    #[doc="Sets the POL6 field."]
    #[inline] pub fn set_pol6<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel 7 Polarity"]
    #[inline] pub fn pol7(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if POL7 != 0"]
    #[inline] pub fn test_pol7(&self) -> bool {
        self.pol7() != 0
    }

    #[doc="Sets the POL7 field."]
    #[inline] pub fn set_pol7<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Pol {
    #[inline]
    fn from(other: u32) -> Self {
         Pol(other)
    }
}

impl ::core::fmt::Display for Pol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.pol0() != 0 { try!(write!(f, " pol0"))}
        if self.pol1() != 0 { try!(write!(f, " pol1"))}
        if self.pol2() != 0 { try!(write!(f, " pol2"))}
        if self.pol3() != 0 { try!(write!(f, " pol3"))}
        if self.pol4() != 0 { try!(write!(f, " pol4"))}
        if self.pol5() != 0 { try!(write!(f, " pol5"))}
        if self.pol6() != 0 { try!(write!(f, " pol6"))}
        if self.pol7() != 0 { try!(write!(f, " pol7"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Fault Mode Status"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fms(pub u32);
impl Fms {
    #[doc="Fault Detection Flag 0"]
    #[inline] pub fn faultf0(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FAULTF0 != 0"]
    #[inline] pub fn test_faultf0(&self) -> bool {
        self.faultf0() != 0
    }

    #[doc="Sets the FAULTF0 field."]
    #[inline] pub fn set_faultf0<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fault Detection Flag 1"]
    #[inline] pub fn faultf1(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FAULTF1 != 0"]
    #[inline] pub fn test_faultf1(&self) -> bool {
        self.faultf1() != 0
    }

    #[doc="Sets the FAULTF1 field."]
    #[inline] pub fn set_faultf1<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Fault Detection Flag 2"]
    #[inline] pub fn faultf2(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FAULTF2 != 0"]
    #[inline] pub fn test_faultf2(&self) -> bool {
        self.faultf2() != 0
    }

    #[doc="Sets the FAULTF2 field."]
    #[inline] pub fn set_faultf2<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Fault Detection Flag 3"]
    #[inline] pub fn faultf3(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FAULTF3 != 0"]
    #[inline] pub fn test_faultf3(&self) -> bool {
        self.faultf3() != 0
    }

    #[doc="Sets the FAULTF3 field."]
    #[inline] pub fn set_faultf3<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Fault Inputs"]
    #[inline] pub fn faultin(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FAULTIN != 0"]
    #[inline] pub fn test_faultin(&self) -> bool {
        self.faultin() != 0
    }

    #[doc="Sets the FAULTIN field."]
    #[inline] pub fn set_faultin<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Write Protection Enable"]
    #[inline] pub fn wpen(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if WPEN != 0"]
    #[inline] pub fn test_wpen(&self) -> bool {
        self.wpen() != 0
    }

    #[doc="Sets the WPEN field."]
    #[inline] pub fn set_wpen<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Fault Detection Flag"]
    #[inline] pub fn faultf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FAULTF != 0"]
    #[inline] pub fn test_faultf(&self) -> bool {
        self.faultf() != 0
    }

    #[doc="Sets the FAULTF field."]
    #[inline] pub fn set_faultf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

}

impl From<u32> for Fms {
    #[inline]
    fn from(other: u32) -> Self {
         Fms(other)
    }
}

impl ::core::fmt::Display for Fms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.faultf0() != 0 { try!(write!(f, " faultf0"))}
        if self.faultf1() != 0 { try!(write!(f, " faultf1"))}
        if self.faultf2() != 0 { try!(write!(f, " faultf2"))}
        if self.faultf3() != 0 { try!(write!(f, " faultf3"))}
        if self.faultin() != 0 { try!(write!(f, " faultin"))}
        if self.wpen() != 0 { try!(write!(f, " wpen"))}
        if self.faultf() != 0 { try!(write!(f, " faultf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Input Capture Filter Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Filter(pub u32);
impl Filter {
    #[doc="Channel 0 Input Filter"]
    #[inline] pub fn ch0fval(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if CH0FVAL != 0"]
    #[inline] pub fn test_ch0fval(&self) -> bool {
        self.ch0fval() != 0
    }

    #[doc="Sets the CH0FVAL field."]
    #[inline] pub fn set_ch0fval<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 1 Input Filter"]
    #[inline] pub fn ch1fval(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0xf) as u8) } // [7:4]
    }

    #[doc="Returns true if CH1FVAL != 0"]
    #[inline] pub fn test_ch1fval(&self) -> bool {
        self.ch1fval() != 0
    }

    #[doc="Sets the CH1FVAL field."]
    #[inline] pub fn set_ch1fval<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 2 Input Filter"]
    #[inline] pub fn ch2fval(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if CH2FVAL != 0"]
    #[inline] pub fn test_ch2fval(&self) -> bool {
        self.ch2fval() != 0
    }

    #[doc="Sets the CH2FVAL field."]
    #[inline] pub fn set_ch2fval<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Channel 3 Input Filter"]
    #[inline] pub fn ch3fval(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0xf) as u8) } // [15:12]
    }

    #[doc="Returns true if CH3FVAL != 0"]
    #[inline] pub fn test_ch3fval(&self) -> bool {
        self.ch3fval() != 0
    }

    #[doc="Sets the CH3FVAL field."]
    #[inline] pub fn set_ch3fval<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 12);
        self.0 |= value << 12;
        self
    }

}

impl From<u32> for Filter {
    #[inline]
    fn from(other: u32) -> Self {
         Filter(other)
    }
}

impl ::core::fmt::Display for Filter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Filter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch0fval() != 0 { try!(write!(f, " ch0fval=0x{:x}", self.ch0fval()))}
        if self.ch1fval() != 0 { try!(write!(f, " ch1fval=0x{:x}", self.ch1fval()))}
        if self.ch2fval() != 0 { try!(write!(f, " ch2fval=0x{:x}", self.ch2fval()))}
        if self.ch3fval() != 0 { try!(write!(f, " ch3fval=0x{:x}", self.ch3fval()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Fault Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltctrl(pub u32);
impl Fltctrl {
    #[doc="Fault Input 0 Enable"]
    #[inline] pub fn fault0en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FAULT0EN != 0"]
    #[inline] pub fn test_fault0en(&self) -> bool {
        self.fault0en() != 0
    }

    #[doc="Sets the FAULT0EN field."]
    #[inline] pub fn set_fault0en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fault Input 1 Enable"]
    #[inline] pub fn fault1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FAULT1EN != 0"]
    #[inline] pub fn test_fault1en(&self) -> bool {
        self.fault1en() != 0
    }

    #[doc="Sets the FAULT1EN field."]
    #[inline] pub fn set_fault1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Fault Input 2 Enable"]
    #[inline] pub fn fault2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FAULT2EN != 0"]
    #[inline] pub fn test_fault2en(&self) -> bool {
        self.fault2en() != 0
    }

    #[doc="Sets the FAULT2EN field."]
    #[inline] pub fn set_fault2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Fault Input 3 Enable"]
    #[inline] pub fn fault3en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FAULT3EN != 0"]
    #[inline] pub fn test_fault3en(&self) -> bool {
        self.fault3en() != 0
    }

    #[doc="Sets the FAULT3EN field."]
    #[inline] pub fn set_fault3en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Fault Input 0 Filter Enable"]
    #[inline] pub fn ffltr0en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FFLTR0EN != 0"]
    #[inline] pub fn test_ffltr0en(&self) -> bool {
        self.ffltr0en() != 0
    }

    #[doc="Sets the FFLTR0EN field."]
    #[inline] pub fn set_ffltr0en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Fault Input 1 Filter Enable"]
    #[inline] pub fn ffltr1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FFLTR1EN != 0"]
    #[inline] pub fn test_ffltr1en(&self) -> bool {
        self.ffltr1en() != 0
    }

    #[doc="Sets the FFLTR1EN field."]
    #[inline] pub fn set_ffltr1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Fault Input 2 Filter Enable"]
    #[inline] pub fn ffltr2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FFLTR2EN != 0"]
    #[inline] pub fn test_ffltr2en(&self) -> bool {
        self.ffltr2en() != 0
    }

    #[doc="Sets the FFLTR2EN field."]
    #[inline] pub fn set_ffltr2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Fault Input 3 Filter Enable"]
    #[inline] pub fn ffltr3en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if FFLTR3EN != 0"]
    #[inline] pub fn test_ffltr3en(&self) -> bool {
        self.ffltr3en() != 0
    }

    #[doc="Sets the FFLTR3EN field."]
    #[inline] pub fn set_ffltr3en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Fault Input Filter"]
    #[inline] pub fn ffval(&self) -> ::bobbin_bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xf) as u8) } // [11:8]
    }

    #[doc="Returns true if FFVAL != 0"]
    #[inline] pub fn test_ffval(&self) -> bool {
        self.ffval() != 0
    }

    #[doc="Sets the FFVAL field."]
    #[inline] pub fn set_ffval<V: Into<::bobbin_bits::U4>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 8);
        self.0 |= value << 8;
        self
    }

}

impl From<u32> for Fltctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Fltctrl(other)
    }
}

impl ::core::fmt::Display for Fltctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fault0en() != 0 { try!(write!(f, " fault0en"))}
        if self.fault1en() != 0 { try!(write!(f, " fault1en"))}
        if self.fault2en() != 0 { try!(write!(f, " fault2en"))}
        if self.fault3en() != 0 { try!(write!(f, " fault3en"))}
        if self.ffltr0en() != 0 { try!(write!(f, " ffltr0en"))}
        if self.ffltr1en() != 0 { try!(write!(f, " ffltr1en"))}
        if self.ffltr2en() != 0 { try!(write!(f, " ffltr2en"))}
        if self.ffltr3en() != 0 { try!(write!(f, " ffltr3en"))}
        if self.ffval() != 0 { try!(write!(f, " ffval=0x{:x}", self.ffval()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Conf(pub u32);
impl Conf {
    #[doc="TOF Frequency"]
    #[inline] pub fn numtof(&self) -> ::bobbin_bits::U5 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1f) as u8) } // [4:0]
    }

    #[doc="Returns true if NUMTOF != 0"]
    #[inline] pub fn test_numtof(&self) -> bool {
        self.numtof() != 0
    }

    #[doc="Sets the NUMTOF field."]
    #[inline] pub fn set_numtof<V: Into<::bobbin_bits::U5>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U5 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1f << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Debug Mode"]
    #[inline] pub fn bdmmode(&self) -> ::bobbin_bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x3) as u8) } // [7:6]
    }

    #[doc="Returns true if BDMMODE != 0"]
    #[inline] pub fn test_bdmmode(&self) -> bool {
        self.bdmmode() != 0
    }

    #[doc="Sets the BDMMODE field."]
    #[inline] pub fn set_bdmmode<V: Into<::bobbin_bits::U2>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Global Time Base Enable"]
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

    #[doc="Global Time Base Output"]
    #[inline] pub fn gtbeout(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if GTBEOUT != 0"]
    #[inline] pub fn test_gtbeout(&self) -> bool {
        self.gtbeout() != 0
    }

    #[doc="Sets the GTBEOUT field."]
    #[inline] pub fn set_gtbeout<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
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
        if self.numtof() != 0 { try!(write!(f, " numtof=0x{:x}", self.numtof()))}
        if self.bdmmode() != 0 { try!(write!(f, " bdmmode=0x{:x}", self.bdmmode()))}
        if self.gtbeen() != 0 { try!(write!(f, " gtbeen"))}
        if self.gtbeout() != 0 { try!(write!(f, " gtbeout"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FTM Fault Input Polarity"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fltpol(pub u32);
impl Fltpol {
    #[doc="Fault Input 0 Polarity"]
    #[inline] pub fn flt0pol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FLT0POL != 0"]
    #[inline] pub fn test_flt0pol(&self) -> bool {
        self.flt0pol() != 0
    }

    #[doc="Sets the FLT0POL field."]
    #[inline] pub fn set_flt0pol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Fault Input 1 Polarity"]
    #[inline] pub fn flt1pol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FLT1POL != 0"]
    #[inline] pub fn test_flt1pol(&self) -> bool {
        self.flt1pol() != 0
    }

    #[doc="Sets the FLT1POL field."]
    #[inline] pub fn set_flt1pol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Fault Input 2 Polarity"]
    #[inline] pub fn flt2pol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FLT2POL != 0"]
    #[inline] pub fn test_flt2pol(&self) -> bool {
        self.flt2pol() != 0
    }

    #[doc="Sets the FLT2POL field."]
    #[inline] pub fn set_flt2pol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Fault Input 3 Polarity"]
    #[inline] pub fn flt3pol(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FLT3POL != 0"]
    #[inline] pub fn test_flt3pol(&self) -> bool {
        self.flt3pol() != 0
    }

    #[doc="Sets the FLT3POL field."]
    #[inline] pub fn set_flt3pol<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Fltpol {
    #[inline]
    fn from(other: u32) -> Self {
         Fltpol(other)
    }
}

impl ::core::fmt::Display for Fltpol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fltpol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.flt0pol() != 0 { try!(write!(f, " flt0pol"))}
        if self.flt1pol() != 0 { try!(write!(f, " flt1pol"))}
        if self.flt2pol() != 0 { try!(write!(f, " flt2pol"))}
        if self.flt3pol() != 0 { try!(write!(f, " flt3pol"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Synchronization Configuration"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Synconf(pub u32);
impl Synconf {
    #[doc="Hardware Trigger Mode"]
    #[inline] pub fn hwtrigmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if HWTRIGMODE != 0"]
    #[inline] pub fn test_hwtrigmode(&self) -> bool {
        self.hwtrigmode() != 0
    }

    #[doc="Sets the HWTRIGMODE field."]
    #[inline] pub fn set_hwtrigmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="CNTIN Register Synchronization"]
    #[inline] pub fn cntinc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CNTINC != 0"]
    #[inline] pub fn test_cntinc(&self) -> bool {
        self.cntinc() != 0
    }

    #[doc="Sets the CNTINC field."]
    #[inline] pub fn set_cntinc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="INVCTRL Register Synchronization"]
    #[inline] pub fn invc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if INVC != 0"]
    #[inline] pub fn test_invc(&self) -> bool {
        self.invc() != 0
    }

    #[doc="Sets the INVC field."]
    #[inline] pub fn set_invc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="SWOCTRL Register Synchronization"]
    #[inline] pub fn swoc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if SWOC != 0"]
    #[inline] pub fn test_swoc(&self) -> bool {
        self.swoc() != 0
    }

    #[doc="Sets the SWOC field."]
    #[inline] pub fn set_swoc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Synchronization Mode"]
    #[inline] pub fn syncmode(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if SYNCMODE != 0"]
    #[inline] pub fn test_syncmode(&self) -> bool {
        self.syncmode() != 0
    }

    #[doc="Sets the SYNCMODE field."]
    #[inline] pub fn set_syncmode<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="FTM counter synchronization is activated by the software trigger."]
    #[inline] pub fn swrstcnt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if SWRSTCNT != 0"]
    #[inline] pub fn test_swrstcnt(&self) -> bool {
        self.swrstcnt() != 0
    }

    #[doc="Sets the SWRSTCNT field."]
    #[inline] pub fn set_swrstcnt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="MOD, CNTIN, and CV registers synchronization is activated by the software trigger."]
    #[inline] pub fn swwrbuf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if SWWRBUF != 0"]
    #[inline] pub fn test_swwrbuf(&self) -> bool {
        self.swwrbuf() != 0
    }

    #[doc="Sets the SWWRBUF field."]
    #[inline] pub fn set_swwrbuf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Output mask synchronization is activated by the software trigger."]
    #[inline] pub fn swom(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SWOM != 0"]
    #[inline] pub fn test_swom(&self) -> bool {
        self.swom() != 0
    }

    #[doc="Sets the SWOM field."]
    #[inline] pub fn set_swom<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Inverting control synchronization is activated by the software trigger."]
    #[inline] pub fn swinvc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if SWINVC != 0"]
    #[inline] pub fn test_swinvc(&self) -> bool {
        self.swinvc() != 0
    }

    #[doc="Sets the SWINVC field."]
    #[inline] pub fn set_swinvc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Software output control synchronization is activated by the software trigger."]
    #[inline] pub fn swsoc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if SWSOC != 0"]
    #[inline] pub fn test_swsoc(&self) -> bool {
        self.swsoc() != 0
    }

    #[doc="Sets the SWSOC field."]
    #[inline] pub fn set_swsoc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="FTM counter synchronization is activated by a hardware trigger."]
    #[inline] pub fn hwrstcnt(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if HWRSTCNT != 0"]
    #[inline] pub fn test_hwrstcnt(&self) -> bool {
        self.hwrstcnt() != 0
    }

    #[doc="Sets the HWRSTCNT field."]
    #[inline] pub fn set_hwrstcnt<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="MOD, CNTIN, and CV registers synchronization is activated by a hardware trigger."]
    #[inline] pub fn hwwrbuf(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if HWWRBUF != 0"]
    #[inline] pub fn test_hwwrbuf(&self) -> bool {
        self.hwwrbuf() != 0
    }

    #[doc="Sets the HWWRBUF field."]
    #[inline] pub fn set_hwwrbuf<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="Output mask synchronization is activated by a hardware trigger."]
    #[inline] pub fn hwom(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if HWOM != 0"]
    #[inline] pub fn test_hwom(&self) -> bool {
        self.hwom() != 0
    }

    #[doc="Sets the HWOM field."]
    #[inline] pub fn set_hwom<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="Inverting control synchronization is activated by a hardware trigger."]
    #[inline] pub fn hwinvc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if HWINVC != 0"]
    #[inline] pub fn test_hwinvc(&self) -> bool {
        self.hwinvc() != 0
    }

    #[doc="Sets the HWINVC field."]
    #[inline] pub fn set_hwinvc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="Software output control synchronization is activated by a hardware trigger."]
    #[inline] pub fn hwsoc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x1) as u8) } // [20]
    }

    #[doc="Returns true if HWSOC != 0"]
    #[inline] pub fn test_hwsoc(&self) -> bool {
        self.hwsoc() != 0
    }

    #[doc="Sets the HWSOC field."]
    #[inline] pub fn set_hwsoc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 20);
        self.0 |= value << 20;
        self
    }

}

impl From<u32> for Synconf {
    #[inline]
    fn from(other: u32) -> Self {
         Synconf(other)
    }
}

impl ::core::fmt::Display for Synconf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Synconf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.hwtrigmode() != 0 { try!(write!(f, " hwtrigmode"))}
        if self.cntinc() != 0 { try!(write!(f, " cntinc"))}
        if self.invc() != 0 { try!(write!(f, " invc"))}
        if self.swoc() != 0 { try!(write!(f, " swoc"))}
        if self.syncmode() != 0 { try!(write!(f, " syncmode"))}
        if self.swrstcnt() != 0 { try!(write!(f, " swrstcnt"))}
        if self.swwrbuf() != 0 { try!(write!(f, " swwrbuf"))}
        if self.swom() != 0 { try!(write!(f, " swom"))}
        if self.swinvc() != 0 { try!(write!(f, " swinvc"))}
        if self.swsoc() != 0 { try!(write!(f, " swsoc"))}
        if self.hwrstcnt() != 0 { try!(write!(f, " hwrstcnt"))}
        if self.hwwrbuf() != 0 { try!(write!(f, " hwwrbuf"))}
        if self.hwom() != 0 { try!(write!(f, " hwom"))}
        if self.hwinvc() != 0 { try!(write!(f, " hwinvc"))}
        if self.hwsoc() != 0 { try!(write!(f, " hwsoc"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FTM Inverting Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Invctrl(pub u32);
impl Invctrl {
    #[doc="Pair Channels 0 Inverting Enable"]
    #[inline] pub fn inv0en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INV0EN != 0"]
    #[inline] pub fn test_inv0en(&self) -> bool {
        self.inv0en() != 0
    }

    #[doc="Sets the INV0EN field."]
    #[inline] pub fn set_inv0en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Pair Channels 1 Inverting Enable"]
    #[inline] pub fn inv1en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if INV1EN != 0"]
    #[inline] pub fn test_inv1en(&self) -> bool {
        self.inv1en() != 0
    }

    #[doc="Sets the INV1EN field."]
    #[inline] pub fn set_inv1en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Pair Channels 2 Inverting Enable"]
    #[inline] pub fn inv2en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if INV2EN != 0"]
    #[inline] pub fn test_inv2en(&self) -> bool {
        self.inv2en() != 0
    }

    #[doc="Sets the INV2EN field."]
    #[inline] pub fn set_inv2en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Pair Channels 3 Inverting Enable"]
    #[inline] pub fn inv3en(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if INV3EN != 0"]
    #[inline] pub fn test_inv3en(&self) -> bool {
        self.inv3en() != 0
    }

    #[doc="Sets the INV3EN field."]
    #[inline] pub fn set_inv3en<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

}

impl From<u32> for Invctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Invctrl(other)
    }
}

impl ::core::fmt::Display for Invctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Invctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.inv0en() != 0 { try!(write!(f, " inv0en"))}
        if self.inv1en() != 0 { try!(write!(f, " inv1en"))}
        if self.inv2en() != 0 { try!(write!(f, " inv2en"))}
        if self.inv3en() != 0 { try!(write!(f, " inv3en"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FTM Software Output Control"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Swoctrl(pub u32);
impl Swoctrl {
    #[doc="Channel 0 Software Output Control Enable"]
    #[inline] pub fn ch0oc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH0OC != 0"]
    #[inline] pub fn test_ch0oc(&self) -> bool {
        self.ch0oc() != 0
    }

    #[doc="Sets the CH0OC field."]
    #[inline] pub fn set_ch0oc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 1 Software Output Control Enable"]
    #[inline] pub fn ch1oc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH1OC != 0"]
    #[inline] pub fn test_ch1oc(&self) -> bool {
        self.ch1oc() != 0
    }

    #[doc="Sets the CH1OC field."]
    #[inline] pub fn set_ch1oc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel 2 Software Output Control Enable"]
    #[inline] pub fn ch2oc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH2OC != 0"]
    #[inline] pub fn test_ch2oc(&self) -> bool {
        self.ch2oc() != 0
    }

    #[doc="Sets the CH2OC field."]
    #[inline] pub fn set_ch2oc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Channel 3 Software Output Control Enable"]
    #[inline] pub fn ch3oc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH3OC != 0"]
    #[inline] pub fn test_ch3oc(&self) -> bool {
        self.ch3oc() != 0
    }

    #[doc="Sets the CH3OC field."]
    #[inline] pub fn set_ch3oc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel 4 Software Output Control Enable"]
    #[inline] pub fn ch4oc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH4OC != 0"]
    #[inline] pub fn test_ch4oc(&self) -> bool {
        self.ch4oc() != 0
    }

    #[doc="Sets the CH4OC field."]
    #[inline] pub fn set_ch4oc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 5 Software Output Control Enable"]
    #[inline] pub fn ch5oc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH5OC != 0"]
    #[inline] pub fn test_ch5oc(&self) -> bool {
        self.ch5oc() != 0
    }

    #[doc="Sets the CH5OC field."]
    #[inline] pub fn set_ch5oc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel 6 Software Output Control Enable"]
    #[inline] pub fn ch6oc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CH6OC != 0"]
    #[inline] pub fn test_ch6oc(&self) -> bool {
        self.ch6oc() != 0
    }

    #[doc="Sets the CH6OC field."]
    #[inline] pub fn set_ch6oc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel 7 Software Output Control Enable"]
    #[inline] pub fn ch7oc(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CH7OC != 0"]
    #[inline] pub fn test_ch7oc(&self) -> bool {
        self.ch7oc() != 0
    }

    #[doc="Sets the CH7OC field."]
    #[inline] pub fn set_ch7oc<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Channel 0 Software Output Control Value"]
    #[inline] pub fn ch0ocv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if CH0OCV != 0"]
    #[inline] pub fn test_ch0ocv(&self) -> bool {
        self.ch0ocv() != 0
    }

    #[doc="Sets the CH0OCV field."]
    #[inline] pub fn set_ch0ocv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="Channel 1 Software Output Control Value"]
    #[inline] pub fn ch1ocv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if CH1OCV != 0"]
    #[inline] pub fn test_ch1ocv(&self) -> bool {
        self.ch1ocv() != 0
    }

    #[doc="Sets the CH1OCV field."]
    #[inline] pub fn set_ch1ocv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="Channel 2 Software Output Control Value"]
    #[inline] pub fn ch2ocv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if CH2OCV != 0"]
    #[inline] pub fn test_ch2ocv(&self) -> bool {
        self.ch2ocv() != 0
    }

    #[doc="Sets the CH2OCV field."]
    #[inline] pub fn set_ch2ocv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="Channel 3 Software Output Control Value"]
    #[inline] pub fn ch3ocv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if CH3OCV != 0"]
    #[inline] pub fn test_ch3ocv(&self) -> bool {
        self.ch3ocv() != 0
    }

    #[doc="Sets the CH3OCV field."]
    #[inline] pub fn set_ch3ocv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="Channel 4 Software Output Control Value"]
    #[inline] pub fn ch4ocv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 12) & 0x1) as u8) } // [12]
    }

    #[doc="Returns true if CH4OCV != 0"]
    #[inline] pub fn test_ch4ocv(&self) -> bool {
        self.ch4ocv() != 0
    }

    #[doc="Sets the CH4OCV field."]
    #[inline] pub fn set_ch4ocv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 12);
        self.0 |= value << 12;
        self
    }

    #[doc="Channel 5 Software Output Control Value"]
    #[inline] pub fn ch5ocv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 13) & 0x1) as u8) } // [13]
    }

    #[doc="Returns true if CH5OCV != 0"]
    #[inline] pub fn test_ch5ocv(&self) -> bool {
        self.ch5ocv() != 0
    }

    #[doc="Sets the CH5OCV field."]
    #[inline] pub fn set_ch5ocv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 13);
        self.0 |= value << 13;
        self
    }

    #[doc="Channel 6 Software Output Control Value"]
    #[inline] pub fn ch6ocv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 14) & 0x1) as u8) } // [14]
    }

    #[doc="Returns true if CH6OCV != 0"]
    #[inline] pub fn test_ch6ocv(&self) -> bool {
        self.ch6ocv() != 0
    }

    #[doc="Sets the CH6OCV field."]
    #[inline] pub fn set_ch6ocv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 14);
        self.0 |= value << 14;
        self
    }

    #[doc="Channel 7 Software Output Control Value"]
    #[inline] pub fn ch7ocv(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if CH7OCV != 0"]
    #[inline] pub fn test_ch7ocv(&self) -> bool {
        self.ch7ocv() != 0
    }

    #[doc="Sets the CH7OCV field."]
    #[inline] pub fn set_ch7ocv<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

}

impl From<u32> for Swoctrl {
    #[inline]
    fn from(other: u32) -> Self {
         Swoctrl(other)
    }
}

impl ::core::fmt::Display for Swoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Swoctrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch0oc() != 0 { try!(write!(f, " ch0oc"))}
        if self.ch1oc() != 0 { try!(write!(f, " ch1oc"))}
        if self.ch2oc() != 0 { try!(write!(f, " ch2oc"))}
        if self.ch3oc() != 0 { try!(write!(f, " ch3oc"))}
        if self.ch4oc() != 0 { try!(write!(f, " ch4oc"))}
        if self.ch5oc() != 0 { try!(write!(f, " ch5oc"))}
        if self.ch6oc() != 0 { try!(write!(f, " ch6oc"))}
        if self.ch7oc() != 0 { try!(write!(f, " ch7oc"))}
        if self.ch0ocv() != 0 { try!(write!(f, " ch0ocv"))}
        if self.ch1ocv() != 0 { try!(write!(f, " ch1ocv"))}
        if self.ch2ocv() != 0 { try!(write!(f, " ch2ocv"))}
        if self.ch3ocv() != 0 { try!(write!(f, " ch3ocv"))}
        if self.ch4ocv() != 0 { try!(write!(f, " ch4ocv"))}
        if self.ch5ocv() != 0 { try!(write!(f, " ch5ocv"))}
        if self.ch6ocv() != 0 { try!(write!(f, " ch6ocv"))}
        if self.ch7ocv() != 0 { try!(write!(f, " ch7ocv"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="FTM PWM Load"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Pwmload(pub u32);
impl Pwmload {
    #[doc="Channel 0 Select"]
    #[inline] pub fn ch0sel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if CH0SEL != 0"]
    #[inline] pub fn test_ch0sel(&self) -> bool {
        self.ch0sel() != 0
    }

    #[doc="Sets the CH0SEL field."]
    #[inline] pub fn set_ch0sel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

    #[doc="Channel 1 Select"]
    #[inline] pub fn ch1sel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if CH1SEL != 0"]
    #[inline] pub fn test_ch1sel(&self) -> bool {
        self.ch1sel() != 0
    }

    #[doc="Sets the CH1SEL field."]
    #[inline] pub fn set_ch1sel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="Channel 2 Select"]
    #[inline] pub fn ch2sel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if CH2SEL != 0"]
    #[inline] pub fn test_ch2sel(&self) -> bool {
        self.ch2sel() != 0
    }

    #[doc="Sets the CH2SEL field."]
    #[inline] pub fn set_ch2sel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="Channel 3 Select"]
    #[inline] pub fn ch3sel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if CH3SEL != 0"]
    #[inline] pub fn test_ch3sel(&self) -> bool {
        self.ch3sel() != 0
    }

    #[doc="Sets the CH3SEL field."]
    #[inline] pub fn set_ch3sel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="Channel 4 Select"]
    #[inline] pub fn ch4sel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if CH4SEL != 0"]
    #[inline] pub fn test_ch4sel(&self) -> bool {
        self.ch4sel() != 0
    }

    #[doc="Sets the CH4SEL field."]
    #[inline] pub fn set_ch4sel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="Channel 5 Select"]
    #[inline] pub fn ch5sel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if CH5SEL != 0"]
    #[inline] pub fn test_ch5sel(&self) -> bool {
        self.ch5sel() != 0
    }

    #[doc="Sets the CH5SEL field."]
    #[inline] pub fn set_ch5sel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="Channel 6 Select"]
    #[inline] pub fn ch6sel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if CH6SEL != 0"]
    #[inline] pub fn test_ch6sel(&self) -> bool {
        self.ch6sel() != 0
    }

    #[doc="Sets the CH6SEL field."]
    #[inline] pub fn set_ch6sel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="Channel 7 Select"]
    #[inline] pub fn ch7sel(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if CH7SEL != 0"]
    #[inline] pub fn test_ch7sel(&self) -> bool {
        self.ch7sel() != 0
    }

    #[doc="Sets the CH7SEL field."]
    #[inline] pub fn set_ch7sel<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="Load Enable"]
    #[inline] pub fn ldok(&self) -> ::bobbin_bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if LDOK != 0"]
    #[inline] pub fn test_ldok(&self) -> bool {
        self.ldok() != 0
    }

    #[doc="Sets the LDOK field."]
    #[inline] pub fn set_ldok<V: Into<::bobbin_bits::U1>>(mut self, value: V) -> Self {
        let value: ::bobbin_bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

}

impl From<u32> for Pwmload {
    #[inline]
    fn from(other: u32) -> Self {
         Pwmload(other)
    }
}

impl ::core::fmt::Display for Pwmload {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Pwmload {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ch0sel() != 0 { try!(write!(f, " ch0sel"))}
        if self.ch1sel() != 0 { try!(write!(f, " ch1sel"))}
        if self.ch2sel() != 0 { try!(write!(f, " ch2sel"))}
        if self.ch3sel() != 0 { try!(write!(f, " ch3sel"))}
        if self.ch4sel() != 0 { try!(write!(f, " ch4sel"))}
        if self.ch5sel() != 0 { try!(write!(f, " ch5sel"))}
        if self.ch6sel() != 0 { try!(write!(f, " ch6sel"))}
        if self.ch7sel() != 0 { try!(write!(f, " ch7sel"))}
        if self.ldok() != 0 { try!(write!(f, " ldok"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

