pub mod writer;

use core::fmt;

use crate::{boot::boot_info, serial_println};
use lazy_static::lazy_static;
use limine::framebuffer::Framebuffer;
use spin::Mutex;
use writer::WRITER;

pub struct Buffer {
    fb: &'static Framebuffer<'static>,
}

impl Buffer {
    fn new() -> Self {
        Self {
            fb: &boot_info().framebuffer,
        }
    }

    pub fn write_pixel(&mut self, x: u64, y: u64, color: u32) {
        if (x >= self.fb.height()) || (y >= self.fb.width()) {
            serial_println!("Buffer out of bounds: {}x{}", x, y);
            return;
        }

        let offset = (x * self.fb.pitch() + y * 4) as usize;
        unsafe {
            self.fb
                .addr()
                .add(offset)
                .cast::<u32>()
                .write_volatile(color);
        }
    }

    pub fn scroll_lines(&mut self, lines: u64) {
        let height = self.fb.height();
        if lines == 0 || lines >= height {
            self.clear_rows(0, height);
            return;
        }

        let pitch = self.fb.pitch();
        let bytes_per_row = pitch as u64;

        let src_offset = (lines * pitch) as usize;
        let dst = self.fb.addr();
        let src = unsafe { dst.add(src_offset) };
        let bytes_to_move = ((height - lines) * bytes_per_row) as usize;

        unsafe {
            core::ptr::copy(src, dst, bytes_to_move);
        }

        self.clear_rows(height - lines, lines);
    }

    pub fn clear_rows(&mut self, start_row: u64, count: u64) {
        let pitch = self.fb.pitch();
        let bytes_per_row = pitch as usize;
        let start_offset = (start_row * pitch) as usize;
        let total_bytes = (count * bytes_per_row as u64) as usize;

        let base = unsafe { self.fb.addr().add(start_offset) };

        unsafe {
            core::ptr::write_bytes(base, 0, total_bytes);
        }
    }
}

lazy_static! {
    pub static ref BUFFER: Mutex<Buffer> = Mutex::new(Buffer::new());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
