# raylib bindings for rust

Feaures: 
- [ ] all raylib core functions
- [ ] support for rmath
- [ ] rlgl.h
- [ ] rcamera.h
- [ ] raygui
- [ ] egui

# sample usage

for general sample usage see the files in the `examples` folder.

### Cargo.toml


### main function

# build wasm

cd wasm
./emsdk activate latest
source emsdk_env.sh

EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1" PATH=$PATH:$(pwd)/emsdk/upstream/emscripten/ cargo build --example raygui --target wasm32-unknown-emscripten --release --features raygui

# TODO

- [ ] make raymath optional
- [ ] include math library

