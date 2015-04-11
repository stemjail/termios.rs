BINDGEN = env LD_PRELOAD=/usr/lib/llvm-3.6/lib/libclang-3.6.so LD_LIBRARY_PATH=../rust-bindgen/target/release/ ../rust-bindgen/target/release/bindgen

src/bindings.rs: src/bindings.h
	$(BINDGEN) $< -o $@
