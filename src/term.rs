use std::{ffi::{c_int, c_ulong, c_ushort}, os::fd::AsRawFd};

use crate::TermSize;

// constant only for linux
pub const TIOCGWINSZ: c_ulong = 0x5413;

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
}

#[repr(C)]
#[derive(Default, Debug)]
pub struct Winsize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

pub fn get_terminal_size() -> Option<TermSize> {
    let mut winsize = Winsize::default();

    let res = unsafe { ioctl(std::io::stdout().as_raw_fd(), TIOCGWINSZ, &mut winsize) };
    
    if res == 0 {
        Some(TermSize { rows: winsize.ws_row, cols: winsize.ws_col })
    } else {
        None
    }
}