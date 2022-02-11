obj-m += $(MODULE_NAME).o
$(MODULE_NAME)-objs += $(MODULE_NAME).rust.o

.PHONY: build-rust

$(src)/target/$(RUST_TARGET)/release/lib$(MODULE_NAME).a: build-rust
	cd $(src); cargo b 

%.rust.o: target/$(RUST_TARGET)/release/lib%.a
	$(LD) -r -o $@ --whole-archive $<
