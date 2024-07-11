extern crate libc;
use libc::c_char;
use std::ffi::CStr;
use std::str;

extern "C" {
    fn get_string()  -> *mut c_char;
    fn get_integer() -> i32;
    fn deallocate_string(s: *mut c_char);
}

pub fn get_string_from_cpp() -> String {
    let c_str: *mut c_char = unsafe { get_string() };
    let c_new_str = unsafe { CStr::from_ptr(c_str) };
    let str_slice: &str = c_new_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();
    unsafe {deallocate_string(c_str)};
    str_buf
}

fn main() {
    println!("The integer from C is: {}", unsafe{ get_integer() });
    println!("The string from C is: {}", get_string_from_cpp());
}

// TODO: Figure out how to get CTest to work with Rust Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_string_from_cpp() {
        assert_eq!(get_string_from_cpp(), "Hello from C++!");
    }
}
