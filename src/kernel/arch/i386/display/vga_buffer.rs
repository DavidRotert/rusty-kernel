/**
 * VGA buffer implementation
 */
 
use super::super::ports::port_out;

static mut VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

pub const VGA_COLUMNS: u32 = 80;
pub const VGA_LINES: u32 = 25;

#[derive(Copy, Clone)]
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
    // User OR to add bits into binary representation
    ((bg_color as u8) << 4) | (fg_color as u8)
}

pub fn calc_vga_txt_entry(ch: u8, fg_color: VgaColor, bg_color: VgaColor) -> u16 {
    // User OR to add bits into binary representation
    ((calc_color_code(fg_color, bg_color) as u16) << 8) | ch as u16
}

pub fn clear_vga(bg_color: VgaColor) {
    let size = VGA_COLUMNS * VGA_LINES;
    for i in 0..size {
        //put_char(i as isize, 0, bg_color, bg_color);
        put_char(i as isize, 0, bg_color, bg_color);
    }
}

pub fn put_char(buf_pos: isize, ch: u8, fg_color: VgaColor, bg_color: VgaColor) {
    if buf_pos < 0 || buf_pos as u32 >= (VGA_LINES * VGA_COLUMNS) {
        panic!("[ <kernel/arch/i386/vga_buffer/display/vga_buffer.rs:put_char> ] Invalid buffer position: must be betwenn 0 and max charcter size");
    }
    
    unsafe {
        *VGA_BUFFER.offset(buf_pos) = calc_vga_txt_entry(ch, fg_color, bg_color);
    }
}

pub fn test() {
    clear_vga(VgaColor::Black);
    for i in 0..255 {
        put_char(i as isize, i, VgaColor::White, VgaColor::Black);
    }
    for (i, &byte) in b"Hello World!".iter().enumerate() {
        put_char(i as isize, byte, VgaColor::White, VgaColor::Black);
    }
    
    port_out(0x3D4, 0x0A);
	port_out(0x3D5, 0);

	port_out(0x3D4, 0x0B);
	port_out(0x3D5, 15);
	
	port_out(0x3D4, 0x0F);
	port_out(0x3D5, (1 & 0xFF));
	port_out(0x3D4, 0x0E);
 	port_out(0x3D5, ((1u16 >> 8) & 0xFF) as u8);
}
