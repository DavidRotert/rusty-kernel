/**
 * VGA buffer implementation
 */

static HELLO: &[u8] = b"Hello World!";

static mut VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

fn vga_put_char(buf_pos: isize, ch: u8) {
    unsafe {
        *VGA_BUFFER.offset(buf_pos * 2) = ch;
        *VGA_BUFFER.offset(buf_pos * 2 + 1) = 0xb;
    }
}

pub fn testfn() {
    for (i, &byte) in HELLO.iter().enumerate() {
        vga_put_char(i as isize, byte);
    }
}
