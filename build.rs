use std::{env, path::PathBuf};

use bindgen;

const ALLOWED_FUNCTIONS: &[&str] = &["_printk", "__kmalloc", "kfree"];

fn main() {
    let kernel_build_dir = env::var("KDIR").expect("KDIR env variable not found");
    let cflags = env::var("c_flags").expect("You need to build for a Kbuild");
    let cflags = cflags.split(" ").collect::<Vec<&str>>();

    // formatting all -I and -include flags with absolute paths
    let mut processed_flags = cflags
        .clone()
        .into_iter()
        .filter(|e| e.starts_with("-I"))
        .map(|e| e.replace("./", &format!("{}/", kernel_build_dir)))
        .collect::<Vec<String>>();
    for i in 0..cflags.len() {
        if cflags[i] == "-include" {
            // Wierd thing, the include flag should be separated from the file path with a space, but it causes an error here
            processed_flags.push(cflags[i+1].replace("./", &format!("-include{}/", kernel_build_dir)));
        }
    }
    // Mandatory to build kernel headers
    processed_flags.push("-D__KERNEL__".to_string());

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .use_core()
        .ctypes_prefix("cty")
        .clang_args(processed_flags.into_iter());

    for i in ALLOWED_FUNCTIONS {
        builder = builder.allowlist_function(i);
    }

    let builder = builder
        .generate()
        .expect("Couldn't generate kernel bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    builder
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings.");
}
