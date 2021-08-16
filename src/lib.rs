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
    tty::clear(tty::TTYColor::White, tty::TTYColor::Black);
    /*for i in 0..255 {
        tty::put_char(i as usize, i, tty::TTYColor::White, tty::TTYColor::Black);
    }
    for (i, &byte) in b"Hello World!".iter().enumerate() {
        tty::put_char(i as usize, byte, tty::TTYColor::White, tty::TTYColor::Black);
    }

    tty::update_cursor_pos(3, 1);
    tty::enable_cursor();*/
    
    tty::enable_cursor();
    tty::update_cursor_pos(4, 2);
    let mut tty_w = tty::TTYWriter{ fg: tty::TTYColor::White, bg: tty::TTYColor::Black, line: 1, column: 1 };
    tty_w.print("Hallo welt\nabc");
    use core::fmt::Write;
    //write!(tty_w, "Haa {}", format_args!("{} {}", 1, "ttt"));
    //writeln!(tty_w, "Tatata {}", "20");
    tty_w.write_str("Ahllo\n");
    tty_w.print("Hallo welt\nabc");
    
    let mut tty_w2 = tty::TTYWriter{ fg: tty::TTYColor::White, bg: tty::TTYColor::Black, line: 1, column: 1 };
    tty_w2.print("Hallo welt\nabc");
    

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
