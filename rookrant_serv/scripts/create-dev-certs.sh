#!/bin/sh

mkdir -p etc/certs
openssl req -newkey rsa:2048 -nodes -keyout etc/certs/key.pem -x509 -days 365 -out etc/certs/certificate.pem