#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(wos_os_n71::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use wos_os_n71::{memory, println};
use x86_64::structures::paging::Page;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // use wos_os_n71::memory::active_level4_table;
    // use wos_os_n71::memory::translate_addr;
    use x86_64::{structures::paging::Translate, VirtAddr};

    println!("Hello World{}", "!");
    wos_os_n71::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // mapperを初期化
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // 恒等対応しているVGAバッファのページ
        0xb8000,
        // コードページのどこか
        0x201008,
        // スタックぺージのどこか
        0x0100_0020_1a10,
        // 物理アドレス 0 にマップされている仮想アドレス
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        // let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        // ライブラリの実装を使ったもの
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

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
