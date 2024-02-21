#!/bin/sh

cargo build --release --target=x86_64-unknown-linux-musl || exit
mkdir -p bin
cp -v target/x86_64-unknown-linux-musl/release/rookrant_serv bin
