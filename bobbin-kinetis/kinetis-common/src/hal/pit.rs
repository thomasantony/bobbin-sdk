pub use chip::pit::*;

pub trait PitExt {
    fn enabled(&self) -> bool;
    fn set_enabled(&self, value: bool) -> &Self;
    fn set_timer_enabled(&self, index: usize, value: bool) -> &Self;
    fn set_load_value(&self, index: usize, value: u32) -> &Self;
    fn interrupt_flag(&self, index: usize) -> bool;
    fn clr_interrupt_flag(&self, index: usize) -> &Self;
}

impl PitExt for PitImpl {
    fn enabled(&self) -> bool {
        // MDIS = 1 to disable module
        self.mcr().mdis() == 0
    }

    fn set_enabled(&self, value: bool) -> &Self {
        // MDIS = 1 to disable module
        let value = if value { 0 } else { 1 };
        self.with_mcr(|r| r.set_mdis(value))        
    }

    fn set_timer_enabled(&self, index: usize, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_tctrl(index, |r| r.set_ten(value))
    }
    fn set_load_value(&self, index: usize, value: u32) -> &Self {
        self.set_ldval(index, Ldval(0).set_tsv(value))
    }    

    fn interrupt_flag(&self, index: usize) -> bool {        
        self.tflg(index).tif() != 0        
    }

    fn clr_interrupt_flag(&self, index: usize) -> &Self {
        self.set_tflg(index, Tflg(0).set_tif(1))
    }    
}