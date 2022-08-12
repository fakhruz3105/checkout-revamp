# Checkout Revamp with Tauri (RUST)

## Requirements
- diesel-cli (For schema generation) (require `libsqlite3-dev` package in linux)  
  1. `cargo install diesel_cli`
  2. `cd src-tauri`
  3. `diesel setup`
  4. `diesel migration run`

## Start application
In root folder run command `npm run tauri dev`

## Build application
1. `npm run build`
2. `cd src-tauri`
3. `cargo build --release`
4. `cd target/release`