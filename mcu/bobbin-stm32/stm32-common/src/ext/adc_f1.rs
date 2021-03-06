use bobbin_common::bits::*;
use bobbin_hal::analog::AnalogRead;
use adc_f1::*;

impl AdcPeriph {
    pub fn set_enabled(&self, value: bool) -> &Self {
        self.with_cr2(|r| r.set_adon(value))
    }

    pub fn set_sequence_channel(&self, sequence: u8, channel: u8) -> &Self {
        assert!(channel <= 17, "Channel must b 0..17");
        let channel = channel as u32;
        match sequence {
            1 => self.with_sqr3(|r| r.set_sq1(channel)),
            2 => self.with_sqr3(|r| r.set_sq2(channel)),
            3 => self.with_sqr3(|r| r.set_sq3(channel)),
            4 => self.with_sqr3(|r| r.set_sq4(channel)),
            5 => self.with_sqr3(|r| r.set_sq5(channel)),
            6 => self.with_sqr3(|r| r.set_sq6(channel)),
            7 => self.with_sqr2(|r| r.set_sq7(channel)),
            8 => self.with_sqr2(|r| r.set_sq8(channel)),
            9 => self.with_sqr2(|r| r.set_sq9(channel)),
            10 => self.with_sqr2(|r| r.set_sq10(channel)),
            11 => self.with_sqr2(|r| r.set_sq11(channel)),
            12 => self.with_sqr2(|r| r.set_sq12(channel)),
            13 => self.with_sqr1(|r| r.set_sq13(channel)),
            14 => self.with_sqr1(|r| r.set_sq14(channel)),
            15 => self.with_sqr1(|r| r.set_sq15(channel)),
            16 => self.with_sqr1(|r| r.set_sq16(channel)),
            _ => unimplemented!()
        };
        self
    }

    pub fn set_sequence_length(&self, length: u8) -> &Self {
        assert!(length > 0 && length <= 16, "length must be 1..16");
        self.with_sqr1(|r| r.set_l((length - 1) as u32))
    }

    pub fn start(&self) -> &Self {
        // self.with_cr2(|r| r.set_extsel(0b111));
        self.with_cr2(|r| r.set_adon(1))
    }

    // pub fn set_continuous(&self, value: bool) {
    //     let value = if value { 1 } else { 0 };
    //     self.with_cr2(|r| r.set_cont(value))
    // }

    // pub fn set_resolution(&self, value: Resolution) {
    //     self.with_cr1(|r| r.set_res(value as u32))
    // }

    pub fn calibrate(&self) -> &Self {
        self.with_cr2(|r| r.set_cal(true));
        while self.cr2().cal() == 1 {}
        self
    }

    #[inline]
    pub fn complete(&self) -> bool {
        self.sr().eoc() != 0
    }

    #[inline]
    pub fn data(&self) -> u16 {
        self.dr().data().value()
    }

    pub fn start_single(&self, channel: usize) -> &Self {
        self
            .set_sequence_channel(1, channel as u8)
            .set_sequence_length(1)
            .start()
    }
}

// pub trait AdcChExt {
//     fn start(&self) -> &Self;
//     fn complete(&self) -> bool;
//     fn wait(&self) -> &Self;
//     fn read(&self) -> u16;
// }

// impl<P, T> AdcChExt for Channel<P, T> {
//     fn start(&self) -> &Self {
//         self.periph
//             .set_sequence_channel(1, self.index as u8)
//             .set_sequence_length(1)
//             .start();
//         self
//     }

//     fn complete(&self) -> bool {
//         self.periph.complete()
//     }

//     fn wait(&self) -> &Self {
//         while !self.periph.complete() {}
//         self
//     }

//     fn read(&self) -> u16 {
//         self.periph.data()
//     }
// }

impl AnalogRead<U12> for AdcCh {
    fn start(&self) -> &Self {
        self.periph.start_single(self.index);
        self
    }

    fn is_complete(&self) -> bool {
        self.periph.complete()
    }

    fn read(&self) -> U12 {
        self.periph.dr().data_12()
    }
}