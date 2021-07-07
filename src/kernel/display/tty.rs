/**
 * TTY functions for displaying a console
 */

#[cfg(target_arch = "x86")]
pub fn hello_world() {
    super::super::arch::i386::display::vga_buffer::testfn();
}

#[cfg(target_arch = "x86")]
pub fn clear() {
    super::super::arch::i386::display::vga_buffer::clear_vga();
}