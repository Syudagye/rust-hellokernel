obj-m += hellokernel.o
hellokernel-objs += module-info.o hellokernel.rust.o

.PHONY: build-rust

$(src)/target/$(RUST_TARGET)/release/libhellokernel.a: build-rust
	cd $(src); cargo b --release -Z build-std=core --target=$(RUST_TARGET)

%.rust.o: target/$(RUST_TARGET)/release/lib%.a
	$(LD) -r -o $@ --whole-archive $<
