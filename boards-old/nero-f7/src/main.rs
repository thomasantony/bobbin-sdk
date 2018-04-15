#![no_std]
#![no_main]

#[macro_use]
extern crate nero_f7 as board;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("[start] Running tests for nucleo-f429zi");
    test_crc();
    test_systick();
    test_dma();
    test_adc();
    // test_i2c();
    test_spi_lora();
    println!("[done] All tests passed");
    loop {}
}

fn test_crc() {
    use board::hal::crc::*;

    let crc = CRC;

    // println!("# Setting up CRC");

    crc.rcc_enable();
    // println!("# Starting CRC");

    let expect: [u32;4 ] = [0xffffffff, 0xc704dd7b, 0x6dc5a6ee,0x491308c2];


    for i in 0..4 {
        // println!("{:08x}", crc.read());
        assert_eq!(crc.read(), expect[i]);
        crc.write(i as u32);
    }

    crc.rcc_disable();

    println!("[pass] CRC OK");    
}

fn test_systick() {
    use board::hal::systick::*;

    println!("# Testing SYSTICK");
    test_systick(&SYSTICK, ClockSource::Internal);
    println!("[pass] SYSTICK OK");
}


fn test_dma() {
    use board::hal::dma::*;    
    
    let dma = DMA2;
    let dma_ch = DMA2_STREAM0;

    dma.rcc_enable();

    let src = [0xffu8; 1024];
    let dst = [0u8; 1024];

    dma_ch    
        .set_pa(&src as *const u8 as u32)
        .set_m0a(&dst as *const u8 as u32)
        .set_psize(Size::Bit8)
        .set_pinc(true)
        .set_msize(Size::Bit8)
        .set_minc(true)
        .set_dir(Dir::MtoM)
        .set_ndt(1024)
        .set_tcie(true)
        .clr_teif()
        .clr_tcif();

    println!("Starting DMA Transfer");
    dma_ch.clr_tcif().set_enabled(true);
    while !dma_ch.tcif() {}
    for i in 0..1024 {
        assert_eq!(src[i], dst[i]);
    }

    dma.rcc_disable();
    println!("[pass] DMA OK");
}

fn test_adc() {
    use board::chip::c_adc::*;
    use board::hal::adc::*;

    let adc = ADC1;
    let adc_temp = ADC1_TEMP;
    let adc_ref = ADC1_REF;

    adc.rcc_enable();
    adc.set_enabled(true).calibrate();
    C_ADC.with_ccr(|r| r.set_tsvrefe(1));
    // adc.with_smpr(|r| r.set_smp(0b111));
    

    let t: u8 = <AdcCh as AnalogRead<u8>>::start(&adc_temp).analog_read();
    let v: u8 = <AdcCh as AnalogRead<u8>>::start(&adc_ref).analog_read();

    println!("# t: {} v: {}", t, v);

    // assert!(t > 110 && t < 130);
    // assert!(v > 220 && t < 240);


    adc.rcc_disable();

    println!("[pass] ADC OK")
}


// fn test_i2c() {
//     use board::hal::gpio::*;
//     use board::hal::i2c::*;
//     use board::common::bits::*;

//     let addr: U7 = U7::from(0x60);

//     let i2c = I2C1;
//     let i2c_port = GPIOB;
//     let i2c_scl = PB8; // D15
//     let i2c_sda = PB9; // D14

//     i2c.rcc_enable();
//     i2c_port.rcc_enable();

//     // Attached to MPL3115A2 
//     // NOTE: SCL and SCA must have pull-up resistors.
//     // NOTE: HSI Clock must be enabled.

//     i2c_scl.mode_i2c_scl(&i2c).open_drain();
//     i2c_sda.mode_i2c_sda(&i2c).open_drain();

//     // i2c.set_config(|c| c.set_timing(0x8.into(), 0x3.into(), 0x0.into(), 0xd.into(), 0x12.into()));
//     i2c.set_enabled(false);
//     // i2c.set_timingr(|_| Timingr(0x00300619));
//         // Set Peripheral Clock Frequency = 42Mhz
//     i2c.with_cr2(|r| r.set_freq(42));
    
//     // Set Clock Speed = 100khz (divisor = 42Mhz / 100khz = 420)
//     i2c.set_ccr(|_| Ccr(0).set_f_s(0).set_duty(0).set_ccr(420));

//     // Set Rise Time Register = 43
//     i2c.set_trise(|_| Trise(0).set_trise(43));

//     assert_eq!(i2c.read_reg(addr, 0x0c), 0xc4);
   
//     // println!("Mode:  0x{:08x}", i2c.read_reg(addr, 0x26));
    

//     i2c.write_reg(addr, 0x26, 0xb8); // OSR = 128
//     i2c.write_reg(addr, 0x13, 0x06); // Enable Data Flags
//     i2c.write_reg(addr, 0x26, 0xb9); // Set Active
//     // println!("Mode:  0x{:08x}", i2c.read_reg(addr, 0x26));

//     loop {
//         while i2c.read_reg(addr, 0x00) != 0x04 {}    
//         let mut buf = [0u8; 5];
//         i2c.transfer(addr, &[0x01], &mut buf);
//         println!("# {:?}", buf);
//         // assert!(buf[0] == 0 && buf[1] != 0 && buf[2] != 0 && buf[3] != 0 && buf[4] != 0);
//         break
//     }

    

//     i2c_port.rcc_disable();
//     i2c.rcc_disable();
//     println!("[pass] I2C OK");
// }


fn test_spi_lora() {
    use board::hal::gpio::*;
    use board::hal::spi::*;

    let spi = SPI2;
    let port = GPIOB;
    let spi_sck = PB13;
    let spi_miso = PB14;
    let spi_mosi = PB15;
    let spi_nss = PB12;

    spi.rcc_enable();
    port.rcc_enable();

    // NOTE: Pins must be set with output speed HIGH or leading edge
    // of transmission will occasionally be missed.

    spi_miso.mode_spi_miso(&spi).speed_high().pull_up();
    spi_mosi.mode_spi_mosi(&spi).speed_high().push_pull();
    spi_sck.mode_spi_sck(&spi).speed_high();
    spi_nss.mode_output();

    spi.set_config(|cfg| cfg
        .set_data_size(DataSize::Bits8)
        .set_master(true)
        .set_baud_divider(0b100.into())
    );
    spi.with_cr2(|r| r.set_frxth(1));
    spi.set_output_enabled(true).set_enabled(true);


    let test_data = [(0x42, 0x12), (0x01, 0x09), (0x02, 0x1a), (0x03, 0x0b), (0x04, 0x00), (0x05, 0x52), (0x06, 0x6c)];

    for &(tx, rx) in test_data.iter() {
        println!("0x{:02x}: 0x{:02x}", tx, rx);
        assert_eq!(reg_read(&spi, &spi_nss, tx), rx);
    }


    println!("[pass] SPI OK");
    spi.rcc_disable();
    spi_sck.mode_analog();
    spi_mosi.mode_analog();
    spi_miso.mode_analog();
    spi_nss.mode_analog();

    fn transfer(spi: &SpiPeriph, nss: &GpioPin, src: &[u8], dst: &mut[u8]) {
        let mut i = 0;
        let mut j = 0;
        nss.set_output(false);
        loop {
            if i < src.len() && spi.can_tx() {
                spi.tx(src[i]);
                i += 1;
            }
            if j < dst.len() && spi.can_rx() {
                dst[j] = spi.rx();
                j += 1;
            }
            if j == dst.len() {
                break;
            }        
        }
        nss.set_output(true);
    }

    fn reg_read(spi: &SpiPeriph, nss: &GpioPin, reg: u8) -> u8 {
        let cmd = [reg, 0xff];
        let mut buf = [0u8, 0u8];
        transfer(spi, nss, &cmd, &mut buf);
        buf[1]
    }
    
}