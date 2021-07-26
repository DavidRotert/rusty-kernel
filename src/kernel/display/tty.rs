/**
 * TTY functions for displaying a console
 */

use super::super::arch::i386::display::vga_buffer;

#[cfg(target_arch = "x86")]
pub fn clear() {
    vga_buffer::clear_vga(vga_buffer::VgaColor::Black);
}

#[cfg(target_arch = "x86")]
pub fn clear_panic() {
    vga_buffer::clear_vga(vga_buffer::VgaColor::Red);
}

#[cfg(target_arch = "x86")]
pub fn test() {
    vga_buffer::test();
}

#[cfg(target_arch = "x86")]
pub fn enable_cursor() {
    vga_buffer::enable_cursor(0, 15);
}

#[cfg(target_arch = "x86")]
pub fn put_char(buf_pos: usize, ch: u8) {
    vga_buffer::put_char(buf_pos, ch, vga_buffer::VgaColor::White, vga_buffer::VgaColor::Black);
}

pub fn update_cursor_pos(col: u32, line: u32) {
    vga_buffer::update_cursor_pos(col, line);
}