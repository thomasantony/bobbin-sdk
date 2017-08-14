#[allow(unused_imports)] use bobbin_common::bits;
pub use kinetis_common::chip::adc::*;

pub const ADC0: Adc0 = Periph(0x4003b000, Adc0Id {});
pub const ADC1: Adc1 = Periph(0x400bb000, Adc1Id {});

#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc0Id {}
pub type Adc0 = Periph<Adc0Id>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(hidden)]
pub struct Adc1Id {}
pub type Adc1 = Periph<Adc1Id>;

impl super::sig::Signal<super::sig::Adc0Dp0> for Adc0 {}
impl super::sig::SignalAdcDp0<super::sig::Adc0Dp0> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Dp1> for Adc0 {}
impl super::sig::SignalAdcDp1<super::sig::Adc0Dp1> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Dp2> for Adc0 {}
impl super::sig::SignalAdcDp2<super::sig::Adc0Dp2> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Dp3> for Adc0 {}
impl super::sig::SignalAdcDp3<super::sig::Adc0Dp3> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Dm0> for Adc0 {}
impl super::sig::SignalAdcDm0<super::sig::Adc0Dm0> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Dm1> for Adc0 {}
impl super::sig::SignalAdcDm1<super::sig::Adc0Dm1> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Dm2> for Adc0 {}
impl super::sig::SignalAdcDm2<super::sig::Adc0Dm2> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Dm3> for Adc0 {}
impl super::sig::SignalAdcDm3<super::sig::Adc0Dm3> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se4a> for Adc0 {}
impl super::sig::SignalAdcSe4a<super::sig::Adc0Se4a> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se5a> for Adc0 {}
impl super::sig::SignalAdcSe5a<super::sig::Adc0Se5a> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se6a> for Adc0 {}
impl super::sig::SignalAdcSe6a<super::sig::Adc0Se6a> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se7a> for Adc0 {}
impl super::sig::SignalAdcSe7a<super::sig::Adc0Se7a> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se4b> for Adc0 {}
impl super::sig::SignalAdcSe4b<super::sig::Adc0Se4b> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se5b> for Adc0 {}
impl super::sig::SignalAdcSe5b<super::sig::Adc0Se5b> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se6b> for Adc0 {}
impl super::sig::SignalAdcSe6b<super::sig::Adc0Se6b> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se7b> for Adc0 {}
impl super::sig::SignalAdcSe7b<super::sig::Adc0Se7b> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se8> for Adc0 {}
impl super::sig::SignalAdcSe8<super::sig::Adc0Se8> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se9> for Adc0 {}
impl super::sig::SignalAdcSe9<super::sig::Adc0Se9> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se10> for Adc0 {}
impl super::sig::SignalAdcSe10<super::sig::Adc0Se10> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se11> for Adc0 {}
impl super::sig::SignalAdcSe11<super::sig::Adc0Se11> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se12> for Adc0 {}
impl super::sig::SignalAdcSe12<super::sig::Adc0Se12> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se13> for Adc0 {}
impl super::sig::SignalAdcSe13<super::sig::Adc0Se13> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se14> for Adc0 {}
impl super::sig::SignalAdcSe14<super::sig::Adc0Se14> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se15> for Adc0 {}
impl super::sig::SignalAdcSe15<super::sig::Adc0Se15> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se16> for Adc0 {}
impl super::sig::SignalAdcSe16<super::sig::Adc0Se16> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se17> for Adc0 {}
impl super::sig::SignalAdcSe17<super::sig::Adc0Se17> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se18> for Adc0 {}
impl super::sig::SignalAdcSe18<super::sig::Adc0Se18> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se19> for Adc0 {}
impl super::sig::SignalAdcSe19<super::sig::Adc0Se19> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se20> for Adc0 {}
impl super::sig::SignalAdcSe20<super::sig::Adc0Se20> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se21> for Adc0 {}
impl super::sig::SignalAdcSe21<super::sig::Adc0Se21> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se22> for Adc0 {}
impl super::sig::SignalAdcSe22<super::sig::Adc0Se22> for Adc0 {}
impl super::sig::Signal<super::sig::Adc0Se23> for Adc0 {}
impl super::sig::SignalAdcSe23<super::sig::Adc0Se23> for Adc0 {}

impl super::sig::Signal<super::sig::Adc1Dp0> for Adc1 {}
impl super::sig::SignalAdcDp0<super::sig::Adc1Dp0> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Dp1> for Adc1 {}
impl super::sig::SignalAdcDp1<super::sig::Adc1Dp1> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Dp2> for Adc1 {}
impl super::sig::SignalAdcDp2<super::sig::Adc1Dp2> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Dp3> for Adc1 {}
impl super::sig::SignalAdcDp3<super::sig::Adc1Dp3> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Dm0> for Adc1 {}
impl super::sig::SignalAdcDm0<super::sig::Adc1Dm0> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Dm1> for Adc1 {}
impl super::sig::SignalAdcDm1<super::sig::Adc1Dm1> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Dm2> for Adc1 {}
impl super::sig::SignalAdcDm2<super::sig::Adc1Dm2> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Dm3> for Adc1 {}
impl super::sig::SignalAdcDm3<super::sig::Adc1Dm3> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se4a> for Adc1 {}
impl super::sig::SignalAdcSe4a<super::sig::Adc1Se4a> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se5a> for Adc1 {}
impl super::sig::SignalAdcSe5a<super::sig::Adc1Se5a> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se6a> for Adc1 {}
impl super::sig::SignalAdcSe6a<super::sig::Adc1Se6a> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se7a> for Adc1 {}
impl super::sig::SignalAdcSe7a<super::sig::Adc1Se7a> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se4b> for Adc1 {}
impl super::sig::SignalAdcSe4b<super::sig::Adc1Se4b> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se5b> for Adc1 {}
impl super::sig::SignalAdcSe5b<super::sig::Adc1Se5b> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se6b> for Adc1 {}
impl super::sig::SignalAdcSe6b<super::sig::Adc1Se6b> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se7b> for Adc1 {}
impl super::sig::SignalAdcSe7b<super::sig::Adc1Se7b> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se8> for Adc1 {}
impl super::sig::SignalAdcSe8<super::sig::Adc1Se8> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se9> for Adc1 {}
impl super::sig::SignalAdcSe9<super::sig::Adc1Se9> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se10> for Adc1 {}
impl super::sig::SignalAdcSe10<super::sig::Adc1Se10> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se11> for Adc1 {}
impl super::sig::SignalAdcSe11<super::sig::Adc1Se11> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se12> for Adc1 {}
impl super::sig::SignalAdcSe12<super::sig::Adc1Se12> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se13> for Adc1 {}
impl super::sig::SignalAdcSe13<super::sig::Adc1Se13> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se14> for Adc1 {}
impl super::sig::SignalAdcSe14<super::sig::Adc1Se14> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se15> for Adc1 {}
impl super::sig::SignalAdcSe15<super::sig::Adc1Se15> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se16> for Adc1 {}
impl super::sig::SignalAdcSe16<super::sig::Adc1Se16> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se17> for Adc1 {}
impl super::sig::SignalAdcSe17<super::sig::Adc1Se17> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se18> for Adc1 {}
impl super::sig::SignalAdcSe18<super::sig::Adc1Se18> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se21> for Adc1 {}
impl super::sig::SignalAdcSe21<super::sig::Adc1Se21> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se22> for Adc1 {}
impl super::sig::SignalAdcSe22<super::sig::Adc1Se22> for Adc1 {}
impl super::sig::Signal<super::sig::Adc1Se23> for Adc1 {}
impl super::sig::SignalAdcSe23<super::sig::Adc1Se23> for Adc1 {}


pub const ADC0_CH0: Channel<Adc0Ch0Id, Adc0Id> = Channel { periph: ADC0, index: 0, id: Adc0Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch0Id {}
pub type Adc0Ch0 = Channel<Adc0Ch0Id, Adc0Id>;

pub const ADC0_CH1: Channel<Adc0Ch1Id, Adc0Id> = Channel { periph: ADC0, index: 1, id: Adc0Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc0Ch1Id {}
pub type Adc0Ch1 = Channel<Adc0Ch1Id, Adc0Id>;

pub const ADC1_CH0: Channel<Adc1Ch0Id, Adc1Id> = Channel { periph: ADC1, index: 0, id: Adc1Ch0Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch0Id {}
pub type Adc1Ch0 = Channel<Adc1Ch0Id, Adc1Id>;

pub const ADC1_CH1: Channel<Adc1Ch1Id, Adc1Id> = Channel { periph: ADC1, index: 1, id: Adc1Ch1Id {} }; 
#[derive(Clone, Copy, PartialEq)]
#[doc(hidden)]
pub struct Adc1Ch1Id {}
pub type Adc1Ch1 = Channel<Adc1Ch1Id, Adc1Id>;
