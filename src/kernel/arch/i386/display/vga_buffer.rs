/**
 * VGA buffer implementation
 */
use super::super::ports::port_out;

static mut VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

pub const VGA_COLUMNS: u32 = 80;
pub const VGA_LINES: u32 = 25;

#[derive(Copy, Clone)]
pub enum VGAColor {
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
    White,
}

/**
 * The VGA color code is an unsigned 1 byte int. It can represent 16 colors (foreground)
 * and 16 colors for the background. The first 4 bit represent the background color,
 * the last 4 bit the foreground color.
 *
 */
pub fn calc_color_code(fg_color: VGAColor, bg_color: VGAColor) -> u8 {
    // Uses bitwise OR to add bits
    ((bg_color as u8) << 4) | (fg_color as u8)
}

pub fn calc_vga_txt_entry(ch: u8, fg_color: VGAColor, bg_color: VGAColor) -> u16 {
    // Uses bitwise OR to add bits
    ((calc_color_code(fg_color, bg_color) as u16) << 8) | ch as u16
}

pub fn clear_vga(fg_color: VGAColor, bg_color: VGAColor) {
    let size = VGA_COLUMNS * VGA_LINES;
    for i in 0..size {
        put_char(i as usize, 0, fg_color, bg_color);
    }
}

pub fn put_char(buf_pos: usize, ch: u8, fg_color: VGAColor, bg_color: VGAColor) {
    if buf_pos as u32 >= (VGA_LINES * VGA_COLUMNS) {
        panic!("Invalid buffer position: must be between 0 and max buffer size");
    }

    unsafe {
        *VGA_BUFFER.offset(buf_pos as isize) = calc_vga_txt_entry(ch, fg_color, bg_color);
    }
}

pub fn enable_cursor(start: u8, stop: u8) {
    // begin cursor size
    port_out(0x3D4, 0x0A);
    port_out(0x3D5, start);

    // end cursor size
    port_out(0x3D4, 0x0B);
    port_out(0x3D5, stop);
}

pub fn update_cursor_pos(col: u32, line: u32) {
    if col == 0 || line == 0 {
        panic!("Column and line have to be > 0");
    }
    
    let pos = (line - 1) * VGA_COLUMNS + (col - 1);

    port_out(0x3D4, 0x0F);
    port_out(0x3D5, (pos & 0xFF) as u8);

    port_out(0x3D4, 0x0E);
    port_out(0x3D5, ((pos as u16 >> 8) & 0xFF) as u8);
}

pub fn test() {
    clear_vga(VGAColor::White, VGAColor::Black);
    for i in 0..255 {
        put_char(i as usize, i, VGAColor::White, VGAColor::Black);
    }
    for (i, &byte) in b"Hello World!".iter().enumerate() {
        put_char(i as usize, byte, VGAColor::White, VGAColor::Black);
    }

    update_cursor_pos(3, 1);
}
