/**
 * VGA buffer implementation
 */

static mut VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

pub const VGA_COLUMNS: u32 = 80;
pub const VGA_LINES: u32 = 25;

pub enum VgaColor {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White
}

/**
 * The VGA color code is an unsigned 1 byte int. It can represent 16 colors (foreground)
 * and 16 colors for the background. The first 4 bit represent the background color,
 * the last 4 bit the foreground color.
 * 
 */
pub fn calc_color_code(fg_color: VgaColor, bg_color: VgaColor) -> u8 {
    // User OR to binary add bits
    (bg_color as u8) << 4 | (fg_color as u8)
}

pub fn calc_vga_text_entry(ch: u8, fg_color: VgaColor, bg_color: VgaColor) -> u16 {
    // User OR to binary add bits
    ((calc_color_code(fg_color, bg_color) as u16) << 8) | ch as u16
}

pub fn clear_vga() {
    let size = VGA_COLUMNS * VGA_LINES;
    for i in 0..size {
        put_char(i as isize, 0, VgaColor::White, VgaColor::Black);
    }
}

pub fn put_char(buf_pos: isize, ch: u8, fg_color: VgaColor, bg_color: VgaColor) {
    unsafe {
        *VGA_BUFFER.offset(buf_pos) = calc_vga_text_entry(ch, fg_color, bg_color);
    }
}

pub fn testfn() {
    clear_vga();
    /*for i in 0..255 {
        put_char(i as isize, i, VgaColor::White, VgaColor::Black);
    }
    for (i, &byte) in b"Hello World!".iter().enumerate() {
        put_char(i as isize, byte, VgaColor::White, VgaColor::Black);
    }*/
}
