#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("Hello again!! i am writing from Rust! HaHa!! We are Oxide!! yehhhhhhhhhhhhhhhhhh!!!!");
    println!(", some numbers: {} {}", 42, 1.337);

    println!("Hello World{}", "!");

    panic!("Some panic message");

    loop {}
}
