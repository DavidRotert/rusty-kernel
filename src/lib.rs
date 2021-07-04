#![no_std]
#![no_main]

/**
 * Kernel entrypoint
 */

use core::panic::PanicInfo;

mod kernel;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    kernel::display::tty::hello_world();

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}