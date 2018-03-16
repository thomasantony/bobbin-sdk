#[allow(unused_imports)] use ::bobbin_common::*;
pub use ::hal::pit::*;

periph!( PIT, Pit, PIT_PERIPH, PitPeriph, 0x40037000, 0x0d);

channel!(PIT_CH0, PitCh0, PIT, Pit, PIT_CH0_CH, PitCh, PIT_PERIPH, 0);
channel!(PIT_CH1, PitCh1, PIT, Pit, PIT_CH1_CH, PitCh, PIT_PERIPH, 1);
channel!(PIT_CH2, PitCh2, PIT, Pit, PIT_CH2_CH, PitCh, PIT_PERIPH, 2);
channel!(PIT_CH3, PitCh3, PIT, Pit, PIT_CH3_CH, PitCh, PIT_PERIPH, 3);