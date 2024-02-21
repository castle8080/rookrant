#!/bin/sh

az login || exit
az acr login --name rookcr || exit

docker tag rookrant_serv:latest rookcr.azurecr.io/rookrant_serv:latest || exit
docker push rookcr.azurecr.io/rookrant_serv:latest || exit