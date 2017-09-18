//! Signals

pub trait Signal<T> {}

pub trait AdcSe0 {}
pub trait SignalAdcSe0<T> {}
pub trait AdcSe1 {}
pub trait SignalAdcSe1<T> {}
pub trait AdcSe2 {}
pub trait SignalAdcSe2<T> {}
pub trait AdcSe3 {}
pub trait SignalAdcSe3<T> {}
pub trait AdcSe4 {}
pub trait SignalAdcSe4<T> {}
pub trait AdcSe5 {}
pub trait SignalAdcSe5<T> {}
pub trait AdcSe6 {}
pub trait SignalAdcSe6<T> {}
pub trait AdcSe7 {}
pub trait SignalAdcSe7<T> {}
pub trait AdcD0p {}
pub trait SignalAdcD0p<T> {}
pub trait AdcSe8 {}
pub trait SignalAdcSe8<T> {}
pub trait AdcD0n {}
pub trait SignalAdcD0n<T> {}
pub trait AdcSe9 {}
pub trait SignalAdcSe9<T> {}
pub trait AdcD1p {}
pub trait SignalAdcD1p<T> {}
pub trait AdcD1n {}
pub trait SignalAdcD1n<T> {}
pub trait Sck {}
pub trait SignalSck<T> {}
pub trait SckLb {}
pub trait SignalSckLb<T> {}
pub trait Miso {}
pub trait SignalMiso<T> {}
pub trait Mosi {}
pub trait SignalMosi<T> {}
pub trait Ce0 {}
pub trait SignalCe0<T> {}
pub trait Ce1 {}
pub trait SignalCe1<T> {}
pub trait Ce2 {}
pub trait SignalCe2<T> {}
pub trait Ce3 {}
pub trait SignalCe3<T> {}
pub trait Ce4 {}
pub trait SignalCe4<T> {}
pub trait Ce5 {}
pub trait SignalCe5<T> {}
pub trait Ce6 {}
pub trait SignalCe6<T> {}
pub trait Ce7 {}
pub trait SignalCe7<T> {}
pub trait Scl {}
pub trait SignalScl<T> {}
pub trait SclLb {}
pub trait SignalSclLb<T> {}
pub trait Sda {}
pub trait SignalSda<T> {}
pub trait Wir3 {}
pub trait SignalWir3<T> {}
pub trait SlScl {}
pub trait SignalSlScl<T> {}
pub trait SlSclLb {}
pub trait SignalSlSclLb<T> {}
pub trait SlSda {}
pub trait SignalSlSda<T> {}
pub trait SlSdaLb {}
pub trait SignalSlSdaLb<T> {}
pub trait SlMiso {}
pub trait SignalSlMiso<T> {}
pub trait SlMosi {}
pub trait SignalSlMosi<T> {}
pub trait SlWir3 {}
pub trait SignalSlWir3<T> {}
pub trait SlCe {}
pub trait SignalSlCe<T> {}
pub trait SlInt {}
pub trait SignalSlInt<T> {}
pub trait SlIntGp {}
pub trait SignalSlIntGp<T> {}
pub trait SlMisoLb {}
pub trait SignalSlMisoLb<T> {}
pub trait SlMosiLb {}
pub trait SignalSlMosiLb<T> {}
pub trait SlWir3Lb {}
pub trait SignalSlWir3Lb<T> {}
pub trait UartTx {}
pub trait SignalUartTx<T> {}
pub trait UartRx {}
pub trait SignalUartRx<T> {}
pub trait UartCts {}
pub trait SignalUartCts<T> {}
pub trait UartRts {}
pub trait SignalUartRts<T> {}

pub struct Adcse0 {}
pub struct Adcse1 {}
pub struct Adcse2 {}
pub struct Adcse3 {}
pub struct Adcse4 {}
pub struct Adcse5 {}
pub struct Adcse6 {}
pub struct Adcse7 {}
pub struct Adcd0pse8 {}
pub struct Adcd0nse9 {}
pub struct Adcd1p {}
pub struct Adcd1n {}
pub struct M0sck {}
pub struct M0scklb {}
pub struct M0miso {}
pub struct M0mosi {}
pub struct M0nce0 {}
pub struct M0nce1 {}
pub struct M0nce2 {}
pub struct M0nce3 {}
pub struct M0nce4 {}
pub struct M0nce5 {}
pub struct M0nce6 {}
pub struct M0nce7 {}
pub struct M0scl {}
pub struct M0scllb {}
pub struct M0sda {}
pub struct M0wir3 {}
pub struct M1sck {}
pub struct M1scklb {}
pub struct M1miso {}
pub struct M1mosi {}
pub struct M1nce0 {}
pub struct M1nce1 {}
pub struct M1nce2 {}
pub struct M1nce3 {}
pub struct M1nce4 {}
pub struct M1nce5 {}
pub struct M1nce6 {}
pub struct M1nce7 {}
pub struct M1scl {}
pub struct M1scllb {}
pub struct M1sda {}
pub struct M1wir3 {}
pub struct M2sck {}
pub struct M2scklb {}
pub struct M2miso {}
pub struct M2mosi {}
pub struct M2nce0 {}
pub struct M2nce1 {}
pub struct M2nce2 {}
pub struct M2nce3 {}
pub struct M2nce4 {}
pub struct M2nce5 {}
pub struct M2nce6 {}
pub struct M2nce7 {}
pub struct M2scl {}
pub struct M2scllb {}
pub struct M2sda {}
pub struct M2wir3 {}
pub struct M4sck {}
pub struct M4scklb {}
pub struct M4miso {}
pub struct M4mosi {}
pub struct M4nce0 {}
pub struct M4nce1 {}
pub struct M4nce2 {}
pub struct M4nce3 {}
pub struct M4nce4 {}
pub struct M4nce5 {}
pub struct M4nce6 {}
pub struct M4nce7 {}
pub struct M4scl {}
pub struct M4scllb {}
pub struct M4sda {}
pub struct M4wir3 {}
pub struct M5sck {}
pub struct M5scklb {}
pub struct M5miso {}
pub struct M5mosi {}
pub struct M5nce0 {}
pub struct M5nce1 {}
pub struct M5nce2 {}
pub struct M5nce3 {}
pub struct M5nce4 {}
pub struct M5nce5 {}
pub struct M5nce6 {}
pub struct M5nce7 {}
pub struct M5scl {}
pub struct M5scllb {}
pub struct M5sda {}
pub struct M5wir3 {}
pub struct Slscl {}
pub struct Slscllb {}
pub struct Slsda {}
pub struct Slsdalb {}
pub struct Slsmiso {}
pub struct Slsmosi {}
pub struct Slswir3 {}
pub struct Slnce {}
pub struct Slint {}
pub struct Slintgp {}
pub struct Slmisolb {}
pub struct Slmosilb {}
pub struct Slwir3lb {}
pub struct Uart0tx {}
pub struct Uart0rx {}
pub struct Ua0cts {}
pub struct Ua0rts {}
pub struct Uart1tx {}
pub struct Uart1rx {}
pub struct Ua1cts {}
pub struct Ua1rts {}
pub struct Slsck {}
pub struct Clkout {}
pub struct Gpio0 {}
pub struct Mxscklb {}
pub struct Mxscllb {}
pub struct Slmiso {}
pub struct Gpio1 {}
pub struct Mxmisolb {}
pub struct M2mis0 {}
pub struct Mxsdalb {}
pub struct Slwir3 {}
pub struct Slmosi {}
pub struct Gpio2 {}
pub struct Mxmosilb {}
pub struct Mxwir3lb {}
pub struct Gpio3 {}
pub struct Mxncelb {}
pub struct Trig1 {}
pub struct I2sWclk {}
pub struct Gpio4 {}
pub struct Xt32khz {}
pub struct Gpio5 {}
pub struct Gpio6 {}
pub struct I2sDat {}
pub struct Gpio7 {}
pub struct Trig0 {}
pub struct Gpio8 {}
pub struct Gpio9 {}
pub struct Gpio10 {}
pub struct Gpio11 {}
pub struct PdmData {}
pub struct Tcta0 {}
pub struct Gpio12 {}
pub struct PdmClk {}
pub struct Tctb0 {}
pub struct Gpio13 {}
pub struct Gpio14 {}
pub struct Swdck {}
pub struct Gpio15 {}
pub struct Swdio {}
pub struct Swo {}
pub struct Gpio16 {}
pub struct Cmpin0 {}
pub struct Cmprf1 {}
pub struct Gpio17 {}
pub struct Cmpin1 {}
pub struct Tcta1 {}
pub struct Gpio18 {}
pub struct Cmprf0 {}
pub struct Tctb1 {}
pub struct Gpio19 {}
pub struct I2sBclk {}
pub struct Tcta2 {}
pub struct Gpio20 {}
pub struct Tctb2 {}
pub struct Gpio21 {}
pub struct Tcta3 {}
pub struct Gpio22 {}
pub struct Tctb3 {}
pub struct Gpio23 {}
pub struct Cmpout {}
pub struct Gpio24 {}
pub struct Gpio25 {}
pub struct Gpio26 {}
pub struct M3nce0 {}
pub struct Gpio27 {}
pub struct Gpio28 {}
pub struct Gpio29 {}
pub struct Gpio30 {}
pub struct Gpio31 {}
pub struct Gpio32 {}
pub struct Gpio33 {}
pub struct M3nce7 {}
pub struct Gpio34 {}
pub struct Cmprf2 {}
pub struct M3nce1 {}
pub struct Gpio35 {}
pub struct M3nce2 {}
pub struct Gpio36 {}
pub struct M3nce3 {}
pub struct Trig2 {}
pub struct Gpio37 {}
pub struct M3nce4 {}
pub struct Trig3 {}
pub struct Gpio38 {}
pub struct M3wir3 {}
pub struct M3mosi {}
pub struct Gpio39 {}
pub struct Gpio40 {}
pub struct Gpio41 {}
pub struct M3nce5 {}
pub struct Gpio42 {}
pub struct M3scl {}
pub struct M3sck {}
pub struct M3scklb {}
pub struct M3scllb {}
pub struct Gpio43 {}
pub struct M3sda {}
pub struct M3miso {}
pub struct Gpio44 {}
pub struct Gpio45 {}
pub struct M3nce6 {}
pub struct Gpio46 {}
pub struct Gpio47 {}
pub struct Gpio48 {}
pub struct Gpio49 {}
