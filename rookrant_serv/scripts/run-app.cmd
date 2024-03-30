@echo off
set RUST_LOG=information,tower_http=debug,rookrant_serv=trace
set RUST_BACKTRACE=1

REM Make sure the cwd is in the right directory.
cd /d %~dp0..
bin\rookrant_serv %*