#!/bin/bash

docker build -t devops_rust .

docker run -p 3000:4040 -p 8080:8080  --env-file=.env  devops_rust
