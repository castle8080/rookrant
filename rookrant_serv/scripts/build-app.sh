#!/bin/sh

cargo build --release || exit
mkdir -p bin
cp -v target/release/rookrant_serv bin
