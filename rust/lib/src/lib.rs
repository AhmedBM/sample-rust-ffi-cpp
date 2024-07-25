use std::ffi::c_void;

#[no_mangle]
pub extern "C" fn send_vector_to_cpp(data: *mut c_void, size: usize) {
    let vec = vec![1, 2, 3, 4, 5];
    unsafe {
        std::ptr::copy_nonoverlapping(vec.as_ptr(), data as *mut i32, size);
    }
}