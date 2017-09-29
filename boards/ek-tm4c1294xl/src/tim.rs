use hal::timer::*;
use clock::*;

pub const TIM: Timer3 = TIMER3;
// pub const TIM_CLK: u32 = 120_000;


pub fn init() {
    TIM.sysctl_set_enabled(true);
}

pub fn delay(ms: u32) {
    TIM.delay(ms * (TIM.clock(clk()).unwrap() / 1000));
}