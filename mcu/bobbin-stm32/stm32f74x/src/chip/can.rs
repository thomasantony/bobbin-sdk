#[allow(unused_imports)] use bobbin_common::*;

periph!( CAN1, Can1, _CAN1, CanPeriph, 0x40006400);
periph!( CAN2, Can2, _CAN2, CanPeriph, 0x40006800);

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc="CAN Peripheral"]
pub struct CanPeriph(pub usize); 

impl super::sig::Signal<super::sig::Can1Tx> for Can1 {}
impl super::sig::SignalCanTx<super::sig::Can1Tx> for Can1 {}
impl super::sig::Signal<super::sig::Can1Rx> for Can1 {}
impl super::sig::SignalCanRx<super::sig::Can1Rx> for Can1 {}

impl super::sig::Signal<super::sig::Can2Tx> for Can2 {}
impl super::sig::SignalCanTx<super::sig::Can2Tx> for Can2 {}
impl super::sig::Signal<super::sig::Can2Rx> for Can2 {}
impl super::sig::SignalCanRx<super::sig::Can2Rx> for Can2 {}


impl CanPeriph {
    #[doc="Get the *mut pointer for the MCR register."]
    #[inline] pub fn mcr_mut(&self) -> *mut Mcr { 
        (self.0 + 0x0) as *mut Mcr
    }

    #[doc="Get the *const pointer for the MCR register."]
    #[inline] pub fn mcr_ptr(&self) -> *const Mcr { 
           self.mcr_mut()
    }

    #[doc="Read the MCR register."]
    #[inline] pub fn mcr(&self) -> Mcr { 
        unsafe {
            read_volatile(self.mcr_ptr())
        }
    }

    #[doc="Write the MCR register."]
    #[inline] pub fn set_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mcr_mut(), f(Mcr(0)));
        }
        self
    }

    #[doc="Modify the MCR register."]
    #[inline] pub fn with_mcr<F: FnOnce(Mcr) -> Mcr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.mcr_mut(), f(self.mcr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the MSR register."]
    #[inline] pub fn msr_mut(&self) -> *mut Msr { 
        (self.0 + 0x4) as *mut Msr
    }

    #[doc="Get the *const pointer for the MSR register."]
    #[inline] pub fn msr_ptr(&self) -> *const Msr { 
           self.msr_mut()
    }

    #[doc="Read the MSR register."]
    #[inline] pub fn msr(&self) -> Msr { 
        unsafe {
            read_volatile(self.msr_ptr())
        }
    }

    #[doc="Write the MSR register."]
    #[inline] pub fn set_msr<F: FnOnce(Msr) -> Msr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.msr_mut(), f(Msr(0)));
        }
        self
    }

    #[doc="Modify the MSR register."]
    #[inline] pub fn with_msr<F: FnOnce(Msr) -> Msr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.msr_mut(), f(self.msr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TSR register."]
    #[inline] pub fn tsr_mut(&self) -> *mut Tsr { 
        (self.0 + 0x8) as *mut Tsr
    }

    #[doc="Get the *const pointer for the TSR register."]
    #[inline] pub fn tsr_ptr(&self) -> *const Tsr { 
           self.tsr_mut()
    }

    #[doc="Read the TSR register."]
    #[inline] pub fn tsr(&self) -> Tsr { 
        unsafe {
            read_volatile(self.tsr_ptr())
        }
    }

    #[doc="Write the TSR register."]
    #[inline] pub fn set_tsr<F: FnOnce(Tsr) -> Tsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tsr_mut(), f(Tsr(0)));
        }
        self
    }

    #[doc="Modify the TSR register."]
    #[inline] pub fn with_tsr<F: FnOnce(Tsr) -> Tsr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.tsr_mut(), f(self.tsr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the RFR register."]
    #[inline] pub fn rfr_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Rfr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0xc + (index << 2)) as *mut Rfr
    }

    #[doc="Get the *const pointer for the RFR register."]
    #[inline] pub fn rfr_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Rfr { 
           self.rfr_mut(index)
    }

    #[doc="Read the RFR register."]
    #[inline] pub fn rfr<I: Into<bits::R2>>(&self, index: I) -> Rfr { 
        unsafe {
            read_volatile(self.rfr_ptr(index))
        }
    }

    #[doc="Write the RFR register."]
    #[inline] pub fn set_rfr<I: Into<bits::R2>, F: FnOnce(Rfr) -> Rfr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.rfr_mut(index), f(Rfr(0)));
        }
        self
    }

    #[doc="Modify the RFR register."]
    #[inline] pub fn with_rfr<I: Into<bits::R2> + Copy, F: FnOnce(Rfr) -> Rfr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.rfr_mut(index), f(self.rfr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the IER register."]
    #[inline] pub fn ier_mut(&self) -> *mut Ier { 
        (self.0 + 0x14) as *mut Ier
    }

    #[doc="Get the *const pointer for the IER register."]
    #[inline] pub fn ier_ptr(&self) -> *const Ier { 
           self.ier_mut()
    }

    #[doc="Read the IER register."]
    #[inline] pub fn ier(&self) -> Ier { 
        unsafe {
            read_volatile(self.ier_ptr())
        }
    }

    #[doc="Write the IER register."]
    #[inline] pub fn set_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ier_mut(), f(Ier(0)));
        }
        self
    }

    #[doc="Modify the IER register."]
    #[inline] pub fn with_ier<F: FnOnce(Ier) -> Ier>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ier_mut(), f(self.ier()));
        }
        self
    }

    #[doc="Get the *mut pointer for the ESR register."]
    #[inline] pub fn esr_mut(&self) -> *mut Esr { 
        (self.0 + 0x18) as *mut Esr
    }

    #[doc="Get the *const pointer for the ESR register."]
    #[inline] pub fn esr_ptr(&self) -> *const Esr { 
           self.esr_mut()
    }

    #[doc="Read the ESR register."]
    #[inline] pub fn esr(&self) -> Esr { 
        unsafe {
            read_volatile(self.esr_ptr())
        }
    }

    #[doc="Write the ESR register."]
    #[inline] pub fn set_esr<F: FnOnce(Esr) -> Esr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.esr_mut(), f(Esr(0)));
        }
        self
    }

    #[doc="Modify the ESR register."]
    #[inline] pub fn with_esr<F: FnOnce(Esr) -> Esr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.esr_mut(), f(self.esr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the BTR register."]
    #[inline] pub fn btr_mut(&self) -> *mut Btr { 
        (self.0 + 0x1c) as *mut Btr
    }

    #[doc="Get the *const pointer for the BTR register."]
    #[inline] pub fn btr_ptr(&self) -> *const Btr { 
           self.btr_mut()
    }

    #[doc="Read the BTR register."]
    #[inline] pub fn btr(&self) -> Btr { 
        unsafe {
            read_volatile(self.btr_ptr())
        }
    }

    #[doc="Write the BTR register."]
    #[inline] pub fn set_btr<F: FnOnce(Btr) -> Btr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.btr_mut(), f(Btr(0)));
        }
        self
    }

    #[doc="Modify the BTR register."]
    #[inline] pub fn with_btr<F: FnOnce(Btr) -> Btr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.btr_mut(), f(self.btr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the TIR register."]
    #[inline] pub fn tir_mut<I: Into<bits::R3>>(&self, index: I) -> *mut Tir { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x180 + (index << 4)) as *mut Tir
    }

    #[doc="Get the *const pointer for the TIR register."]
    #[inline] pub fn tir_ptr<I: Into<bits::R3>>(&self, index: I) -> *const Tir { 
           self.tir_mut(index)
    }

    #[doc="Read the TIR register."]
    #[inline] pub fn tir<I: Into<bits::R3>>(&self, index: I) -> Tir { 
        unsafe {
            read_volatile(self.tir_ptr(index))
        }
    }

    #[doc="Write the TIR register."]
    #[inline] pub fn set_tir<I: Into<bits::R3>, F: FnOnce(Tir) -> Tir>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tir_mut(index), f(Tir(0)));
        }
        self
    }

    #[doc="Modify the TIR register."]
    #[inline] pub fn with_tir<I: Into<bits::R3> + Copy, F: FnOnce(Tir) -> Tir>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tir_mut(index), f(self.tir(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDTR register."]
    #[inline] pub fn tdtr_mut<I: Into<bits::R3>>(&self, index: I) -> *mut Tdtr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x184 + (index << 4)) as *mut Tdtr
    }

    #[doc="Get the *const pointer for the TDTR register."]
    #[inline] pub fn tdtr_ptr<I: Into<bits::R3>>(&self, index: I) -> *const Tdtr { 
           self.tdtr_mut(index)
    }

    #[doc="Read the TDTR register."]
    #[inline] pub fn tdtr<I: Into<bits::R3>>(&self, index: I) -> Tdtr { 
        unsafe {
            read_volatile(self.tdtr_ptr(index))
        }
    }

    #[doc="Write the TDTR register."]
    #[inline] pub fn set_tdtr<I: Into<bits::R3>, F: FnOnce(Tdtr) -> Tdtr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdtr_mut(index), f(Tdtr(0)));
        }
        self
    }

    #[doc="Modify the TDTR register."]
    #[inline] pub fn with_tdtr<I: Into<bits::R3> + Copy, F: FnOnce(Tdtr) -> Tdtr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdtr_mut(index), f(self.tdtr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDLR register."]
    #[inline] pub fn tdlr_mut<I: Into<bits::R3>>(&self, index: I) -> *mut Tdlr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x188 + (index << 4)) as *mut Tdlr
    }

    #[doc="Get the *const pointer for the TDLR register."]
    #[inline] pub fn tdlr_ptr<I: Into<bits::R3>>(&self, index: I) -> *const Tdlr { 
           self.tdlr_mut(index)
    }

    #[doc="Read the TDLR register."]
    #[inline] pub fn tdlr<I: Into<bits::R3>>(&self, index: I) -> Tdlr { 
        unsafe {
            read_volatile(self.tdlr_ptr(index))
        }
    }

    #[doc="Write the TDLR register."]
    #[inline] pub fn set_tdlr<I: Into<bits::R3>, F: FnOnce(Tdlr) -> Tdlr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdlr_mut(index), f(Tdlr(0)));
        }
        self
    }

    #[doc="Modify the TDLR register."]
    #[inline] pub fn with_tdlr<I: Into<bits::R3> + Copy, F: FnOnce(Tdlr) -> Tdlr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdlr_mut(index), f(self.tdlr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the TDHR register."]
    #[inline] pub fn tdhr_mut<I: Into<bits::R3>>(&self, index: I) -> *mut Tdhr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x18c + (index << 4)) as *mut Tdhr
    }

    #[doc="Get the *const pointer for the TDHR register."]
    #[inline] pub fn tdhr_ptr<I: Into<bits::R3>>(&self, index: I) -> *const Tdhr { 
           self.tdhr_mut(index)
    }

    #[doc="Read the TDHR register."]
    #[inline] pub fn tdhr<I: Into<bits::R3>>(&self, index: I) -> Tdhr { 
        unsafe {
            read_volatile(self.tdhr_ptr(index))
        }
    }

    #[doc="Write the TDHR register."]
    #[inline] pub fn set_tdhr<I: Into<bits::R3>, F: FnOnce(Tdhr) -> Tdhr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdhr_mut(index), f(Tdhr(0)));
        }
        self
    }

    #[doc="Modify the TDHR register."]
    #[inline] pub fn with_tdhr<I: Into<bits::R3> + Copy, F: FnOnce(Tdhr) -> Tdhr>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.tdhr_mut(index), f(self.tdhr(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the RIR register."]
    #[inline] pub fn rir_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Rir { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1b0 + (index << 4)) as *mut Rir
    }

    #[doc="Get the *const pointer for the RIR register."]
    #[inline] pub fn rir_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Rir { 
           self.rir_mut(index)
    }

    #[doc="Read the RIR register."]
    #[inline] pub fn rir<I: Into<bits::R2>>(&self, index: I) -> Rir { 
        unsafe {
            read_volatile(self.rir_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the RDTR register."]
    #[inline] pub fn rdtr_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Rdtr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1b4 + (index << 4)) as *mut Rdtr
    }

    #[doc="Get the *const pointer for the RDTR register."]
    #[inline] pub fn rdtr_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Rdtr { 
           self.rdtr_mut(index)
    }

    #[doc="Read the RDTR register."]
    #[inline] pub fn rdtr<I: Into<bits::R2>>(&self, index: I) -> Rdtr { 
        unsafe {
            read_volatile(self.rdtr_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the RDLR register."]
    #[inline] pub fn rdlr_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Rdlr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1b8 + (index << 4)) as *mut Rdlr
    }

    #[doc="Get the *const pointer for the RDLR register."]
    #[inline] pub fn rdlr_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Rdlr { 
           self.rdlr_mut(index)
    }

    #[doc="Read the RDLR register."]
    #[inline] pub fn rdlr<I: Into<bits::R2>>(&self, index: I) -> Rdlr { 
        unsafe {
            read_volatile(self.rdlr_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the RDHR register."]
    #[inline] pub fn rdhr_mut<I: Into<bits::R2>>(&self, index: I) -> *mut Rdhr { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x1bc + (index << 4)) as *mut Rdhr
    }

    #[doc="Get the *const pointer for the RDHR register."]
    #[inline] pub fn rdhr_ptr<I: Into<bits::R2>>(&self, index: I) -> *const Rdhr { 
           self.rdhr_mut(index)
    }

    #[doc="Read the RDHR register."]
    #[inline] pub fn rdhr<I: Into<bits::R2>>(&self, index: I) -> Rdhr { 
        unsafe {
            read_volatile(self.rdhr_ptr(index))
        }
    }

    #[doc="Get the *mut pointer for the FMR register."]
    #[inline] pub fn fmr_mut(&self) -> *mut Fmr { 
        (self.0 + 0x200) as *mut Fmr
    }

    #[doc="Get the *const pointer for the FMR register."]
    #[inline] pub fn fmr_ptr(&self) -> *const Fmr { 
           self.fmr_mut()
    }

    #[doc="Read the FMR register."]
    #[inline] pub fn fmr(&self) -> Fmr { 
        unsafe {
            read_volatile(self.fmr_ptr())
        }
    }

    #[doc="Write the FMR register."]
    #[inline] pub fn set_fmr<F: FnOnce(Fmr) -> Fmr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fmr_mut(), f(Fmr(0)));
        }
        self
    }

    #[doc="Modify the FMR register."]
    #[inline] pub fn with_fmr<F: FnOnce(Fmr) -> Fmr>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fmr_mut(), f(self.fmr()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FM1R register."]
    #[inline] pub fn fm1r_mut(&self) -> *mut Fm1r { 
        (self.0 + 0x204) as *mut Fm1r
    }

    #[doc="Get the *const pointer for the FM1R register."]
    #[inline] pub fn fm1r_ptr(&self) -> *const Fm1r { 
           self.fm1r_mut()
    }

    #[doc="Read the FM1R register."]
    #[inline] pub fn fm1r(&self) -> Fm1r { 
        unsafe {
            read_volatile(self.fm1r_ptr())
        }
    }

    #[doc="Write the FM1R register."]
    #[inline] pub fn set_fm1r<F: FnOnce(Fm1r) -> Fm1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fm1r_mut(), f(Fm1r(0)));
        }
        self
    }

    #[doc="Modify the FM1R register."]
    #[inline] pub fn with_fm1r<F: FnOnce(Fm1r) -> Fm1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fm1r_mut(), f(self.fm1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FS1R register."]
    #[inline] pub fn fs1r_mut(&self) -> *mut Fs1r { 
        (self.0 + 0x20c) as *mut Fs1r
    }

    #[doc="Get the *const pointer for the FS1R register."]
    #[inline] pub fn fs1r_ptr(&self) -> *const Fs1r { 
           self.fs1r_mut()
    }

    #[doc="Read the FS1R register."]
    #[inline] pub fn fs1r(&self) -> Fs1r { 
        unsafe {
            read_volatile(self.fs1r_ptr())
        }
    }

    #[doc="Write the FS1R register."]
    #[inline] pub fn set_fs1r<F: FnOnce(Fs1r) -> Fs1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fs1r_mut(), f(Fs1r(0)));
        }
        self
    }

    #[doc="Modify the FS1R register."]
    #[inline] pub fn with_fs1r<F: FnOnce(Fs1r) -> Fs1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fs1r_mut(), f(self.fs1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FFA1R register."]
    #[inline] pub fn ffa1r_mut(&self) -> *mut Ffa1r { 
        (self.0 + 0x214) as *mut Ffa1r
    }

    #[doc="Get the *const pointer for the FFA1R register."]
    #[inline] pub fn ffa1r_ptr(&self) -> *const Ffa1r { 
           self.ffa1r_mut()
    }

    #[doc="Read the FFA1R register."]
    #[inline] pub fn ffa1r(&self) -> Ffa1r { 
        unsafe {
            read_volatile(self.ffa1r_ptr())
        }
    }

    #[doc="Write the FFA1R register."]
    #[inline] pub fn set_ffa1r<F: FnOnce(Ffa1r) -> Ffa1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ffa1r_mut(), f(Ffa1r(0)));
        }
        self
    }

    #[doc="Modify the FFA1R register."]
    #[inline] pub fn with_ffa1r<F: FnOnce(Ffa1r) -> Ffa1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.ffa1r_mut(), f(self.ffa1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FA1R register."]
    #[inline] pub fn fa1r_mut(&self) -> *mut Fa1r { 
        (self.0 + 0x21c) as *mut Fa1r
    }

    #[doc="Get the *const pointer for the FA1R register."]
    #[inline] pub fn fa1r_ptr(&self) -> *const Fa1r { 
           self.fa1r_mut()
    }

    #[doc="Read the FA1R register."]
    #[inline] pub fn fa1r(&self) -> Fa1r { 
        unsafe {
            read_volatile(self.fa1r_ptr())
        }
    }

    #[doc="Write the FA1R register."]
    #[inline] pub fn set_fa1r<F: FnOnce(Fa1r) -> Fa1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fa1r_mut(), f(Fa1r(0)));
        }
        self
    }

    #[doc="Modify the FA1R register."]
    #[inline] pub fn with_fa1r<F: FnOnce(Fa1r) -> Fa1r>(&self, f: F) -> &Self {
        unsafe {
            write_volatile(self.fa1r_mut(), f(self.fa1r()));
        }
        self
    }

    #[doc="Get the *mut pointer for the FR0 register."]
    #[inline] pub fn fr0_mut<I: Into<bits::R28>>(&self, index: I) -> *mut Fr0 { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x240 + (index << 3)) as *mut Fr0
    }

    #[doc="Get the *const pointer for the FR0 register."]
    #[inline] pub fn fr0_ptr<I: Into<bits::R28>>(&self, index: I) -> *const Fr0 { 
           self.fr0_mut(index)
    }

    #[doc="Read the FR0 register."]
    #[inline] pub fn fr0<I: Into<bits::R28>>(&self, index: I) -> Fr0 { 
        unsafe {
            read_volatile(self.fr0_ptr(index))
        }
    }

    #[doc="Write the FR0 register."]
    #[inline] pub fn set_fr0<I: Into<bits::R28>, F: FnOnce(Fr0) -> Fr0>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fr0_mut(index), f(Fr0(0)));
        }
        self
    }

    #[doc="Modify the FR0 register."]
    #[inline] pub fn with_fr0<I: Into<bits::R28> + Copy, F: FnOnce(Fr0) -> Fr0>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fr0_mut(index), f(self.fr0(index)));
        }
        self
    }

    #[doc="Get the *mut pointer for the FR1 register."]
    #[inline] pub fn fr1_mut<I: Into<bits::R28>>(&self, index: I) -> *mut Fr1 { 
        let index: usize = index.into().value() as usize;
        (self.0 + 0x244 + (index << 3)) as *mut Fr1
    }

    #[doc="Get the *const pointer for the FR1 register."]
    #[inline] pub fn fr1_ptr<I: Into<bits::R28>>(&self, index: I) -> *const Fr1 { 
           self.fr1_mut(index)
    }

    #[doc="Read the FR1 register."]
    #[inline] pub fn fr1<I: Into<bits::R28>>(&self, index: I) -> Fr1 { 
        unsafe {
            read_volatile(self.fr1_ptr(index))
        }
    }

    #[doc="Write the FR1 register."]
    #[inline] pub fn set_fr1<I: Into<bits::R28>, F: FnOnce(Fr1) -> Fr1>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fr1_mut(index), f(Fr1(0)));
        }
        self
    }

    #[doc="Modify the FR1 register."]
    #[inline] pub fn with_fr1<I: Into<bits::R28> + Copy, F: FnOnce(Fr1) -> Fr1>(&self, index: I, f: F) -> &Self {
        unsafe {
            write_volatile(self.fr1_mut(index), f(self.fr1(index)));
        }
        self
    }

}

#[doc="master control register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc="DBF"]
    #[inline] pub fn dbf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if DBF != 0"]
    #[inline] pub fn test_dbf(&self) -> bool {
        self.dbf() != 0
    }

    #[doc="Sets the DBF field."]
    #[inline] pub fn set_dbf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="RESET"]
    #[inline] pub fn _reset(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if RESET != 0"]
    #[inline] pub fn test_reset(&self) -> bool {
        self._reset() != 0
    }

    #[doc="Sets the RESET field."]
    #[inline] pub fn set_reset<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="TTCM"]
    #[inline] pub fn ttcm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if TTCM != 0"]
    #[inline] pub fn test_ttcm(&self) -> bool {
        self.ttcm() != 0
    }

    #[doc="Sets the TTCM field."]
    #[inline] pub fn set_ttcm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="ABOM"]
    #[inline] pub fn abom(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if ABOM != 0"]
    #[inline] pub fn test_abom(&self) -> bool {
        self.abom() != 0
    }

    #[doc="Sets the ABOM field."]
    #[inline] pub fn set_abom<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="AWUM"]
    #[inline] pub fn awum(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if AWUM != 0"]
    #[inline] pub fn test_awum(&self) -> bool {
        self.awum() != 0
    }

    #[doc="Sets the AWUM field."]
    #[inline] pub fn set_awum<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="NART"]
    #[inline] pub fn nart(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if NART != 0"]
    #[inline] pub fn test_nart(&self) -> bool {
        self.nart() != 0
    }

    #[doc="Sets the NART field."]
    #[inline] pub fn set_nart<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="RFLM"]
    #[inline] pub fn rflm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if RFLM != 0"]
    #[inline] pub fn test_rflm(&self) -> bool {
        self.rflm() != 0
    }

    #[doc="Sets the RFLM field."]
    #[inline] pub fn set_rflm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="TXFP"]
    #[inline] pub fn txfp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if TXFP != 0"]
    #[inline] pub fn test_txfp(&self) -> bool {
        self.txfp() != 0
    }

    #[doc="Sets the TXFP field."]
    #[inline] pub fn set_txfp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SLEEP"]
    #[inline] pub fn sleep(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SLEEP != 0"]
    #[inline] pub fn test_sleep(&self) -> bool {
        self.sleep() != 0
    }

    #[doc="Sets the SLEEP field."]
    #[inline] pub fn set_sleep<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="INRQ"]
    #[inline] pub fn inrq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INRQ != 0"]
    #[inline] pub fn test_inrq(&self) -> bool {
        self.inrq() != 0
    }

    #[doc="Sets the INRQ field."]
    #[inline] pub fn set_inrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Mcr {
    #[inline]
    fn from(other: u32) -> Self {
         Mcr(other)
    }
}

impl ::core::fmt::Display for Mcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.dbf() != 0 { try!(write!(f, " dbf"))}
        if self._reset() != 0 { try!(write!(f, " _reset"))}
        if self.ttcm() != 0 { try!(write!(f, " ttcm"))}
        if self.abom() != 0 { try!(write!(f, " abom"))}
        if self.awum() != 0 { try!(write!(f, " awum"))}
        if self.nart() != 0 { try!(write!(f, " nart"))}
        if self.rflm() != 0 { try!(write!(f, " rflm"))}
        if self.txfp() != 0 { try!(write!(f, " txfp"))}
        if self.sleep() != 0 { try!(write!(f, " sleep"))}
        if self.inrq() != 0 { try!(write!(f, " inrq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="master status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc="RX"]
    #[inline] pub fn rx(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if RX != 0"]
    #[inline] pub fn test_rx(&self) -> bool {
        self.rx() != 0
    }

    #[doc="Sets the RX field."]
    #[inline] pub fn set_rx<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="SAMP"]
    #[inline] pub fn samp(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if SAMP != 0"]
    #[inline] pub fn test_samp(&self) -> bool {
        self.samp() != 0
    }

    #[doc="Sets the SAMP field."]
    #[inline] pub fn set_samp<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="RXM"]
    #[inline] pub fn rxm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if RXM != 0"]
    #[inline] pub fn test_rxm(&self) -> bool {
        self.rxm() != 0
    }

    #[doc="Sets the RXM field."]
    #[inline] pub fn set_rxm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="TXM"]
    #[inline] pub fn txm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TXM != 0"]
    #[inline] pub fn test_txm(&self) -> bool {
        self.txm() != 0
    }

    #[doc="Sets the TXM field."]
    #[inline] pub fn set_txm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="SLAKI"]
    #[inline] pub fn slaki(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if SLAKI != 0"]
    #[inline] pub fn test_slaki(&self) -> bool {
        self.slaki() != 0
    }

    #[doc="Sets the SLAKI field."]
    #[inline] pub fn set_slaki<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="WKUI"]
    #[inline] pub fn wkui(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if WKUI != 0"]
    #[inline] pub fn test_wkui(&self) -> bool {
        self.wkui() != 0
    }

    #[doc="Sets the WKUI field."]
    #[inline] pub fn set_wkui<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ERRI"]
    #[inline] pub fn erri(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ERRI != 0"]
    #[inline] pub fn test_erri(&self) -> bool {
        self.erri() != 0
    }

    #[doc="Sets the ERRI field."]
    #[inline] pub fn set_erri<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="SLAK"]
    #[inline] pub fn slak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if SLAK != 0"]
    #[inline] pub fn test_slak(&self) -> bool {
        self.slak() != 0
    }

    #[doc="Sets the SLAK field."]
    #[inline] pub fn set_slak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="INAK"]
    #[inline] pub fn inak(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if INAK != 0"]
    #[inline] pub fn test_inak(&self) -> bool {
        self.inak() != 0
    }

    #[doc="Sets the INAK field."]
    #[inline] pub fn set_inak<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Msr {
    #[inline]
    fn from(other: u32) -> Self {
         Msr(other)
    }
}

impl ::core::fmt::Display for Msr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Msr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rx() != 0 { try!(write!(f, " rx"))}
        if self.samp() != 0 { try!(write!(f, " samp"))}
        if self.rxm() != 0 { try!(write!(f, " rxm"))}
        if self.txm() != 0 { try!(write!(f, " txm"))}
        if self.slaki() != 0 { try!(write!(f, " slaki"))}
        if self.wkui() != 0 { try!(write!(f, " wkui"))}
        if self.erri() != 0 { try!(write!(f, " erri"))}
        if self.slak() != 0 { try!(write!(f, " slak"))}
        if self.inak() != 0 { try!(write!(f, " inak"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="transmit status register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tsr(pub u32);
impl Tsr {
    #[doc="Lowest priority flag for mailbox 2"]
    #[inline] pub fn low2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if LOW2 != 0"]
    #[inline] pub fn test_low2(&self) -> bool {
        self.low2() != 0
    }

    #[doc="Sets the LOW2 field."]
    #[inline] pub fn set_low2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="Lowest priority flag for mailbox 1"]
    #[inline] pub fn low1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if LOW1 != 0"]
    #[inline] pub fn test_low1(&self) -> bool {
        self.low1() != 0
    }

    #[doc="Sets the LOW1 field."]
    #[inline] pub fn set_low1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="Lowest priority flag for mailbox 0"]
    #[inline] pub fn low0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 29) & 0x1) as u8) } // [29]
    }

    #[doc="Returns true if LOW0 != 0"]
    #[inline] pub fn test_low0(&self) -> bool {
        self.low0() != 0
    }

    #[doc="Sets the LOW0 field."]
    #[inline] pub fn set_low0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 29);
        self.0 |= value << 29;
        self
    }

    #[doc="Lowest priority flag for mailbox 2"]
    #[inline] pub fn tme2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 28) & 0x1) as u8) } // [28]
    }

    #[doc="Returns true if TME2 != 0"]
    #[inline] pub fn test_tme2(&self) -> bool {
        self.tme2() != 0
    }

    #[doc="Sets the TME2 field."]
    #[inline] pub fn set_tme2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 28);
        self.0 |= value << 28;
        self
    }

    #[doc="Lowest priority flag for mailbox 1"]
    #[inline] pub fn tme1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 27) & 0x1) as u8) } // [27]
    }

    #[doc="Returns true if TME1 != 0"]
    #[inline] pub fn test_tme1(&self) -> bool {
        self.tme1() != 0
    }

    #[doc="Sets the TME1 field."]
    #[inline] pub fn set_tme1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 27);
        self.0 |= value << 27;
        self
    }

    #[doc="Lowest priority flag for mailbox 0"]
    #[inline] pub fn tme0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 26) & 0x1) as u8) } // [26]
    }

    #[doc="Returns true if TME0 != 0"]
    #[inline] pub fn test_tme0(&self) -> bool {
        self.tme0() != 0
    }

    #[doc="Sets the TME0 field."]
    #[inline] pub fn set_tme0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 26);
        self.0 |= value << 26;
        self
    }

    #[doc="CODE"]
    #[inline] pub fn code(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if CODE != 0"]
    #[inline] pub fn test_code(&self) -> bool {
        self.code() != 0
    }

    #[doc="Sets the CODE field."]
    #[inline] pub fn set_code<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="ABRQ2"]
    #[inline] pub fn abrq2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 23) & 0x1) as u8) } // [23]
    }

    #[doc="Returns true if ABRQ2 != 0"]
    #[inline] pub fn test_abrq2(&self) -> bool {
        self.abrq2() != 0
    }

    #[doc="Sets the ABRQ2 field."]
    #[inline] pub fn set_abrq2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 23);
        self.0 |= value << 23;
        self
    }

    #[doc="TERR2"]
    #[inline] pub fn terr2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 19) & 0x1) as u8) } // [19]
    }

    #[doc="Returns true if TERR2 != 0"]
    #[inline] pub fn test_terr2(&self) -> bool {
        self.terr2() != 0
    }

    #[doc="Sets the TERR2 field."]
    #[inline] pub fn set_terr2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 19);
        self.0 |= value << 19;
        self
    }

    #[doc="ALST2"]
    #[inline] pub fn alst2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 18) & 0x1) as u8) } // [18]
    }

    #[doc="Returns true if ALST2 != 0"]
    #[inline] pub fn test_alst2(&self) -> bool {
        self.alst2() != 0
    }

    #[doc="Sets the ALST2 field."]
    #[inline] pub fn set_alst2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 18);
        self.0 |= value << 18;
        self
    }

    #[doc="TXOK2"]
    #[inline] pub fn txok2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if TXOK2 != 0"]
    #[inline] pub fn test_txok2(&self) -> bool {
        self.txok2() != 0
    }

    #[doc="Sets the TXOK2 field."]
    #[inline] pub fn set_txok2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="RQCP2"]
    #[inline] pub fn rqcp2(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if RQCP2 != 0"]
    #[inline] pub fn test_rqcp2(&self) -> bool {
        self.rqcp2() != 0
    }

    #[doc="Sets the RQCP2 field."]
    #[inline] pub fn set_rqcp2<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ABRQ1"]
    #[inline] pub fn abrq1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ABRQ1 != 0"]
    #[inline] pub fn test_abrq1(&self) -> bool {
        self.abrq1() != 0
    }

    #[doc="Sets the ABRQ1 field."]
    #[inline] pub fn set_abrq1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="TERR1"]
    #[inline] pub fn terr1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if TERR1 != 0"]
    #[inline] pub fn test_terr1(&self) -> bool {
        self.terr1() != 0
    }

    #[doc="Sets the TERR1 field."]
    #[inline] pub fn set_terr1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="ALST1"]
    #[inline] pub fn alst1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if ALST1 != 0"]
    #[inline] pub fn test_alst1(&self) -> bool {
        self.alst1() != 0
    }

    #[doc="Sets the ALST1 field."]
    #[inline] pub fn set_alst1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="TXOK1"]
    #[inline] pub fn txok1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if TXOK1 != 0"]
    #[inline] pub fn test_txok1(&self) -> bool {
        self.txok1() != 0
    }

    #[doc="Sets the TXOK1 field."]
    #[inline] pub fn set_txok1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="RQCP1"]
    #[inline] pub fn rqcp1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if RQCP1 != 0"]
    #[inline] pub fn test_rqcp1(&self) -> bool {
        self.rqcp1() != 0
    }

    #[doc="Sets the RQCP1 field."]
    #[inline] pub fn set_rqcp1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="ABRQ0"]
    #[inline] pub fn abrq0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 7) & 0x1) as u8) } // [7]
    }

    #[doc="Returns true if ABRQ0 != 0"]
    #[inline] pub fn test_abrq0(&self) -> bool {
        self.abrq0() != 0
    }

    #[doc="Sets the ABRQ0 field."]
    #[inline] pub fn set_abrq0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 7);
        self.0 |= value << 7;
        self
    }

    #[doc="TERR0"]
    #[inline] pub fn terr0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if TERR0 != 0"]
    #[inline] pub fn test_terr0(&self) -> bool {
        self.terr0() != 0
    }

    #[doc="Sets the TERR0 field."]
    #[inline] pub fn set_terr0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="ALST0"]
    #[inline] pub fn alst0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if ALST0 != 0"]
    #[inline] pub fn test_alst0(&self) -> bool {
        self.alst0() != 0
    }

    #[doc="Sets the ALST0 field."]
    #[inline] pub fn set_alst0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="TXOK0"]
    #[inline] pub fn txok0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if TXOK0 != 0"]
    #[inline] pub fn test_txok0(&self) -> bool {
        self.txok0() != 0
    }

    #[doc="Sets the TXOK0 field."]
    #[inline] pub fn set_txok0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="RQCP0"]
    #[inline] pub fn rqcp0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if RQCP0 != 0"]
    #[inline] pub fn test_rqcp0(&self) -> bool {
        self.rqcp0() != 0
    }

    #[doc="Sets the RQCP0 field."]
    #[inline] pub fn set_rqcp0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tsr {
    #[inline]
    fn from(other: u32) -> Self {
         Tsr(other)
    }
}

impl ::core::fmt::Display for Tsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tsr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.low2() != 0 { try!(write!(f, " low2"))}
        if self.low1() != 0 { try!(write!(f, " low1"))}
        if self.low0() != 0 { try!(write!(f, " low0"))}
        if self.tme2() != 0 { try!(write!(f, " tme2"))}
        if self.tme1() != 0 { try!(write!(f, " tme1"))}
        if self.tme0() != 0 { try!(write!(f, " tme0"))}
        if self.code() != 0 { try!(write!(f, " code=0x{:x}", self.code()))}
        if self.abrq2() != 0 { try!(write!(f, " abrq2"))}
        if self.terr2() != 0 { try!(write!(f, " terr2"))}
        if self.alst2() != 0 { try!(write!(f, " alst2"))}
        if self.txok2() != 0 { try!(write!(f, " txok2"))}
        if self.rqcp2() != 0 { try!(write!(f, " rqcp2"))}
        if self.abrq1() != 0 { try!(write!(f, " abrq1"))}
        if self.terr1() != 0 { try!(write!(f, " terr1"))}
        if self.alst1() != 0 { try!(write!(f, " alst1"))}
        if self.txok1() != 0 { try!(write!(f, " txok1"))}
        if self.rqcp1() != 0 { try!(write!(f, " rqcp1"))}
        if self.abrq0() != 0 { try!(write!(f, " abrq0"))}
        if self.terr0() != 0 { try!(write!(f, " terr0"))}
        if self.alst0() != 0 { try!(write!(f, " alst0"))}
        if self.txok0() != 0 { try!(write!(f, " txok0"))}
        if self.rqcp0() != 0 { try!(write!(f, " rqcp0"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="receive FIFO register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rfr(pub u32);
impl Rfr {
    #[doc="RFOM"]
    #[inline] pub fn rfom(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if RFOM != 0"]
    #[inline] pub fn test_rfom(&self) -> bool {
        self.rfom() != 0
    }

    #[doc="Sets the RFOM field."]
    #[inline] pub fn set_rfom<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="FOVR"]
    #[inline] pub fn fovr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FOVR != 0"]
    #[inline] pub fn test_fovr(&self) -> bool {
        self.fovr() != 0
    }

    #[doc="Sets the FOVR field."]
    #[inline] pub fn set_fovr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FULL"]
    #[inline] pub fn full(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FULL != 0"]
    #[inline] pub fn test_full(&self) -> bool {
        self.full() != 0
    }

    #[doc="Sets the FULL field."]
    #[inline] pub fn set_full<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FMP"]
    #[inline] pub fn fmp(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3) as u8) } // [1:0]
    }

    #[doc="Returns true if FMP != 0"]
    #[inline] pub fn test_fmp(&self) -> bool {
        self.fmp() != 0
    }

    #[doc="Sets the FMP field."]
    #[inline] pub fn set_fmp<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rfr {
    #[inline]
    fn from(other: u32) -> Self {
         Rfr(other)
    }
}

impl ::core::fmt::Display for Rfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rfr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rfom() != 0 { try!(write!(f, " rfom"))}
        if self.fovr() != 0 { try!(write!(f, " fovr"))}
        if self.full() != 0 { try!(write!(f, " full"))}
        if self.fmp() != 0 { try!(write!(f, " fmp=0x{:x}", self.fmp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc="SLKIE"]
    #[inline] pub fn slkie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 17) & 0x1) as u8) } // [17]
    }

    #[doc="Returns true if SLKIE != 0"]
    #[inline] pub fn test_slkie(&self) -> bool {
        self.slkie() != 0
    }

    #[doc="Sets the SLKIE field."]
    #[inline] pub fn set_slkie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 17);
        self.0 |= value << 17;
        self
    }

    #[doc="WKUIE"]
    #[inline] pub fn wkuie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0x1) as u8) } // [16]
    }

    #[doc="Returns true if WKUIE != 0"]
    #[inline] pub fn test_wkuie(&self) -> bool {
        self.wkuie() != 0
    }

    #[doc="Sets the WKUIE field."]
    #[inline] pub fn set_wkuie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="ERRIE"]
    #[inline] pub fn errie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 15) & 0x1) as u8) } // [15]
    }

    #[doc="Returns true if ERRIE != 0"]
    #[inline] pub fn test_errie(&self) -> bool {
        self.errie() != 0
    }

    #[doc="Sets the ERRIE field."]
    #[inline] pub fn set_errie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 15);
        self.0 |= value << 15;
        self
    }

    #[doc="LECIE"]
    #[inline] pub fn lecie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 11) & 0x1) as u8) } // [11]
    }

    #[doc="Returns true if LECIE != 0"]
    #[inline] pub fn test_lecie(&self) -> bool {
        self.lecie() != 0
    }

    #[doc="Sets the LECIE field."]
    #[inline] pub fn set_lecie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 11);
        self.0 |= value << 11;
        self
    }

    #[doc="BOFIE"]
    #[inline] pub fn bofie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 10) & 0x1) as u8) } // [10]
    }

    #[doc="Returns true if BOFIE != 0"]
    #[inline] pub fn test_bofie(&self) -> bool {
        self.bofie() != 0
    }

    #[doc="Sets the BOFIE field."]
    #[inline] pub fn set_bofie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 10);
        self.0 |= value << 10;
        self
    }

    #[doc="EPVIE"]
    #[inline] pub fn epvie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 9) & 0x1) as u8) } // [9]
    }

    #[doc="Returns true if EPVIE != 0"]
    #[inline] pub fn test_epvie(&self) -> bool {
        self.epvie() != 0
    }

    #[doc="Sets the EPVIE field."]
    #[inline] pub fn set_epvie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 9);
        self.0 |= value << 9;
        self
    }

    #[doc="EWGIE"]
    #[inline] pub fn ewgie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if EWGIE != 0"]
    #[inline] pub fn test_ewgie(&self) -> bool {
        self.ewgie() != 0
    }

    #[doc="Sets the EWGIE field."]
    #[inline] pub fn set_ewgie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FOVIE1"]
    #[inline] pub fn fovie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 6) & 0x1) as u8) } // [6]
    }

    #[doc="Returns true if FOVIE1 != 0"]
    #[inline] pub fn test_fovie1(&self) -> bool {
        self.fovie1() != 0
    }

    #[doc="Sets the FOVIE1 field."]
    #[inline] pub fn set_fovie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 6);
        self.0 |= value << 6;
        self
    }

    #[doc="FFIE1"]
    #[inline] pub fn ffie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 5) & 0x1) as u8) } // [5]
    }

    #[doc="Returns true if FFIE1 != 0"]
    #[inline] pub fn test_ffie1(&self) -> bool {
        self.ffie1() != 0
    }

    #[doc="Sets the FFIE1 field."]
    #[inline] pub fn set_ffie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 5);
        self.0 |= value << 5;
        self
    }

    #[doc="FMPIE1"]
    #[inline] pub fn fmpie1(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x1) as u8) } // [4]
    }

    #[doc="Returns true if FMPIE1 != 0"]
    #[inline] pub fn test_fmpie1(&self) -> bool {
        self.fmpie1() != 0
    }

    #[doc="Sets the FMPIE1 field."]
    #[inline] pub fn set_fmpie1<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="FOVIE0"]
    #[inline] pub fn fovie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x1) as u8) } // [3]
    }

    #[doc="Returns true if FOVIE0 != 0"]
    #[inline] pub fn test_fovie0(&self) -> bool {
        self.fovie0() != 0
    }

    #[doc="Sets the FOVIE0 field."]
    #[inline] pub fn set_fovie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="FFIE0"]
    #[inline] pub fn ffie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if FFIE0 != 0"]
    #[inline] pub fn test_ffie0(&self) -> bool {
        self.ffie0() != 0
    }

    #[doc="Sets the FFIE0 field."]
    #[inline] pub fn set_ffie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="FMPIE0"]
    #[inline] pub fn fmpie0(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if FMPIE0 != 0"]
    #[inline] pub fn test_fmpie0(&self) -> bool {
        self.fmpie0() != 0
    }

    #[doc="Sets the FMPIE0 field."]
    #[inline] pub fn set_fmpie0<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TMEIE"]
    #[inline] pub fn tmeie(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TMEIE != 0"]
    #[inline] pub fn test_tmeie(&self) -> bool {
        self.tmeie() != 0
    }

    #[doc="Sets the TMEIE field."]
    #[inline] pub fn set_tmeie<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
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
        if self.slkie() != 0 { try!(write!(f, " slkie"))}
        if self.wkuie() != 0 { try!(write!(f, " wkuie"))}
        if self.errie() != 0 { try!(write!(f, " errie"))}
        if self.lecie() != 0 { try!(write!(f, " lecie"))}
        if self.bofie() != 0 { try!(write!(f, " bofie"))}
        if self.epvie() != 0 { try!(write!(f, " epvie"))}
        if self.ewgie() != 0 { try!(write!(f, " ewgie"))}
        if self.fovie1() != 0 { try!(write!(f, " fovie1"))}
        if self.ffie1() != 0 { try!(write!(f, " ffie1"))}
        if self.fmpie1() != 0 { try!(write!(f, " fmpie1"))}
        if self.fovie0() != 0 { try!(write!(f, " fovie0"))}
        if self.ffie0() != 0 { try!(write!(f, " ffie0"))}
        if self.fmpie0() != 0 { try!(write!(f, " fmpie0"))}
        if self.tmeie() != 0 { try!(write!(f, " tmeie"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="interrupt enable register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Esr(pub u32);
impl Esr {
    #[doc="REC"]
    #[inline] pub fn rec(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if REC != 0"]
    #[inline] pub fn test_rec(&self) -> bool {
        self.rec() != 0
    }

    #[doc="Sets the REC field."]
    #[inline] pub fn set_rec<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="TEC"]
    #[inline] pub fn tec(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if TEC != 0"]
    #[inline] pub fn test_tec(&self) -> bool {
        self.tec() != 0
    }

    #[doc="Sets the TEC field."]
    #[inline] pub fn set_tec<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="LEC"]
    #[inline] pub fn lec(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 4) & 0x7) as u8) } // [6:4]
    }

    #[doc="Returns true if LEC != 0"]
    #[inline] pub fn test_lec(&self) -> bool {
        self.lec() != 0
    }

    #[doc="Sets the LEC field."]
    #[inline] pub fn set_lec<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 4);
        self.0 |= value << 4;
        self
    }

    #[doc="BOFF"]
    #[inline] pub fn boff(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if BOFF != 0"]
    #[inline] pub fn test_boff(&self) -> bool {
        self.boff() != 0
    }

    #[doc="Sets the BOFF field."]
    #[inline] pub fn set_boff<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="EPVF"]
    #[inline] pub fn epvf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if EPVF != 0"]
    #[inline] pub fn test_epvf(&self) -> bool {
        self.epvf() != 0
    }

    #[doc="Sets the EPVF field."]
    #[inline] pub fn set_epvf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="EWGF"]
    #[inline] pub fn ewgf(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if EWGF != 0"]
    #[inline] pub fn test_ewgf(&self) -> bool {
        self.ewgf() != 0
    }

    #[doc="Sets the EWGF field."]
    #[inline] pub fn set_ewgf<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Esr {
    #[inline]
    fn from(other: u32) -> Self {
         Esr(other)
    }
}

impl ::core::fmt::Display for Esr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Esr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.rec() != 0 { try!(write!(f, " rec=0x{:x}", self.rec()))}
        if self.tec() != 0 { try!(write!(f, " tec=0x{:x}", self.tec()))}
        if self.lec() != 0 { try!(write!(f, " lec=0x{:x}", self.lec()))}
        if self.boff() != 0 { try!(write!(f, " boff"))}
        if self.epvf() != 0 { try!(write!(f, " epvf"))}
        if self.ewgf() != 0 { try!(write!(f, " ewgf"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="bit timing register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Btr(pub u32);
impl Btr {
    #[doc="SILM"]
    #[inline] pub fn silm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 31) & 0x1) as u8) } // [31]
    }

    #[doc="Returns true if SILM != 0"]
    #[inline] pub fn test_silm(&self) -> bool {
        self.silm() != 0
    }

    #[doc="Sets the SILM field."]
    #[inline] pub fn set_silm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 31);
        self.0 |= value << 31;
        self
    }

    #[doc="LBKM"]
    #[inline] pub fn lbkm(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 30) & 0x1) as u8) } // [30]
    }

    #[doc="Returns true if LBKM != 0"]
    #[inline] pub fn test_lbkm(&self) -> bool {
        self.lbkm() != 0
    }

    #[doc="Sets the LBKM field."]
    #[inline] pub fn set_lbkm<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 30);
        self.0 |= value << 30;
        self
    }

    #[doc="SJW"]
    #[inline] pub fn sjw(&self) -> bits::U2 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0x3) as u8) } // [25:24]
    }

    #[doc="Returns true if SJW != 0"]
    #[inline] pub fn test_sjw(&self) -> bool {
        self.sjw() != 0
    }

    #[doc="Sets the SJW field."]
    #[inline] pub fn set_sjw<V: Into<bits::U2>>(mut self, value: V) -> Self {
        let value: bits::U2 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3 << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="TS2"]
    #[inline] pub fn ts2(&self) -> bits::U3 {
        unsafe { ::core::mem::transmute(((self.0 >> 20) & 0x7) as u8) } // [22:20]
    }

    #[doc="Returns true if TS2 != 0"]
    #[inline] pub fn test_ts2(&self) -> bool {
        self.ts2() != 0
    }

    #[doc="Sets the TS2 field."]
    #[inline] pub fn set_ts2<V: Into<bits::U3>>(mut self, value: V) -> Self {
        let value: bits::U3 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7 << 20);
        self.0 |= value << 20;
        self
    }

    #[doc="TS1"]
    #[inline] pub fn ts1(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xf) as u8) } // [19:16]
    }

    #[doc="Returns true if TS1 != 0"]
    #[inline] pub fn test_ts1(&self) -> bool {
        self.ts1() != 0
    }

    #[doc="Sets the TS1 field."]
    #[inline] pub fn set_ts1<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="BRP"]
    #[inline] pub fn brp(&self) -> bits::U10 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x3ff) as u16) } // [9:0]
    }

    #[doc="Returns true if BRP != 0"]
    #[inline] pub fn test_brp(&self) -> bool {
        self.brp() != 0
    }

    #[doc="Sets the BRP field."]
    #[inline] pub fn set_brp<V: Into<bits::U10>>(mut self, value: V) -> Self {
        let value: bits::U10 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Btr {
    #[inline]
    fn from(other: u32) -> Self {
         Btr(other)
    }
}

impl ::core::fmt::Display for Btr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Btr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.silm() != 0 { try!(write!(f, " silm"))}
        if self.lbkm() != 0 { try!(write!(f, " lbkm"))}
        if self.sjw() != 0 { try!(write!(f, " sjw=0x{:x}", self.sjw()))}
        if self.ts2() != 0 { try!(write!(f, " ts2=0x{:x}", self.ts2()))}
        if self.ts1() != 0 { try!(write!(f, " ts1=0x{:x}", self.ts1()))}
        if self.brp() != 0 { try!(write!(f, " brp=0x{:x}", self.brp()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="TX mailbox identifier register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tir(pub u32);
impl Tir {
    #[doc="STID"]
    #[inline] pub fn stid(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7ff) as u16) } // [31:21]
    }

    #[doc="Returns true if STID != 0"]
    #[inline] pub fn test_stid(&self) -> bool {
        self.stid() != 0
    }

    #[doc="Sets the STID field."]
    #[inline] pub fn set_stid<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EXID"]
    #[inline] pub fn exid(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3ffff) as u32) } // [20:3]
    }

    #[doc="Returns true if EXID != 0"]
    #[inline] pub fn test_exid(&self) -> bool {
        self.exid() != 0
    }

    #[doc="Sets the EXID field."]
    #[inline] pub fn set_exid<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IDE"]
    #[inline] pub fn ide(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IDE != 0"]
    #[inline] pub fn test_ide(&self) -> bool {
        self.ide() != 0
    }

    #[doc="Sets the IDE field."]
    #[inline] pub fn set_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RTR"]
    #[inline] pub fn rtr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RTR != 0"]
    #[inline] pub fn test_rtr(&self) -> bool {
        self.rtr() != 0
    }

    #[doc="Sets the RTR field."]
    #[inline] pub fn set_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

    #[doc="TXRQ"]
    #[inline] pub fn txrq(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if TXRQ != 0"]
    #[inline] pub fn test_txrq(&self) -> bool {
        self.txrq() != 0
    }

    #[doc="Sets the TXRQ field."]
    #[inline] pub fn set_txrq<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tir {
    #[inline]
    fn from(other: u32) -> Self {
         Tir(other)
    }
}

impl ::core::fmt::Display for Tir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stid() != 0 { try!(write!(f, " stid=0x{:x}", self.stid()))}
        if self.exid() != 0 { try!(write!(f, " exid=0x{:x}", self.exid()))}
        if self.ide() != 0 { try!(write!(f, " ide"))}
        if self.rtr() != 0 { try!(write!(f, " rtr"))}
        if self.txrq() != 0 { try!(write!(f, " txrq"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data length control and time stamp register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdtr(pub u32);
impl Tdtr {
    #[doc="TIME"]
    #[inline] pub fn time(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if TIME != 0"]
    #[inline] pub fn test_time(&self) -> bool {
        self.time() != 0
    }

    #[doc="Sets the TIME field."]
    #[inline] pub fn set_time<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="TGT"]
    #[inline] pub fn tgt(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x1) as u8) } // [8]
    }

    #[doc="Returns true if TGT != 0"]
    #[inline] pub fn test_tgt(&self) -> bool {
        self.tgt() != 0
    }

    #[doc="Sets the TGT field."]
    #[inline] pub fn set_tgt<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DLC"]
    #[inline] pub fn dlc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DLC != 0"]
    #[inline] pub fn test_dlc(&self) -> bool {
        self.dlc() != 0
    }

    #[doc="Sets the DLC field."]
    #[inline] pub fn set_dlc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdtr {
    #[inline]
    fn from(other: u32) -> Self {
         Tdtr(other)
    }
}

impl ::core::fmt::Display for Tdtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.time() != 0 { try!(write!(f, " time=0x{:x}", self.time()))}
        if self.tgt() != 0 { try!(write!(f, " tgt"))}
        if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data low register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdlr(pub u32);
impl Tdlr {
    #[doc="DATA3"]
    #[inline] pub fn data3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA3 != 0"]
    #[inline] pub fn test_data3(&self) -> bool {
        self.data3() != 0
    }

    #[doc="Sets the DATA3 field."]
    #[inline] pub fn set_data3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA2"]
    #[inline] pub fn data2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA2 != 0"]
    #[inline] pub fn test_data2(&self) -> bool {
        self.data2() != 0
    }

    #[doc="Sets the DATA2 field."]
    #[inline] pub fn set_data2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA1"]
    #[inline] pub fn data1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA1 != 0"]
    #[inline] pub fn test_data1(&self) -> bool {
        self.data1() != 0
    }

    #[doc="Sets the DATA1 field."]
    #[inline] pub fn set_data1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA0"]
    #[inline] pub fn data0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA0 != 0"]
    #[inline] pub fn test_data0(&self) -> bool {
        self.data0() != 0
    }

    #[doc="Sets the DATA0 field."]
    #[inline] pub fn set_data0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdlr {
    #[inline]
    fn from(other: u32) -> Self {
         Tdlr(other)
    }
}

impl ::core::fmt::Display for Tdlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data3() != 0 { try!(write!(f, " data3=0x{:x}", self.data3()))}
        if self.data2() != 0 { try!(write!(f, " data2=0x{:x}", self.data2()))}
        if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
        if self.data0() != 0 { try!(write!(f, " data0=0x{:x}", self.data0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Tdhr(pub u32);
impl Tdhr {
    #[doc="DATA7"]
    #[inline] pub fn data7(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA7 != 0"]
    #[inline] pub fn test_data7(&self) -> bool {
        self.data7() != 0
    }

    #[doc="Sets the DATA7 field."]
    #[inline] pub fn set_data7<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA6"]
    #[inline] pub fn data6(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA6 != 0"]
    #[inline] pub fn test_data6(&self) -> bool {
        self.data6() != 0
    }

    #[doc="Sets the DATA6 field."]
    #[inline] pub fn set_data6<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA5"]
    #[inline] pub fn data5(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA5 != 0"]
    #[inline] pub fn test_data5(&self) -> bool {
        self.data5() != 0
    }

    #[doc="Sets the DATA5 field."]
    #[inline] pub fn set_data5<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA4"]
    #[inline] pub fn data4(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA4 != 0"]
    #[inline] pub fn test_data4(&self) -> bool {
        self.data4() != 0
    }

    #[doc="Sets the DATA4 field."]
    #[inline] pub fn set_data4<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Tdhr {
    #[inline]
    fn from(other: u32) -> Self {
         Tdhr(other)
    }
}

impl ::core::fmt::Display for Tdhr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Tdhr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data7() != 0 { try!(write!(f, " data7=0x{:x}", self.data7()))}
        if self.data6() != 0 { try!(write!(f, " data6=0x{:x}", self.data6()))}
        if self.data5() != 0 { try!(write!(f, " data5=0x{:x}", self.data5()))}
        if self.data4() != 0 { try!(write!(f, " data4=0x{:x}", self.data4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="receive FIFO mailbox identifier register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rir(pub u32);
impl Rir {
    #[doc="STID"]
    #[inline] pub fn stid(&self) -> bits::U11 {
        unsafe { ::core::mem::transmute(((self.0 >> 21) & 0x7ff) as u16) } // [31:21]
    }

    #[doc="Returns true if STID != 0"]
    #[inline] pub fn test_stid(&self) -> bool {
        self.stid() != 0
    }

    #[doc="Sets the STID field."]
    #[inline] pub fn set_stid<V: Into<bits::U11>>(mut self, value: V) -> Self {
        let value: bits::U11 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x7ff << 21);
        self.0 |= value << 21;
        self
    }

    #[doc="EXID"]
    #[inline] pub fn exid(&self) -> bits::U18 {
        unsafe { ::core::mem::transmute(((self.0 >> 3) & 0x3ffff) as u32) } // [20:3]
    }

    #[doc="Returns true if EXID != 0"]
    #[inline] pub fn test_exid(&self) -> bool {
        self.exid() != 0
    }

    #[doc="Sets the EXID field."]
    #[inline] pub fn set_exid<V: Into<bits::U18>>(mut self, value: V) -> Self {
        let value: bits::U18 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3ffff << 3);
        self.0 |= value << 3;
        self
    }

    #[doc="IDE"]
    #[inline] pub fn ide(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 2) & 0x1) as u8) } // [2]
    }

    #[doc="Returns true if IDE != 0"]
    #[inline] pub fn test_ide(&self) -> bool {
        self.ide() != 0
    }

    #[doc="Sets the IDE field."]
    #[inline] pub fn set_ide<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 2);
        self.0 |= value << 2;
        self
    }

    #[doc="RTR"]
    #[inline] pub fn rtr(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 1) & 0x1) as u8) } // [1]
    }

    #[doc="Returns true if RTR != 0"]
    #[inline] pub fn test_rtr(&self) -> bool {
        self.rtr() != 0
    }

    #[doc="Sets the RTR field."]
    #[inline] pub fn set_rtr<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 1);
        self.0 |= value << 1;
        self
    }

}

impl From<u32> for Rir {
    #[inline]
    fn from(other: u32) -> Self {
         Rir(other)
    }
}

impl ::core::fmt::Display for Rir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rir {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.stid() != 0 { try!(write!(f, " stid=0x{:x}", self.stid()))}
        if self.exid() != 0 { try!(write!(f, " exid=0x{:x}", self.exid()))}
        if self.ide() != 0 { try!(write!(f, " ide"))}
        if self.rtr() != 0 { try!(write!(f, " rtr"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdtr(pub u32);
impl Rdtr {
    #[doc="TIME"]
    #[inline] pub fn time(&self) -> bits::U16 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xffff) as u16) } // [31:16]
    }

    #[doc="Returns true if TIME != 0"]
    #[inline] pub fn test_time(&self) -> bool {
        self.time() != 0
    }

    #[doc="Sets the TIME field."]
    #[inline] pub fn set_time<V: Into<bits::U16>>(mut self, value: V) -> Self {
        let value: bits::U16 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="FMI"]
    #[inline] pub fn fmi(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if FMI != 0"]
    #[inline] pub fn test_fmi(&self) -> bool {
        self.fmi() != 0
    }

    #[doc="Sets the FMI field."]
    #[inline] pub fn set_fmi<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DLC"]
    #[inline] pub fn dlc(&self) -> bits::U4 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xf) as u8) } // [3:0]
    }

    #[doc="Returns true if DLC != 0"]
    #[inline] pub fn test_dlc(&self) -> bool {
        self.dlc() != 0
    }

    #[doc="Sets the DLC field."]
    #[inline] pub fn set_dlc<V: Into<bits::U4>>(mut self, value: V) -> Self {
        let value: bits::U4 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xf << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdtr {
    #[inline]
    fn from(other: u32) -> Self {
         Rdtr(other)
    }
}

impl ::core::fmt::Display for Rdtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdtr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.time() != 0 { try!(write!(f, " time=0x{:x}", self.time()))}
        if self.fmi() != 0 { try!(write!(f, " fmi=0x{:x}", self.fmi()))}
        if self.dlc() != 0 { try!(write!(f, " dlc=0x{:x}", self.dlc()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdlr(pub u32);
impl Rdlr {
    #[doc="DATA3"]
    #[inline] pub fn data3(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA3 != 0"]
    #[inline] pub fn test_data3(&self) -> bool {
        self.data3() != 0
    }

    #[doc="Sets the DATA3 field."]
    #[inline] pub fn set_data3<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA2"]
    #[inline] pub fn data2(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA2 != 0"]
    #[inline] pub fn test_data2(&self) -> bool {
        self.data2() != 0
    }

    #[doc="Sets the DATA2 field."]
    #[inline] pub fn set_data2<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA1"]
    #[inline] pub fn data1(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA1 != 0"]
    #[inline] pub fn test_data1(&self) -> bool {
        self.data1() != 0
    }

    #[doc="Sets the DATA1 field."]
    #[inline] pub fn set_data1<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA0"]
    #[inline] pub fn data0(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA0 != 0"]
    #[inline] pub fn test_data0(&self) -> bool {
        self.data0() != 0
    }

    #[doc="Sets the DATA0 field."]
    #[inline] pub fn set_data0<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdlr {
    #[inline]
    fn from(other: u32) -> Self {
         Rdlr(other)
    }
}

impl ::core::fmt::Display for Rdlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdlr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data3() != 0 { try!(write!(f, " data3=0x{:x}", self.data3()))}
        if self.data2() != 0 { try!(write!(f, " data2=0x{:x}", self.data2()))}
        if self.data1() != 0 { try!(write!(f, " data1=0x{:x}", self.data1()))}
        if self.data0() != 0 { try!(write!(f, " data0=0x{:x}", self.data0()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="receive FIFO mailbox data high register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Rdhr(pub u32);
impl Rdhr {
    #[doc="DATA7"]
    #[inline] pub fn data7(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 24) & 0xff) as u8) } // [31:24]
    }

    #[doc="Returns true if DATA7 != 0"]
    #[inline] pub fn test_data7(&self) -> bool {
        self.data7() != 0
    }

    #[doc="Sets the DATA7 field."]
    #[inline] pub fn set_data7<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 24);
        self.0 |= value << 24;
        self
    }

    #[doc="DATA6"]
    #[inline] pub fn data6(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 16) & 0xff) as u8) } // [23:16]
    }

    #[doc="Returns true if DATA6 != 0"]
    #[inline] pub fn test_data6(&self) -> bool {
        self.data6() != 0
    }

    #[doc="Sets the DATA6 field."]
    #[inline] pub fn set_data6<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 16);
        self.0 |= value << 16;
        self
    }

    #[doc="DATA5"]
    #[inline] pub fn data5(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0xff) as u8) } // [15:8]
    }

    #[doc="Returns true if DATA5 != 0"]
    #[inline] pub fn test_data5(&self) -> bool {
        self.data5() != 0
    }

    #[doc="Sets the DATA5 field."]
    #[inline] pub fn set_data5<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="DATA4"]
    #[inline] pub fn data4(&self) -> bits::U8 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xff) as u8) } // [7:0]
    }

    #[doc="Returns true if DATA4 != 0"]
    #[inline] pub fn test_data4(&self) -> bool {
        self.data4() != 0
    }

    #[doc="Sets the DATA4 field."]
    #[inline] pub fn set_data4<V: Into<bits::U8>>(mut self, value: V) -> Self {
        let value: bits::U8 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Rdhr {
    #[inline]
    fn from(other: u32) -> Self {
         Rdhr(other)
    }
}

impl ::core::fmt::Display for Rdhr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Rdhr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.data7() != 0 { try!(write!(f, " data7=0x{:x}", self.data7()))}
        if self.data6() != 0 { try!(write!(f, " data6=0x{:x}", self.data6()))}
        if self.data5() != 0 { try!(write!(f, " data5=0x{:x}", self.data5()))}
        if self.data4() != 0 { try!(write!(f, " data4=0x{:x}", self.data4()))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter master register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fmr(pub u32);
impl Fmr {
    #[doc="CAN2SB"]
    #[inline] pub fn can2sb(&self) -> bits::U6 {
        unsafe { ::core::mem::transmute(((self.0 >> 8) & 0x3f) as u8) } // [13:8]
    }

    #[doc="Returns true if CAN2SB != 0"]
    #[inline] pub fn test_can2sb(&self) -> bool {
        self.can2sb() != 0
    }

    #[doc="Sets the CAN2SB field."]
    #[inline] pub fn set_can2sb<V: Into<bits::U6>>(mut self, value: V) -> Self {
        let value: bits::U6 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x3f << 8);
        self.0 |= value << 8;
        self
    }

    #[doc="FINIT"]
    #[inline] pub fn finit(&self) -> bits::U1 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FINIT != 0"]
    #[inline] pub fn test_finit(&self) -> bool {
        self.finit() != 0
    }

    #[doc="Sets the FINIT field."]
    #[inline] pub fn set_finit<V: Into<bits::U1>>(mut self, value: V) -> Self {
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0x1 << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fmr {
    #[inline]
    fn from(other: u32) -> Self {
         Fmr(other)
    }
}

impl ::core::fmt::Display for Fmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fmr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.can2sb() != 0 { try!(write!(f, " can2sb=0x{:x}", self.can2sb()))}
        if self.finit() != 0 { try!(write!(f, " finit"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter mode register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fm1r(pub u32);
impl Fm1r {
    #[doc="Filter mode"]
    #[inline] pub fn fbm<I: Into<bits::R27>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FBM != 0"]
    #[inline] pub fn test_fbm<I: Into<bits::R27>>(&self, index: I) -> bool{
        self.fbm(index) != 0
    }

    #[doc="Sets the FBM field."]
    #[inline] pub fn set_fbm<I: Into<bits::R27>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Fm1r {
    #[inline]
    fn from(other: u32) -> Self {
         Fm1r(other)
    }
}

impl ::core::fmt::Display for Fm1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fm1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fbm(0) != 0 { try!(write!(f, " fbm[0]"))}
        if self.fbm(1) != 0 { try!(write!(f, " fbm[1]"))}
        if self.fbm(2) != 0 { try!(write!(f, " fbm[2]"))}
        if self.fbm(3) != 0 { try!(write!(f, " fbm[3]"))}
        if self.fbm(4) != 0 { try!(write!(f, " fbm[4]"))}
        if self.fbm(5) != 0 { try!(write!(f, " fbm[5]"))}
        if self.fbm(6) != 0 { try!(write!(f, " fbm[6]"))}
        if self.fbm(7) != 0 { try!(write!(f, " fbm[7]"))}
        if self.fbm(8) != 0 { try!(write!(f, " fbm[8]"))}
        if self.fbm(9) != 0 { try!(write!(f, " fbm[9]"))}
        if self.fbm(10) != 0 { try!(write!(f, " fbm[10]"))}
        if self.fbm(11) != 0 { try!(write!(f, " fbm[11]"))}
        if self.fbm(12) != 0 { try!(write!(f, " fbm[12]"))}
        if self.fbm(13) != 0 { try!(write!(f, " fbm[13]"))}
        if self.fbm(14) != 0 { try!(write!(f, " fbm[14]"))}
        if self.fbm(15) != 0 { try!(write!(f, " fbm[15]"))}
        if self.fbm(16) != 0 { try!(write!(f, " fbm[16]"))}
        if self.fbm(17) != 0 { try!(write!(f, " fbm[17]"))}
        if self.fbm(18) != 0 { try!(write!(f, " fbm[18]"))}
        if self.fbm(19) != 0 { try!(write!(f, " fbm[19]"))}
        if self.fbm(20) != 0 { try!(write!(f, " fbm[20]"))}
        if self.fbm(21) != 0 { try!(write!(f, " fbm[21]"))}
        if self.fbm(22) != 0 { try!(write!(f, " fbm[22]"))}
        if self.fbm(23) != 0 { try!(write!(f, " fbm[23]"))}
        if self.fbm(24) != 0 { try!(write!(f, " fbm[24]"))}
        if self.fbm(25) != 0 { try!(write!(f, " fbm[25]"))}
        if self.fbm(26) != 0 { try!(write!(f, " fbm[26]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter scale register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fs1r(pub u32);
impl Fs1r {
    #[doc="Filter scale configuration"]
    #[inline] pub fn fsc<I: Into<bits::R27>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FSC != 0"]
    #[inline] pub fn test_fsc<I: Into<bits::R27>>(&self, index: I) -> bool{
        self.fsc(index) != 0
    }

    #[doc="Sets the FSC field."]
    #[inline] pub fn set_fsc<I: Into<bits::R27>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Fs1r {
    #[inline]
    fn from(other: u32) -> Self {
         Fs1r(other)
    }
}

impl ::core::fmt::Display for Fs1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fs1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fsc(0) != 0 { try!(write!(f, " fsc[0]"))}
        if self.fsc(1) != 0 { try!(write!(f, " fsc[1]"))}
        if self.fsc(2) != 0 { try!(write!(f, " fsc[2]"))}
        if self.fsc(3) != 0 { try!(write!(f, " fsc[3]"))}
        if self.fsc(4) != 0 { try!(write!(f, " fsc[4]"))}
        if self.fsc(5) != 0 { try!(write!(f, " fsc[5]"))}
        if self.fsc(6) != 0 { try!(write!(f, " fsc[6]"))}
        if self.fsc(7) != 0 { try!(write!(f, " fsc[7]"))}
        if self.fsc(8) != 0 { try!(write!(f, " fsc[8]"))}
        if self.fsc(9) != 0 { try!(write!(f, " fsc[9]"))}
        if self.fsc(10) != 0 { try!(write!(f, " fsc[10]"))}
        if self.fsc(11) != 0 { try!(write!(f, " fsc[11]"))}
        if self.fsc(12) != 0 { try!(write!(f, " fsc[12]"))}
        if self.fsc(13) != 0 { try!(write!(f, " fsc[13]"))}
        if self.fsc(14) != 0 { try!(write!(f, " fsc[14]"))}
        if self.fsc(15) != 0 { try!(write!(f, " fsc[15]"))}
        if self.fsc(16) != 0 { try!(write!(f, " fsc[16]"))}
        if self.fsc(17) != 0 { try!(write!(f, " fsc[17]"))}
        if self.fsc(18) != 0 { try!(write!(f, " fsc[18]"))}
        if self.fsc(19) != 0 { try!(write!(f, " fsc[19]"))}
        if self.fsc(20) != 0 { try!(write!(f, " fsc[20]"))}
        if self.fsc(21) != 0 { try!(write!(f, " fsc[21]"))}
        if self.fsc(22) != 0 { try!(write!(f, " fsc[22]"))}
        if self.fsc(23) != 0 { try!(write!(f, " fsc[23]"))}
        if self.fsc(24) != 0 { try!(write!(f, " fsc[24]"))}
        if self.fsc(25) != 0 { try!(write!(f, " fsc[25]"))}
        if self.fsc(26) != 0 { try!(write!(f, " fsc[26]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter FIFO assignment register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Ffa1r(pub u32);
impl Ffa1r {
    #[doc="Filter FIFO assignment for filter 0"]
    #[inline] pub fn ffa<I: Into<bits::R27>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FFA != 0"]
    #[inline] pub fn test_ffa<I: Into<bits::R27>>(&self, index: I) -> bool{
        self.ffa(index) != 0
    }

    #[doc="Sets the FFA field."]
    #[inline] pub fn set_ffa<I: Into<bits::R27>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Ffa1r {
    #[inline]
    fn from(other: u32) -> Self {
         Ffa1r(other)
    }
}

impl ::core::fmt::Display for Ffa1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Ffa1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.ffa(0) != 0 { try!(write!(f, " ffa[0]"))}
        if self.ffa(1) != 0 { try!(write!(f, " ffa[1]"))}
        if self.ffa(2) != 0 { try!(write!(f, " ffa[2]"))}
        if self.ffa(3) != 0 { try!(write!(f, " ffa[3]"))}
        if self.ffa(4) != 0 { try!(write!(f, " ffa[4]"))}
        if self.ffa(5) != 0 { try!(write!(f, " ffa[5]"))}
        if self.ffa(6) != 0 { try!(write!(f, " ffa[6]"))}
        if self.ffa(7) != 0 { try!(write!(f, " ffa[7]"))}
        if self.ffa(8) != 0 { try!(write!(f, " ffa[8]"))}
        if self.ffa(9) != 0 { try!(write!(f, " ffa[9]"))}
        if self.ffa(10) != 0 { try!(write!(f, " ffa[10]"))}
        if self.ffa(11) != 0 { try!(write!(f, " ffa[11]"))}
        if self.ffa(12) != 0 { try!(write!(f, " ffa[12]"))}
        if self.ffa(13) != 0 { try!(write!(f, " ffa[13]"))}
        if self.ffa(14) != 0 { try!(write!(f, " ffa[14]"))}
        if self.ffa(15) != 0 { try!(write!(f, " ffa[15]"))}
        if self.ffa(16) != 0 { try!(write!(f, " ffa[16]"))}
        if self.ffa(17) != 0 { try!(write!(f, " ffa[17]"))}
        if self.ffa(18) != 0 { try!(write!(f, " ffa[18]"))}
        if self.ffa(19) != 0 { try!(write!(f, " ffa[19]"))}
        if self.ffa(20) != 0 { try!(write!(f, " ffa[20]"))}
        if self.ffa(21) != 0 { try!(write!(f, " ffa[21]"))}
        if self.ffa(22) != 0 { try!(write!(f, " ffa[22]"))}
        if self.ffa(23) != 0 { try!(write!(f, " ffa[23]"))}
        if self.ffa(24) != 0 { try!(write!(f, " ffa[24]"))}
        if self.ffa(25) != 0 { try!(write!(f, " ffa[25]"))}
        if self.ffa(26) != 0 { try!(write!(f, " ffa[26]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="filter activation register"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fa1r(pub u32);
impl Fa1r {
    #[doc="Filter active"]
    #[inline] pub fn fact<I: Into<bits::R27>>(&self, index: I) -> bits::U1 {
        let index: usize = index.into().value() as usize;
        let shift: usize = 0 + index;
        unsafe { ::core::mem::transmute(((self.0 >> shift) & 0x1) as u8) } // [0]
    }

    #[doc="Returns true if FACT != 0"]
    #[inline] pub fn test_fact<I: Into<bits::R27>>(&self, index: I) -> bool{
        self.fact(index) != 0
    }

    #[doc="Sets the FACT field."]
    #[inline] pub fn set_fact<I: Into<bits::R27>, V: Into<bits::U1>>(mut self, index: I, value: V) -> Self {
        let index: usize = index.into().value() as usize;
        let value: bits::U1 = value.into();
        let value: u32 = value.into();
        let shift: usize = 0 + index;
        self.0 &= !(0x1 << shift);
        self.0 |= value << shift;
        self
    }

}

impl From<u32> for Fa1r {
    #[inline]
    fn from(other: u32) -> Self {
         Fa1r(other)
    }
}

impl ::core::fmt::Display for Fa1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fa1r {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        if self.fact(0) != 0 { try!(write!(f, " fact[0]"))}
        if self.fact(1) != 0 { try!(write!(f, " fact[1]"))}
        if self.fact(2) != 0 { try!(write!(f, " fact[2]"))}
        if self.fact(3) != 0 { try!(write!(f, " fact[3]"))}
        if self.fact(4) != 0 { try!(write!(f, " fact[4]"))}
        if self.fact(5) != 0 { try!(write!(f, " fact[5]"))}
        if self.fact(6) != 0 { try!(write!(f, " fact[6]"))}
        if self.fact(7) != 0 { try!(write!(f, " fact[7]"))}
        if self.fact(8) != 0 { try!(write!(f, " fact[8]"))}
        if self.fact(9) != 0 { try!(write!(f, " fact[9]"))}
        if self.fact(10) != 0 { try!(write!(f, " fact[10]"))}
        if self.fact(11) != 0 { try!(write!(f, " fact[11]"))}
        if self.fact(12) != 0 { try!(write!(f, " fact[12]"))}
        if self.fact(13) != 0 { try!(write!(f, " fact[13]"))}
        if self.fact(14) != 0 { try!(write!(f, " fact[14]"))}
        if self.fact(15) != 0 { try!(write!(f, " fact[15]"))}
        if self.fact(16) != 0 { try!(write!(f, " fact[16]"))}
        if self.fact(17) != 0 { try!(write!(f, " fact[17]"))}
        if self.fact(18) != 0 { try!(write!(f, " fact[18]"))}
        if self.fact(19) != 0 { try!(write!(f, " fact[19]"))}
        if self.fact(20) != 0 { try!(write!(f, " fact[20]"))}
        if self.fact(21) != 0 { try!(write!(f, " fact[21]"))}
        if self.fact(22) != 0 { try!(write!(f, " fact[22]"))}
        if self.fact(23) != 0 { try!(write!(f, " fact[23]"))}
        if self.fact(24) != 0 { try!(write!(f, " fact[24]"))}
        if self.fact(25) != 0 { try!(write!(f, " fact[25]"))}
        if self.fact(26) != 0 { try!(write!(f, " fact[26]"))}
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Filter Bank Register 0"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fr0(pub u32);
impl Fr0 {
    #[doc="Filter Word"]
    #[inline] pub fn fb(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if FB != 0"]
    #[inline] pub fn test_fb(&self) -> bool {
        self.fb() != 0
    }

    #[doc="Sets the FB field."]
    #[inline] pub fn set_fb<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fr0 {
    #[inline]
    fn from(other: u32) -> Self {
         Fr0(other)
    }
}

impl ::core::fmt::Display for Fr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fr0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

#[doc="Filter Bank Register 1"]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Fr1(pub u32);
impl Fr1 {
    #[doc="Filter Word"]
    #[inline] pub fn fb(&self) -> bits::U32 {
        unsafe { ::core::mem::transmute(((self.0 >> 0) & 0xffffffff) as u32) } // [31:0]
    }

    #[doc="Returns true if FB != 0"]
    #[inline] pub fn test_fb(&self) -> bool {
        self.fb() != 0
    }

    #[doc="Sets the FB field."]
    #[inline] pub fn set_fb<V: Into<bits::U32>>(mut self, value: V) -> Self {
        let value: bits::U32 = value.into();
        let value: u32 = value.into();
        self.0 &= !(0xffffffff << 0);
        self.0 |= value << 0;
        self
    }

}

impl From<u32> for Fr1 {
    #[inline]
    fn from(other: u32) -> Self {
         Fr1(other)
    }
}

impl ::core::fmt::Display for Fr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
         self.0.fmt(f)
    }
}

impl ::core::fmt::Debug for Fr1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        try!(write!(f, "[0x{:08x}", self.0));
        try!(write!(f, "]"));
        Ok(())
    }
}

