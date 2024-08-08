WASM_TARGET=wasm32-unknown-unknown

all: extensions_rs.wat extensions.wat
	mv extensions.wasm ..
	mv extensions_rs.wasm ..

extensions.wasm: c/test.c c/extension.c
	emcc $^ -o $@ -O3 -s WASM=1 -s SIDE_MODULE=1
	wasm-opt -Oz $@ -o $@
	wasm-strip $@

extensions.wat: extensions.wasm
	wasm2wat $< -o $@

extensions_rs.wasm:
	cd rust && cargo build --release --target $(WASM_TARGET)
	cp rust/target/$(WASM_TARGET)/release/rust_extension.wasm $@
	wasm-opt -Oz $@ -o $@
	wasm-strip $@

extensions_rs.wat: extensions_rs.wasm
	wasm2wat $< -o $@

clean:
	cd rust && cargo clean
	rm ../extensions.wasm extensions.wat ../extensions_rs.wasm extensions_rs.wat
