#![no_std]

/**
 * Kernel entrypoint
 */

mod kernel;

use core::panic::PanicInfo;

use kernel::display::tty;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    tty::clear();
    tty::test();

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    tty::clear_panic();
    
    loop {}
}