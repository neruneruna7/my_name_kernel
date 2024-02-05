#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(wos_os_n71::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use wos_os_n71::{
    println, serial_println,
    task::{keyboard, simple_executor::SimpleExecutor, Task},
    vga_buffer::{colored_letter, ColorCode},
};
use x86_64::structures::paging::Page;

use wos_os_n71::task::executor::Executor;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // use wos_os_n71::memory::active_level4_table;
    // use wos_os_n71::memory::translate_addr;
    use wos_os_n71::allocator;
    use wos_os_n71::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    wos_os_n71::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // mapperを初期化
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // let heap_value = Box::new(41);
    // println!("heap_value at {:p}", heap_value);

    // let mut vec = Vec::new();
    // for i in 0..500 {
    //     vec.push(i);
    // }
    // println!("vec at {:p}", vec.as_slice());

    // let referense_counted = Rc::new(vec![1, 2, 3]);
    // let cloned_reference = referense_counted.clone();
    // println!(
    //     "current reference count is {}",
    //     Rc::strong_count(&cloned_reference)
    // );
    // core::mem::drop(referense_counted);
    // println!(
    //     "reference count is {} now",
    //     Rc::strong_count(&cloned_reference)
    // );

    // 未使用のページをマップする
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // 新しいマッピングを使って、文字列`New!`を画面に書き出す
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    use wos_os_n71::task::keyboard::introduction::introduction_icon;
    use wos_os_n71::vga_buffer::colored_letter::ColoredString;
    use wos_os_n71::vga_buffer::Color;

    introduction_icon();

    let colored_string = ColoredString::from(
        "Hello Color World!",
        ColorCode::new(Color::Cyan, Color::DarkGray),
    );
    // colored_letter::color_print(colored_string);

    #[cfg(test)]
    test_main();

    // let mut executor = SimpleExecutor::new();
    let mut executor = Executor::new();
    // executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::introduction::print_keypresses()));
    executor.run();

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

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
