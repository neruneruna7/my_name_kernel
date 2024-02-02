#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(wos_os_n71::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use wos_os_n71::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    wos_os_n71::init();
    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    // let ptr = 0xdeadbeaf as *mut u8;
    let ptr = 0x2054da as *mut u8;

    // コードページから読み込む
    unsafe {
        let x = *ptr;
    }
    println!("read worked");

    unsafe {
        *ptr = 42;
    }
    println!("write worked");

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    wos_os_n71::hit_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    wos_os_n71::hit_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    wos_os_n71::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    // trivialは些末なという意味みたい
    assert_eq!(1, 1);
}
