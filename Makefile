src/bindings.rs: src/bindings.h
	bindgen $< -o $@
