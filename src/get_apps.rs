use std::ffi::OsString;
use std::slice;
use std::os::windows::prelude::*;
use fancy_regex::Regex;

use winapi::shared::{minwindef::LPARAM, windef::HWND__};
use winapi::um::winuser::{EnumWindows, GetWindowTextW, IsWindowVisible};

static mut WINDOWS: Option<Vec<String>> = None;


fn find_windows() {
    extern "system" fn enum_windows_proc(hwnd: *mut HWND__, _l_param: LPARAM) -> i32 {
        let mut name: Vec<u16> = Vec::with_capacity(1024);
        let name_ptr = name.as_mut_ptr();

        unsafe {
            if IsWindowVisible(hwnd) == 0 {
                return 1;
            }
            let name_length = GetWindowTextW(hwnd, name_ptr, 1024);
            let name_slice = slice::from_raw_parts(name_ptr, name_length as usize);
            match OsString::from_wide(name_slice).into_string() {
                Ok(s) if s.len() > 0 => {
                    // println!("debug: {:?}", s);
                    WINDOWS.as_mut().unwrap().push(s);
                },
                _ => ()
            }
        }

        1
    }

    unsafe {
        EnumWindows(Some(enum_windows_proc), 0);
    }
}


pub fn main(order: &Vec<Regex>) -> Option<(String, String)> {
    unsafe {
        WINDOWS = Some(Vec::new());
        // find_windows has tons of unsafe calls
        find_windows();
        for i in order {
            for (index, text) in WINDOWS.as_mut().unwrap().iter().enumerate() {
                match i.is_match(text) {
                    Ok(true) => return Some((i.as_str().to_string(), WINDOWS.as_mut().unwrap().remove(index))),
                    _ => ()
                }
            }
        }
    }
    None
}
