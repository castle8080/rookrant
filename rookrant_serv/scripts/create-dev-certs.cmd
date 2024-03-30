@echo off

rem go get openssl
rem https://slproweb.com/products/Win32OpenSSL.html

set PATH=C:\Program Files\OpenSSL-Win64\bin;%PATH%

mkdir etc\certs
openssl req -newkey rsa:2048 -nodes -keyout etc\certs\key.pem -x509 -days 365 -out etc\certs\certificate.pem