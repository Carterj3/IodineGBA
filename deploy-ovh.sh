#! /bin/bash 
# CD to the directory of the script
cd "`dirname -- "$0"`"

# Build files
## WASM
( cd network-wasm && wasm-pack build --release --target web )
( cd state-wasm && wasm-pack build --release --target web )

## Server
cargo build --bin server --release --target x86_64-unknown-linux-musl

# Clear output files
rm -rf ./www/*

# Copy output files
mkdir -p ./www/

## Emulator
mkdir -p ./www/IodineGBA/
cp -r ./emulator/IodineGBA/* ./www/IodineGBA/

mkdir -p ./www/user_css/
cp -r ./emulator/user_css/* www/user_css

mkdir -p ./www/user_scripts/
cp -r ./emulator/user_scripts/* www/user_scripts

cp -r ./emulator/index.html www/index.html

## WASM
mkdir -p ./www/network/
cp -r ./network-wasm/pkg/* www/network


mkdir -p ./www/state/
cp -r ./state-wasm/pkg/* www/state

# Transfer
scp ./target/x86_64-unknown-linux-musl/release/server ovh:~/mengs/mengs
scp -r ./www/* ovh:~/mengs/www