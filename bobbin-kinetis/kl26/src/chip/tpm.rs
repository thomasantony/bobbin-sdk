#[allow(unused_imports)] use bobbin_common::*;

pub use kinetis_common::chip::tpm::*;

periph!( TPM0, Tpm0, _TPM0, TpmPeriph, 0x40038000);
periph!( TPM1, Tpm1, _TPM1, TpmPeriph, 0x40039000);
periph!( TPM2, Tpm2, _TPM2, TpmPeriph, 0x4003a000);

impl super::sig::Signal<super::sig::Tpm0Ch0> for Tpm0Ch0 {}
impl super::sig::SignalTpm<super::sig::Tpm0Ch0> for Tpm0Ch0 {}
impl super::sig::Signal<super::sig::Tpm0Ch1> for Tpm0Ch1 {}
impl super::sig::SignalTpm<super::sig::Tpm0Ch1> for Tpm0Ch1 {}
impl super::sig::Signal<super::sig::Tpm0Ch2> for Tpm0Ch2 {}
impl super::sig::SignalTpm<super::sig::Tpm0Ch2> for Tpm0Ch2 {}
impl super::sig::Signal<super::sig::Tpm0Ch3> for Tpm0Ch3 {}
impl super::sig::SignalTpm<super::sig::Tpm0Ch3> for Tpm0Ch3 {}
impl super::sig::Signal<super::sig::Tpm0Ch4> for Tpm0Ch4 {}
impl super::sig::SignalTpm<super::sig::Tpm0Ch4> for Tpm0Ch4 {}
impl super::sig::Signal<super::sig::Tpm0Ch5> for Tpm0Ch5 {}
impl super::sig::SignalTpm<super::sig::Tpm0Ch5> for Tpm0Ch5 {}

impl super::sig::Signal<super::sig::Tpm1Ch0> for Tpm1Ch0 {}
impl super::sig::SignalTpm<super::sig::Tpm1Ch0> for Tpm1Ch0 {}
impl super::sig::Signal<super::sig::Tpm1Ch1> for Tpm1Ch1 {}
impl super::sig::SignalTpm<super::sig::Tpm1Ch1> for Tpm1Ch1 {}
impl super::sig::Signal<super::sig::Tpm1Ch2> for Tpm1Ch2 {}
impl super::sig::SignalTpm<super::sig::Tpm1Ch2> for Tpm1Ch2 {}
impl super::sig::Signal<super::sig::Tpm1Ch3> for Tpm1Ch3 {}
impl super::sig::SignalTpm<super::sig::Tpm1Ch3> for Tpm1Ch3 {}
impl super::sig::Signal<super::sig::Tpm1Ch4> for Tpm1Ch4 {}
impl super::sig::SignalTpm<super::sig::Tpm1Ch4> for Tpm1Ch4 {}
impl super::sig::Signal<super::sig::Tpm1Ch5> for Tpm1Ch5 {}
impl super::sig::SignalTpm<super::sig::Tpm1Ch5> for Tpm1Ch5 {}

impl super::sig::Signal<super::sig::Tpm2Ch0> for Tpm2Ch0 {}
impl super::sig::SignalTpm<super::sig::Tpm2Ch0> for Tpm2Ch0 {}
impl super::sig::Signal<super::sig::Tpm2Ch1> for Tpm2Ch1 {}
impl super::sig::SignalTpm<super::sig::Tpm2Ch1> for Tpm2Ch1 {}
impl super::sig::Signal<super::sig::Tpm2Ch2> for Tpm2Ch2 {}
impl super::sig::SignalTpm<super::sig::Tpm2Ch2> for Tpm2Ch2 {}
impl super::sig::Signal<super::sig::Tpm2Ch3> for Tpm2Ch3 {}
impl super::sig::SignalTpm<super::sig::Tpm2Ch3> for Tpm2Ch3 {}
impl super::sig::Signal<super::sig::Tpm2Ch4> for Tpm2Ch4 {}
impl super::sig::SignalTpm<super::sig::Tpm2Ch4> for Tpm2Ch4 {}
impl super::sig::Signal<super::sig::Tpm2Ch5> for Tpm2Ch5 {}
impl super::sig::SignalTpm<super::sig::Tpm2Ch5> for Tpm2Ch5 {}


channel!(TPM0_CH0, Tpm0Ch0, TPM0, Tpm0, _TPM0_CH0, TpmCh, _TPM0, 0);
channel!(TPM0_CH1, Tpm0Ch1, TPM0, Tpm0, _TPM0_CH1, TpmCh, _TPM0, 1);
channel!(TPM0_CH2, Tpm0Ch2, TPM0, Tpm0, _TPM0_CH2, TpmCh, _TPM0, 2);
channel!(TPM0_CH3, Tpm0Ch3, TPM0, Tpm0, _TPM0_CH3, TpmCh, _TPM0, 3);
channel!(TPM0_CH4, Tpm0Ch4, TPM0, Tpm0, _TPM0_CH4, TpmCh, _TPM0, 4);
channel!(TPM0_CH5, Tpm0Ch5, TPM0, Tpm0, _TPM0_CH5, TpmCh, _TPM0, 5);
channel!(TPM1_CH0, Tpm1Ch0, TPM1, Tpm1, _TPM1_CH0, TpmCh, _TPM1, 0);
channel!(TPM1_CH1, Tpm1Ch1, TPM1, Tpm1, _TPM1_CH1, TpmCh, _TPM1, 1);
channel!(TPM1_CH2, Tpm1Ch2, TPM1, Tpm1, _TPM1_CH2, TpmCh, _TPM1, 2);
channel!(TPM1_CH3, Tpm1Ch3, TPM1, Tpm1, _TPM1_CH3, TpmCh, _TPM1, 3);
channel!(TPM1_CH4, Tpm1Ch4, TPM1, Tpm1, _TPM1_CH4, TpmCh, _TPM1, 4);
channel!(TPM1_CH5, Tpm1Ch5, TPM1, Tpm1, _TPM1_CH5, TpmCh, _TPM1, 5);
channel!(TPM2_CH0, Tpm2Ch0, TPM2, Tpm2, _TPM2_CH0, TpmCh, _TPM2, 0);
channel!(TPM2_CH1, Tpm2Ch1, TPM2, Tpm2, _TPM2_CH1, TpmCh, _TPM2, 1);
channel!(TPM2_CH2, Tpm2Ch2, TPM2, Tpm2, _TPM2_CH2, TpmCh, _TPM2, 2);
channel!(TPM2_CH3, Tpm2Ch3, TPM2, Tpm2, _TPM2_CH3, TpmCh, _TPM2, 3);
channel!(TPM2_CH4, Tpm2Ch4, TPM2, Tpm2, _TPM2_CH4, TpmCh, _TPM2, 4);
channel!(TPM2_CH5, Tpm2Ch5, TPM2, Tpm2, _TPM2_CH5, TpmCh, _TPM2, 5);
