use core::fmt::Arguments;

#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(msg: Arguments,
                               file: &'static str,
                               line: u32)
                               -> ! {
    println!("[panic] {}, {} at {}", msg, file, line);
    loop {}
}