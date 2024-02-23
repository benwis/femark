# clang environment variables for web assembly
#export AR := "llvm-ar"
export CC := "/opt/wasi-sdk/bin/clang"
export CXX := "/opt/wasi-sdk/bin/clang"
wasm:
    cargo build --target=wasm32-wasi

wasm-release:
    cargo build --release --target=wasm32-wasi
