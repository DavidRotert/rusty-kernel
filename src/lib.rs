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

pub static mut TTY: tty::TTYWriter = tty::TTYWriter {
    fg: tty::TTYColor::White,
    bg: tty::TTYColor::Black,
    line: 1,
    column: 1,
};

#[no_mangle]
pub extern "C" fn kernel_main() {
    /*tty::clear(tty::TTYColor::White, tty::TTYColor::Black);
    tty::enable_cursor();
    let mut tty_w = tty::TTYWriter::new(tty::TTYColor::White, tty::TTYColor::Black, 1, 1);
    tty_w.clear_screen();
    /*core::fmt::write(&mut tty_w, x);*/
    write!(tty_w, "T {} {}", 1, "Test2");
    //panic!("!!PANIK LEUTE!!");
    writeln!(tty_w, "Tatatataaa ...");
    let tty_w_addr = &mut tty_w as *mut tty::TTYWriter;
    unsafe {
        TTY_PTR = tty_w_addr;
    }
    unsafe {
        writeln!(tty_w, "Address of TTY writer: {:?}", TTY_PTR);
        writeln!(*TTY_PTR, "Tatatataaa ...");
    }
    call_2();*/
    unsafe {
        TTY.clear_screen();
        panic!("");
        TTY.print("Line 1");
        TTY.print("Hallo");
    }
}

/*fn call_2() {
    unsafe {
        writeln!(*TTY_PTR, "Call2 ...");
        TTY_PTR = &mut tty::TTYWriter::new(tty::TTYColor::White, tty::TTYColor::Black, 6, 8);
        TTY_2 = tty::TTYWriter::new(tty::TTYColor::White, tty::TTYColor::Black, 6, 8);
        writeln!(*TTY_PTR, "New TTY!!!");
        writeln!(*TTY_PTR, "Address of TTY writer: {:?}", TTY_PTR);
    }
}*/

/// This function is called on panic.
#[panic_handler]
fn on_panic(_info: &PanicInfo) -> ! {
    //let mut tty_w = tty::TTYWriter::new(tty::TTYColor::White, tty::TTYColor::Red, 1, 1);
    //tty_w.clear_screen();
    //write!(tty_w, "[panic] A kernel panic occured: {}", _info);
    unsafe {
        TTY.fg = tty::TTYColor::LightRed;
        TTY.bg = tty::TTYColor::Black;
        write!(TTY, "[panic] A kernel panic occured: {}", _info);
    }

    loop {}
}
