pub fn port_out(port: u16, val: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") val,
        );
    }
}
