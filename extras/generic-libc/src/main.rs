extern crate libc;

use std::mem;

fn main() {
    unsafe {
        let my_num: *mut i32 = libc::malloc(mem::size_of::<i32>()) as *mut i32;
        if my_num.is_null() {
            panic!("failed to allocate memory");
        }
        *my_num = 62;

        // Rust strings are not the same as C strings
        libc::printf("My num %d\n\0".as_ptr() as *const i8, *my_num);

        libc::free(my_num as *mut libc::c_void);
    }

}