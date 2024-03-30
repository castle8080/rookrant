@echo off

cargo build --release && (
    echo Cargo build succeeded.
) || (
    echo Cargo build failed. Exiting...
    exit /b 1
)

mkdir bin
copy target\release\rookrant_serv.exe bin