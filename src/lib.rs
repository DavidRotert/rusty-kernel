#![no_std]
#![allow(unused)]
#![feature(asm)]

/**
 * Kernel entrypoint
 */
mod kernel;

use core::fmt::Write;
use core::panic::PanicInfo;

use kernel::display::tty;

#[no_mangle]
pub extern "C" fn kernel_main() {
    tty::clear(tty::TTYColor::White, tty::TTYColor::Black);
    
    tty::enable_cursor();
    let mut tty_w = tty::TTYWriter{ fg: tty::TTYColor::White, bg: tty::TTYColor::Black, line: 1, column: 1 };
    tty_w.clear_screen();
    
    /*core::fmt::write(&mut tty_w, x);*/
    writeln!(tty_w, "T {}", 1);
    //panic!("PANIC!!");
    writeln!(tty_w, "Tatatata");
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut tty_w = tty::TTYWriter{ fg: tty::TTYColor::White, bg: tty::TTYColor::Red, line: 1, column: 1 };
    tty_w.clear_screen();
    write!(tty_w, "[panic] A kernel panic occured: {}", _info);
    
    loop {};
}
