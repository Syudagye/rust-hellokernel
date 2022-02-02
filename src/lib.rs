#![no_std]
#![feature(alloc_error_handler)]
#![feature(lang_items)]

mod bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

extern crate alloc;

use core::{panic::PanicInfo, alloc::GlobalAlloc, borrow::Borrow};

use alloc::vec::Vec;
use cty::{c_char, c_uint, c_void};

// Not sure what to do here
#[panic_handler]
fn rust_panic(_: &PanicInfo) -> ! {
    loop {}
}
#[alloc_error_handler]
fn rust_alloc_error(_: core::alloc::Layout) -> ! {
    loop {}
}
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        // Using the GFP_KERNEL flag for allocation.
        // The value is calculated like this (from the macros in kernel headers):
        // GFP_KERNEL = (GFP_RECLAIM | GFP_IO | GFP_FS)
        // GFP_RECLAIM = (GFP_DIRECT_RECLAIM | GFP_KSWAPD_RECLAIM)
        // GFP_IO = 0x40
        // GFP_FS = 0x80
        // GFP_DIRECT_RECLAIM = 0x400
        // GFP_KSWAPD_RECLAIM = 0x800
        // "|" is bitwise OR
        let gfp = 0x40 | 0x80 | 0x400 | 0x800;
        bindings::__kmalloc(layout.size() as u64, gfp) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        bindings::kfree(ptr as *const c_void);
    }
}

#[global_allocator]
static GLOBAL: KernelAllocator = KernelAllocator;

#[no_mangle]
pub extern "C" fn init_module() -> c_uint {
    let mut v: Vec<&str> = Vec::new();
    v.push("This is a string in a vector, which needs an allocator to work :)\n\0");
    unsafe {
        bindings::_printk("Hello Kernel from Rust :D !\n\0".as_ptr() as *const c_char);
        bindings::_printk(v[0].as_ptr() as *const c_char);
    }
    0
}

#[no_mangle]
pub extern "C" fn cleanup_module() {
    unsafe {
        bindings::_printk("See ya kernel :p !\n\0".as_ptr() as *const c_char);
    }
}
