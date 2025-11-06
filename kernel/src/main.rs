#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod boot;
mod framebuffer;
pub mod gdt;
pub mod interrupt;
pub mod serial;

use framebuffer::writer::WRITER;
use x86_64::instructions::hlt;

fn main() -> ! {
    println!("emuOS!");

    let fb = &boot::boot_info().framebuffer;
    println!("{}x{}", fb.width(), fb.height());
    println!("{}B", fb.pitch());
    println!("{}B", fb.bpp() / 8);

    for _ in 0..60 {
        print!("Hello, world!");
    }

    let mut buf = WRITER.lock();
    buf.change_color(0xFF00FF00);
    drop(buf);

    for _ in 0..60 {
        print!("Hello, Tirth!");
    }

    let mut buf = WRITER.lock();
    buf.change_color(0xFF0000FF);
    drop(buf);

    for _ in 0..60 {
        println!("EMUOS");
    }

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
