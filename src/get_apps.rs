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


/*
// (full, base)
pub fn main(order: &Vec<&str>) -> Vec<(String, String)> {
    // the part where I request windows to give me a list of all tasks
    // this is also the slowest part
    // and it only works on windows (i think ??)
    let var = std::process::Command::new("tasklist")
        .arg("/v")
        .arg("/FO")
        .arg("CSV")
        .output()
        .expect("Failed to get the running windows")
        .stdout;
    let s = String::from_utf8_lossy(&var);
    let mut data = Vec::new();
    for i in s.lines() {
        let (full, base) = parse_line(&order, i);
        if full.len() != 0 {
            data.push((full, base));
        }
    }
    data
}


// (full name, base name)
fn parse_line(order: &Vec<&str>, line: &str) -> (String, String) {
    let mut string = String::new();
    let mut output_full = String::new();
    let mut output_base = String::new();
    let mut escape = false;
    let lowercase_line = line.to_lowercase();
    for i in line.trim().chars() {
        match i {
            '"' => {escape = !escape;},
            ',' if !escape => {string.clear();},
            x => {string.push(x);}
        }
    }
    for i in order {
        let mut contains = true;
        for j in split(i) {
            if let Some(0) = j.find("-") {
                if lowercase_line.contains(j[1..].trim()) {
                    contains = false;
                    break;
                }
            } else if Some(0) == j.find("\\") && !lowercase_line.contains(j[1..].trim()){
                contains = false;
                break;
            } else if !lowercase_line.contains(j.trim()) {
                contains = false;
                break;
            }
        }
        if contains {
            output_full.push_str(&string);
            output_base.push_str(i);
            break;
        }
    }
    (output_full.replace("N/A", ""), output_base)
}


fn split(text: &str) -> Vec<String> {
    let mut output = Vec::new();
    let mut escape = false;
    let mut buffer = String::new();
    for i in text.chars() {
        match i {
            '\\' if !escape => {
                escape = true
            },
            ',' if !escape => {
                output.push(buffer.to_lowercase().trim().to_string());
                buffer.clear();
                escape = false
            },
            '[' if !escape => (),
            ']' if !escape => (),
            x => {
                buffer.push(x);
                escape = false
            }
        }
    }
    output.push(buffer.to_lowercase().trim().to_string());
    output
}

 */