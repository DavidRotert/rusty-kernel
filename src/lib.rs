#![no_std]
#![allow(unused)]
#![feature(asm)]

/**
 * Kernel entrypoint
 */
mod kernel;

use core::panic::PanicInfo;

use kernel::display::tty;

#[no_mangle]
pub extern "C" fn kernel_main() {
    tty::clear(tty::TTYColor::White, tty::TTYColor::Black);
    
    tty::enable_cursor();
    tty::update_cursor_pos(4, 2);
    let mut tty_w = tty::TTYWriter{ fg: tty::TTYColor::White, bg: tty::TTYColor::Black, line: 1, column: 1 };
    tty_w.println("Hallö welt\nabc");
    use core::fmt::Write;
    write!(tty_w, "Tatata {}", 20).unwrap();
    

    //loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    tty::clear_panic();
    for (i, &byte) in b"A kernel panic occured!".iter().enumerate() {
        tty::put_char(i as usize, byte, tty::TTYColor::White, tty::TTYColor::Black);
    }

    loop {}
}
