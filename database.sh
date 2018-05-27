#!/bin/bash

docker kill exchange-db
docker rm exchange-db
docker run --rm -d \
  --name exchange-db \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD=postgres \
  postgres
  
