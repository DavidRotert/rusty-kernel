#![no_std]
#![feature(asm)]

/**
 * Kernel entrypoint
 */

mod kernel;

use core::panic::PanicInfo;

use kernel::display::tty;

#[no_mangle]
pub extern "C" fn kernel_main() {
    tty::clear();
    for i in 0..255 {
        tty::put_char(i as usize, i);
    }
    for (i, &byte) in b"Hello World!".iter().enumerate() {
        tty::put_char(i as usize, byte);
    }
    
    tty::update_cursor_pos(3, 1);
    tty::enable_cursor();
    
    //loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    tty::clear_panic();
    for (i, &byte) in b"A kernel panic occured!".iter().enumerate() {
        tty::put_char(i as usize, byte);
    }
    
    loop {}
}
