#! /bin/bash 
cd "`dirname -- "$0"`"

(cd .. && ./build-wasm)

cargo build --release --target x86_64-unknown-linux-musl \
 && scp ../target/x86_64-unknown-linux-musl/release/server ovh:~/mengs/mengs