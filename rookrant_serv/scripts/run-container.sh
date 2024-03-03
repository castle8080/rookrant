#!/bin/sh
exec docker run -it --rm -p 127.0.0.1:3000:3000 --name rookrant_serv rookrant_serv \
    scripts/run-app.sh \
    --tls=false --listen=0.0.0.0:3000 \
    --environment=dev