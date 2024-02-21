#!/bin/sh

export RUST_LOG=information,tower_http=debug,rookrant_serv=trace
export RUST_BACKTRACE=1

exec cargo run "$@"