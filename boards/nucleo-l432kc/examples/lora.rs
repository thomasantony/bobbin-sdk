
#![no_std]
#![no_main]

#[macro_use]
extern crate nucleo_l432kc as board;

use board::hal::spi::*;
// use board::hal::RegisterPoll;
// use board::hal::nvic::{NvicEnabled, RegisterPoll};
// use board::clock::*;
// use core::ptr;

#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("Millis Driver Test");    
    test_spi_lora();    
    loop {}
}

/// RFM9x LoRa Radio on pins D10-D13
fn test_spi_lora() {
    use board::hal::gpio::*;
    use board::hal::spi::*;

    let spi = SPI1;

    let spi_miso = PB4; // D12
    let spi_mosi = PB5; // D11
    let spi_sck = PB3; // D13
    let spi_nss = PA11; // D10

    spi.rcc_enable();
    GPIOA.rcc_enable();
    GPIOB.rcc_enable();

    // NOTE: Pins must be set with output speed HIGH or leading edge
    // of transmission will occasionally be missed.

    spi_miso.mode_spi_miso(&spi).speed_high().pull_up();
    spi_mosi.mode_spi_mosi(&spi).speed_high().push_pull();
    spi_sck.mode_spi_sck(&spi).speed_high().push_pull();
    // spi_nss.mode_spi_nss(&spi).speed_high().push_pull();
    spi_nss.mode_output();

    spi.set_config(|cfg| cfg
        .set_data_size(DataSize::Bits8)
        .set_master(true)
        .set_baud_divider(0b100.into())
    );

    spi.with_cr2(|r| r.set_frxth(1));
    spi.set_output_enabled(true).set_enabled(true);

    let mut tx_buf = [0u8; 16];
    let mut rx_buf = [0u8; 16];
    let s = SpiDriver::new(spi, &mut tx_buf, &mut rx_buf);


    let test_data = [(0x42, 0x12), (0x01, 0x09), (0x02, 0x1a), (0x03, 0x0b), (0x04, 0x00), (0x05, 0x52), (0x06, 0x6c)];

    for &(tx, rx) in test_data.iter() {
        println!("0x{:02x}: 0x{:02x}", tx, rx);
        let v = s.reg_read(&spi_nss, tx);
        println!("   0x{:02x}", v);
        // assert_eq!(reg_read(&spi, &spi_nss, tx), rx);
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

use board::common::{Irq, Poll};
use board::common::digital::DigitalOutput;
use board::hal::gpio::GpioPin;
use board::hal::scb::SCB;
use board::hal::nvic;

use core::cell::{Cell, UnsafeCell};
use core::marker::PhantomData;
use core::slice;
use core::ptr;

pub struct SpiDriver<'a> {
    spi: SpiPeriph,
    done: UnsafeCell<bool>,
    tx_buf: *mut [u8],
    tx_len: Cell<usize>,
    tx_pos: Cell<usize>,
    rx_buf: *mut [u8],
    rx_len: Cell<usize>,
    rx_pos: Cell<usize>,
    _phantom: PhantomData<&'a mut [u8]>,
}

unsafe impl<'a> Sync for SpiDriver<'a> {}
unsafe impl<'a> Send for SpiDriver<'a> {}

impl<'a> SpiDriver<'a> {
    pub fn new<P: Into<SpiPeriph>>(spi: P, tx_buf: &'a mut [u8], rx_buf: &'a mut [u8]) -> Self {
        SpiDriver { 
            spi: spi.into(),
            done: UnsafeCell::new(false),
            tx_buf: tx_buf,
            tx_pos: Cell::new(0),
            tx_len: Cell::new(0),
            rx_buf: rx_buf,
            rx_pos: Cell::new(0),
            rx_len: Cell::new(0),
            _phantom: PhantomData,
        }
    }

    pub fn enable_irq<I: Irq>(&self, irq: &I) {        
        SCB.set_irq_handler(irq.irq_num() as usize, Some(irq.wrap(self)));
        nvic::set_enabled(irq.irq_num() as usize, true);
    }

    pub fn clear(&self) {
        self.set_done(false);
        self.tx_pos.set(0);
        self.tx_len.set(0);
        self.rx_pos.set(0);
        self.rx_len.set(0);
    }

    pub fn done(&self) -> bool {
        unsafe {
            ptr::read_volatile(self.done.get())
        }
    }

    pub fn set_done(&self, value: bool) {
        unsafe {
            ptr::write_volatile(self.done.get(), value)
        }
    }

    pub fn tx(&self) -> &mut [u8] {        
        unsafe {
            slice::from_raw_parts_mut(self.tx_buf as *mut u8, self.tx_len.get())
        }
    }

    pub fn rx(&self) -> &mut [u8] {        
        unsafe {
            slice::from_raw_parts_mut(self.rx_buf as *mut u8, self.rx_len.get())
        }
    }

    pub fn reg_read(&self, nss: &GpioPin, reg: u8) -> u8 {
        let mut buf = [0u8];
        self.transfer(nss, &[reg], &mut buf);
        buf[0]
    }

    pub fn reg_write(&self, nss: &GpioPin, reg: u8, value: u8) {
        let mut buf = [];
        self.transfer(nss, &[reg, value], &mut buf);
    }


    pub fn transfer(&self, nss: &GpioPin, tx_buf: &[u8], rx_buf: &mut [u8]) {
        nss.set_output(false);
        self.transfer_start(tx_buf, rx_buf.len());        
        loop {
            if self.transfer_done() {
                rx_buf.copy_from_slice(self.rx());
                self.clear();
                nss.set_output(true);
                break
            }            
        }
    }

    pub fn transfer_start(&self, tx_buf: &[u8], rx_len: usize) {
        self.clear();        
        self.tx_len.set(tx_buf.len());                
        &self.tx()[..tx_buf.len()].copy_from_slice(tx_buf);
        self.rx_len.set(rx_len);
    }

    pub fn transfer_done(&self) -> bool {
        self.done()
    }    
}

impl<'a> Poll for SpiDriver<'a> {
    fn poll(&self) {       
        // let isr = self.spi.isr();
    }
}