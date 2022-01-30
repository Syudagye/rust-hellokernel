#![no_std]

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use core::panic::PanicInfo;

use cty::{c_char, c_uint};

// Not sure what to do here
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn init_module() -> c_uint {
    unsafe {
        bindings::_printk("Hello Kernel from Rust :D !\n\0".as_ptr() as *const c_char);
    }
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    unsafe {
        bindings::_printk("See ya kernel :p !\n\0".as_ptr() as *const c_char);
    }
}
