# Rust Kernel Driver

A little kernel driver example written in rust.

## Building

You will need:
  - `rustup` to download the latest nightly rust toolchain
  - `linux-headers` for the kernel headers
  - `llvm` (which should come with rust i think) and `clang`

just build with 
```
make
```
and you will get a `hellokernel.ko` directly on the root of the project.

the insert the module (must have root privileges):
```
insmod hellokernel.ko
```

and see the messages with:
```
dmesg
```

or, to get the messages in real time:
```
dmesg -W
```

## Credits

Mostly inspired from the [linux-kernel-module-rust](https://github.com/fishinabarrel/linux-kernel-module-rust) repo, which gives a lot of informations, but is no longer working.
