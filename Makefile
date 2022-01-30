export KDIR ?= /lib/modules/$(shell uname -r)/build
export RUST_TARGET ?= x86_64-unknown-none-linuxkernel

all:
	touch .hellokernel.rust.o.cmd # fixes wierd bug when compiling
	make -C $(KDIR) M=$(PWD) modules

clean:
	make -C $(KDIR) M=$(PWD) clean
	cargo clean
