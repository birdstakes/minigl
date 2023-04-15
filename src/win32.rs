#![allow(clippy::upper_case_acronyms)]

use std::ffi::c_void;

pub type BOOL = bool;
pub type HDC = u32;
pub type HGLRC = u32;
pub type HWND = *const c_void;
pub type LONG = i32;
pub type LPCSTR = *const u8;
pub type PROC = Option<extern "system" fn() -> isize>;

#[repr(C)]
#[derive(Default)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

// TODO use real types?
#[repr(C)]
#[derive(Default)]
pub struct BITMAPINFOHEADER {
    pub size: u32,
    pub width: i32,
    pub height: i32,
    pub planes: u16,
    pub bit_count: u16,
    pub compression: u32,
    pub size_image: u32,
    pub x_pels_per_meter: i32,
    pub y_pels_per_meter: i32,
    pub clr_used: u32,
    pub clr_important: u32,
}

pub const BI_RGB: u32 = 0;
pub const DIB_RGB_COLORS: u32 = 0;
pub const SRCCOPY: u32 = 0x00CC0020;

#[link(name = "gdi32")]
extern "system" {
    pub fn WindowFromDC(hdc: HDC) -> HWND;
    pub fn GetClientRect(hwnd: HWND, rect: *mut RECT) -> BOOL;

    pub fn StretchDIBits(
        hdc: HDC,
        x_dest: i32,
        y_dest: i32,
        dest_width: i32,
        dest_height: i32,
        x_src: i32,
        y_src: i32,
        src_width: i32,
        src_height: i32,
        bits: *const c_void,
        bmi: *const BITMAPINFOHEADER,
        usage: u32,
        rop: u32,
    );
}
