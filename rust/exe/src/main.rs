extern crate libc;
// extern crate rust_ffi_lib;
use libc::c_char;
use std::ffi::CStr;
use std::str;
use std::ffi::c_void;
// use rust_ffi_lib::send_vector_to_cpp;

#[link(name = "HelloFromCPPLib")]
// #[link(name = "rust_ffi_lib")]
#[link(name = "stdc++")]
extern "C" {
    fn get_string()  -> *mut c_char;
    fn get_integer() -> i32;
    fn get_vector() -> *const std::ffi::c_void;
    fn get_vector_data(vec: *const std::ffi::c_void, data: *mut *const i32, size: *mut usize);
    // fn get_vector_from_rust(vec: *const i32, len: usize);
    fn deallocate_res(s: *mut c_void);
}

pub fn get_string_from_cpp() -> String {
    let c_str: *mut c_char = unsafe { get_string() };
    let c_new_str = unsafe { CStr::from_ptr(c_str) };
    let str_slice: &str = c_new_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();
    unsafe {deallocate_res(c_str as *mut c_void)};
    str_buf
}

pub fn get_vector_from_cpp() -> Vec<i32> {
    let vec: *const std::ffi::c_void = unsafe { get_vector() };
    let mut data: *const i32 = std::ptr::null();
    let mut size: usize = 0;

    unsafe {
        get_vector_data(vec, &mut data, &mut size);
        deallocate_res(vec as *mut c_void);
        std::slice::from_raw_parts(data, size).to_vec()
    }
}

// fn send_vector_to_cpp(vec: Vec<i32>) {
//     let len = vec.len();
//     let ptr = vec.as_ptr();
//     std::mem::forget(vec); // Prevent Rust from deallocating the vector
//     unsafe {
//         get_vector_from_rust(ptr, len);
//     }
// }

fn main() {
    println!("The integer from C is: {}", unsafe{ get_integer() });
    println!("The string from C is: {}", get_string_from_cpp());
    println!("The vector from C is: {:?}", get_vector_from_cpp());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rusttest_get_string_from_cpp() {
        assert_eq!(get_string_from_cpp(), "Hello from C++!");
    }

    #[test]
    fn rusttest_get_integer() {
        assert_eq!(unsafe{get_integer()}, 7);
    }

    #[test]
    fn rusttest_get_vector_from_cpp() {
        assert_eq!(get_vector_from_cpp(), vec![1, 2, 3, 4, 5]);
    }
}
