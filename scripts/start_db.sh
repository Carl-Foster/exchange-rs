#!/bin/bash
docker run --rm -d \
    --name exchange-db \
    -e POSTGRES_PASSWORD=postgres \
    postgres