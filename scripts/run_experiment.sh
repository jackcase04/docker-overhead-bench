#!/bin/bash

# CONFIG 1: No container
for CONCURRENCY in 1 10 50 100 200
do
  for TRIAL in 1 2 3 4 5
  do
    echo "Running trial $TRIAL for concurrency $CONCURRENCY:"
    ./target/release/loadgen 10000 $CONCURRENCY
  done
done