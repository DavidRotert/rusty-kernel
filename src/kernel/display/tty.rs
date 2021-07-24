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