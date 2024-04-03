#![allow(non_camel_case_types)]

use std::{ffi::{c_int, c_ulong, c_ushort}, io::stdout, os::{fd::AsRawFd, raw::{c_uchar, c_uint}}, ptr::addr_of};

type tcflag_t = c_uint;
type cc_t = c_uchar;

const TCSAFLUSH: c_int = 2;

// constant only for linux
const TIOCGWINSZ: c_ulong = 0x5413;

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn cfmakeraw(termios_p: *mut termios);
    fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
    fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
}

#[repr(C)]
#[derive(Default, Debug)]
struct winsize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

#[derive(Debug)]
pub struct TermSize {
    pub rows: u16,
    pub cols: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_cc: [cc_t; 32],
}

impl termios {
    const fn default() -> Self {
        Self { c_iflag: 0, c_oflag: 0, c_cflag: 0, c_lflag: 0, c_cc: [0;32] }
    }
}

static mut ORIG_TERMIOS: termios = termios::default();

pub fn get_terminal_size() -> Option<TermSize> {
    let raw_stdout = stdout().as_raw_fd();
    let mut winsize = winsize::default();

    let res = unsafe { ioctl(raw_stdout, TIOCGWINSZ, &mut winsize) };
    
    if res == 0 {
        Some(TermSize { rows: winsize.ws_row, cols: winsize.ws_col })
    } else {
        None
    }
}

pub fn enable_raw_mode() {
    let raw_stdout = stdout().as_raw_fd();
    let mut raw = termios::default();

    let res = unsafe { tcgetattr(raw_stdout, &mut raw) };

    if res == -1 { panic!("tcgetattr failure"); }

    unsafe { ORIG_TERMIOS = raw };
    unsafe { cfmakeraw(&mut raw) };

    let res = unsafe { tcsetattr(raw_stdout, TCSAFLUSH, &raw) };

    if res == -1 { panic!("tcsetattr failure"); }
}

pub fn disable_raw_mode() {
    let raw_stdout = stdout().as_raw_fd();

    let res = unsafe { tcsetattr(raw_stdout, TCSAFLUSH, addr_of!(ORIG_TERMIOS)) };

    if res == -1 { panic!("tcsetattr failure"); }
}