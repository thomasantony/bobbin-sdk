pub use bobbin_common::timer::*;
use chip::lpit::*;

pub trait LpitExt {
    fn enabled(&self) -> bool;
    fn set_enabled(&self, value: bool) -> &Self;

    fn doze_enabled(&self) -> bool;
    fn set_doze_enabled(&self, value: bool) -> &Self;

    fn ch_enabled(&self, index: usize) -> bool;
    fn set_ch_enabled(&self, index: usize, value: bool) -> &Self;

    fn ch_value(&self, index: usize) -> u32;
    fn set_ch_value(&self, index: usize, value: u32) -> &Self;

    fn ch_tie(&self, index: usize) -> bool;
    fn set_ch_tie(&self, index: usize, value: bool) -> &Self;

    fn ch_tif(&self, index: usize) -> bool;
    fn clr_ch_tif(&self, index: usize) -> &Self;    
}

pub trait LpitChannelExt {
    fn value(&self) -> u32;
    fn set_value(&self, value: u32) -> &Self;

    fn tie(&self) -> bool;
    fn set_tie(&self, value: bool) -> &Self;

    fn tif(&self) -> bool;
    fn clr_tif(&self) -> &Self;    

    fn wait_tif(&self) -> &Self;
}

impl<T> LpitExt for Periph<T> {
    fn enabled(&self) -> bool {
        self.mcr().m_cen() != 0
    }
    fn set_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_mcr(|r| r.set_m_cen(value))
    }

    fn doze_enabled(&self) -> bool {
        self.mcr().doze_en() != 0
    }
    fn set_doze_enabled(&self, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_mcr(|r| r.set_doze_en(value))
    }    

    fn ch_enabled(&self, index: usize) -> bool {
        self.tctrl(index).t_en() != 0
    }

    fn set_ch_enabled(&self, index: usize, value: bool) -> &Self {
        if value {
            self.set_setten(|r| r.set_set_t_en(index, 1))
        } else {
            self.set_clrten(|r| r.set_clr_t_en(index, 1))
        }
    }

    fn ch_value(&self, index: usize) -> u32 {
        self.tval(index).tmr_val().into()
    }
    fn set_ch_value(&self, index: usize, value: u32) -> &Self {
        self.set_tval(index, |r| r.set_tmr_val(value))
    }    

    fn ch_tie(&self, index: usize) -> bool {
        self.mier().tie(index) != 0
    }
    fn set_ch_tie(&self, index: usize, value: bool) -> &Self {
        let value = if value { 1 } else { 0 };
        self.with_mier(|r| r.set_tie(index, value))
    }    

    fn ch_tif(&self, index: usize) -> bool {
        self.msr().tif(index) != 0
    }
    fn clr_ch_tif(&self, index: usize) -> &Self {
        self.set_msr(|r| r.set_tif(index, 1))
    }       
}

impl<P,T> LpitChannelExt for Channel<P, T> {
    fn value(&self) -> u32 {
        self.periph().ch_value(self.index())
    }
    fn set_value(&self, value: u32) -> &Self {
        self.periph().set_ch_value(self.index(), value);
        self
    }

    fn tie(&self) -> bool {
        self.periph().ch_tie(self.index())
    }
    fn set_tie(&self, value: bool) -> &Self {
        self.periph().set_ch_tie(self.index(), value);
        self
    }

    fn tif(&self) -> bool {
        self.periph().ch_tif(self.index())
    }
    fn clr_tif(&self) -> &Self {
        self.periph().clr_ch_tif(self.index());
        self
    }
    fn wait_tif(&self) -> &Self {
        while !self.tif() {}
        self
    }
}

impl<P, T> Start<u32> for Channel<P, T> {
    fn start(&self, value: u32) -> &Self {       
        self.periph()
            .set_ch_value(self.index(), value - 1)
            .set_ch_enabled(self.index(), true);
        self
    }
}

impl<P, T> Timer<u32> for Channel<P, T> {
    fn stop(&self) -> &Self {
        self.periph().set_ch_enabled(self.index(), false);
        self
    }

    fn running(&self) -> bool {
        self.periph().ch_enabled(self.index())        
    }

    fn period(&self) -> u32 {
        self.periph().ch_value(self.index()) + 1        
    }
    
    fn set_period(&self, value: u32) -> &Self {
        self.periph().set_ch_value(self.index(), value - 1);
        self
    }

    fn counter(&self) -> u32 {
        self.periph().ch_value(self.index())
    }

    fn timeout_flag(&self) -> bool {
        self.tif()
    }

    fn clr_timeout_flag(&self) -> &Self {
        self.clr_tif()
    }
}

impl<P, T> SetCounter<u32> for Channel<P, T> {
    fn set_counter(&self, value: u32) -> &Self {
        self.periph().set_ch_value(self.index(), value);
        self
    }
}

impl<P, T> Delay<u32> for Channel<P, T> {
    fn delay(&self, value: u32) -> &Self {
        self
            .start(value)
            .clr_timeout_flag()
            .wait_timeout_flag()
            .stop()
    }
}
