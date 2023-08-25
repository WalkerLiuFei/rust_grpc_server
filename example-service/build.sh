#!/bin/bash

cargo update -p example-service --aggressive
commit_hash=$(git log -1 --pretty=format:"%h")
echo $commit_hash


docker build --build-arg COMMIT_HASH=$commit_hash -t example-service:$commit_hash .
