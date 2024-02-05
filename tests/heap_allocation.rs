#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(wos_os_n71::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use wos_os_n71::allocator::HEAP_SIZE;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    use wos_os_n71::allocator;
    use wos_os_n71::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    wos_os_n71::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    wos_os_n71::test_panic_handler(info)
}

/// アロケーションエラーが起きないことを検証
#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

/// 大きな割り当てと再割り当てによる副スカイの割り当てをテスト
#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

/// 解放されたメモリを次の割り当てで再利用していることを検証
/// でなければ，メモリ不足を起こすはず
#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(i, *x);
    }
}

#[test_case]
fn many_boxes_long_lived() {
    let long_lived = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(i, *x);
    }

    assert_eq!(*long_lived, 1)
}
