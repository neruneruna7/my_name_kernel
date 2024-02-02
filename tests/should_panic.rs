#![no_std]
#![no_main]
// #![feature(custom_test_frameworks)]
// #![test_runner(test_runner)]
// #![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use wos_os_n71::{exit_qemu, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // test_main();
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// pub fn test_runner(tests: &[&dyn Fn()]) {
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//         serial_println!("[test did not panic]");
//         exit_qemu(QemuExitCode::Failed);
//     }
//     // ...これ，複数のテストを作れなくない？
//     // やっぱそうらしい

//     exit_qemu(QemuExitCode::Success);
// }

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    // 成功のエラーコードで終わる
    // should_panicと似たようなことができる
    exit_qemu(QemuExitCode::Success);
    loop {}
}

// #[test_case]
fn should_fail() {
    use wos_os_n71::serial_print;

    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}
