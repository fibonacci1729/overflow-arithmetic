# Build the overflow arithmetic library
cargo component build --release --manifest-path ../library/Cargo.toml 

# Build the example spin application
cargo component build --release

# Compose the example application with the overflow arithmetic library
wasm-tools compose ../target/wasm32-wasi/release/example.wasm -d ../target/wasm32-wasi/release/library.wasm -o service.wasm