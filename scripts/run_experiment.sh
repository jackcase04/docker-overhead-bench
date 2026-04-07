#!/bin/bash

# CONFIG 1: No container
./target/release/server 127.0.0.1 &
SERVER_PID=$!
sleep 1

for CONCURRENCY in 1 10 50 100 200
do
  for TRIAL in 1 2 3 4 5
  do
    echo "Running trial $TRIAL for concurrency $CONCURRENCY:"
    ./target/release/loadgen 10000 $CONCURRENCY $TRIAL "bare metal"
  done
done

kill $SERVER_PID