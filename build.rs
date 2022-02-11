use std::{env, path::PathBuf, process::Command};

use bindgen;

const ALLOWED_FUNCTIONS: &[&str] = &["_printk", "__kmalloc", "kfree"];

fn main() {
    let kernel_build_dir = format!(
        "/lib/modules/{}/build",
        String::from_utf8(Command::new("uname").arg("-r").output().unwrap().stdout)
            .unwrap()
            .replace("\n", "")
    );
    // Flags taken from the kernel headers makefiles
    let cflags = [
        &format!("-I{}/arch/x86/include", kernel_build_dir),
        &format!("-I{}/arch/x86/include/generated", kernel_build_dir),
        &format!("-I{}/include", kernel_build_dir),
        &format!("-I{}/arch/x86/include/uapi", kernel_build_dir),
        &format!("-I{}/arch/x86/include/generated/uapi", kernel_build_dir),
        &format!("-I{}/include/uapi", kernel_build_dir),
        &format!("-I{}/include/generated/uapi", kernel_build_dir),
        &format!("-include{}/include/linux/compiler-version.h", kernel_build_dir),
        &format!("-include{}/include/linux/kconfig.h", kernel_build_dir),
        &format!("-include{}/include/linux/compiler_types.h", kernel_build_dir),
        "-D__KERNEL__"
    ];

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .use_core()
        .ctypes_prefix("cty")
        .clang_args(cflags.iter());

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
