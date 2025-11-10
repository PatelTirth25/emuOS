#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod boot;
mod framebuffer;
pub mod gdt;
pub mod interrupt;
mod memory;
pub mod serial;

use alloc::vec;
use framebuffer::writer::WRITER;
use x86_64::instructions::hlt;

fn test_heap_allocations() {
    use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};

    // 1️⃣ Basic Box allocation
    let heap_value = Box::new(1234);
    println!("Box value at {:p} = {}", heap_value, heap_value);

    // 2️⃣ Vector test
    let mut vec = Vec::new();
    for i in 0..100 {
        vec.push(i);
    }
    println!("Vector sum: {}", vec.iter().sum::<u64>());

    // 3️⃣ String allocation
    let s = String::from("hello heap!");
    println!("Allocated string: {}", s);

    // 4️⃣ Reference counting test
    let rc_a = Rc::new(vec![1, 2, 3]);
    let rc_b = rc_a.clone();
    println!(
        "Rc test: {:p} {:p} (count = {})",
        &*rc_a,
        &*rc_b,
        Rc::strong_count(&rc_a)
    );

    // 5️⃣ Allocate lots of Boxes (stress test)
    for i in 0..1000 {
        let _b = Box::new(i);
    }
    println!("Heap stress test done ✅");
}

fn main() -> ! {
    println!("emuOS!");

    test_heap_allocations();

    // let fb = &boot::boot_info().framebuffer;
    // println!("{}x{}", fb.width(), fb.height());
    // println!("{}B", fb.pitch());
    // println!("{}B", fb.bpp() / 8);

    // for _ in 0..60 {
    //     print!("Hello, world!");
    // }
    //
    // let mut buf = WRITER.lock();
    // buf.change_color(0xFF00FF00);
    // drop(buf);
    //
    // for _ in 0..60 {
    //     print!("Hello, Tirth!");
    // }
    //
    // let mut buf = WRITER.lock();
    // buf.change_color(0xFF0000FF);
    // drop(buf);
    //
    // for _ in 0..60 {
    //     println!("EMUOS");
    // }

    loop {
        hlt();
    }
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}
