/**
 * TTY functions for displaying a console
 */
#[cfg(target_arch = "x86")]
use super::super::arch::i386::display::vga_buffer;

use core::fmt;

#[derive(Copy, Clone)]
pub enum TTYColor {
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

pub struct TTYWriter {
    pub fg: TTYColor,
    pub bg: TTYColor,
    pub line: u32,
    pub column: u32
}

impl TTYWriter {
    pub fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
        update_cursor_pos(self.column, self.line);
    }
    
    pub fn inc_column(&mut self) {
        let new_col = self.column + 1;
        if new_col > get_columns() {
            self.new_line();
        } else {
            self.column = new_col;
            update_cursor_pos(self.column, self.line);
        }
    }
    
    pub fn get_buf_pos(&mut self) -> usize {
        (self.line - 1) as usize * get_columns() as usize + (self.column - 1) as usize
    }
    
    pub fn print(&mut self, s: &str) {
        for b in s.bytes() {
            if b == '\n' as u8 {
                self.new_line();
            } else if b < 0x20 || b > 0x7f {
                put_char(self.get_buf_pos(), 0xfe, self.fg, self.bg);
                self.inc_column();
            } else {
                put_char(self.get_buf_pos(), b, self.fg, self.bg);
                self.inc_column();
            }
        }
    }
    
    pub fn println(&mut self, s: &str) {
        self.print(s);
        self.print("\n");
    }
}

impl fmt::Write for TTYWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        
        Ok(())
    }
}

#[cfg(target_arch = "x86")]
fn tty_to_vga_color(tty_color: TTYColor) -> vga_buffer::VGAColor {
    match tty_color {
        TTYColor::Black => vga_buffer::VGAColor::Black,
        TTYColor::Blue => vga_buffer::VGAColor::Blue,
        TTYColor::Green => vga_buffer::VGAColor::Green,
        TTYColor::Cyan => vga_buffer::VGAColor::Cyan,
        TTYColor::Red => vga_buffer::VGAColor::Red,
        TTYColor::Magenta => vga_buffer::VGAColor::Magenta,
        TTYColor::Brown => vga_buffer::VGAColor::Brown,
        TTYColor::LightGray => vga_buffer::VGAColor::LightGray,
        TTYColor::DarkGray => vga_buffer::VGAColor::DarkGray,
        TTYColor::LightBlue => vga_buffer::VGAColor::LightBlue,
        TTYColor::LightGreen => vga_buffer::VGAColor::LightGreen,
        TTYColor::LightCyan => vga_buffer::VGAColor::LightCyan,
        TTYColor::LightRed => vga_buffer::VGAColor::LightRed,
        TTYColor::Pink => vga_buffer::VGAColor::Pink,
        TTYColor::Yellow => vga_buffer::VGAColor::Yellow,
        TTYColor::White => vga_buffer::VGAColor::White,
    }
}

#[cfg(target_arch = "x86")]
pub fn get_lines() -> u32 {
    vga_buffer::VGA_LINES
}

#[cfg(target_arch = "x86")]
pub fn get_columns() -> u32 {
    vga_buffer::VGA_COLUMNS
}

#[cfg(target_arch = "x86")]
pub fn clear(fg: TTYColor, bg: TTYColor) {
    vga_buffer::clear_vga(tty_to_vga_color(fg), tty_to_vga_color(bg));
}

#[cfg(target_arch = "x86")]
pub fn clear_panic() {
    vga_buffer::clear_vga(tty_to_vga_color(TTYColor::White), tty_to_vga_color(TTYColor::Red));
}

#[cfg(target_arch = "x86")]
pub fn test() {
    vga_buffer::test();
}

#[cfg(target_arch = "x86")]
pub fn enable_cursor() {
    vga_buffer::enable_cursor(0, 15);
}

#[cfg(target_arch = "x86")]
pub fn put_char(buf_pos: usize, ch: u8, fg: TTYColor, bg: TTYColor) {
    vga_buffer::put_char(
        buf_pos,
        ch,
        tty_to_vga_color(fg),
        tty_to_vga_color(bg),
    );
}

#[cfg(target_arch = "x86")]
pub fn update_cursor_pos(col: u32, line: u32) {
    vga_buffer::update_cursor_pos(col, line);
}
