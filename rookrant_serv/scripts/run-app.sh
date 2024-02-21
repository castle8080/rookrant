#!/bin/sh

export RUST_LOG=information,tower_http=debug,rookrant_serv=trace
export RUST_BACKTRACE=1

# Make sure the cwd is in the right directory.
cd $(dirname $0)
cd ..

bin/rookrant_serv "$@"