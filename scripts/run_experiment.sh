#!/bin/bash

# CONFIG 1: No container
./target/release/server 127.0.0.1 &
SERVER_PID=$!
sleep 1

for CONCURRENCY in 1 10 50 100 200
do
  for TRIAL in 1 2 3 4 5
  do
    echo "Running trial $TRIAL for concurrency $CONCURRENCY, config \"bare metal\":"
    ./target/release/loadgen 10000 $CONCURRENCY $TRIAL "bare metal"
  done
done

kill $SERVER_PID

# CONFIG 2: Docker host mode
docker run --rm -d --name server1 --network host fraud-server
sleep 1 

for CONCURRENCY in 1 10 50 100 200
do
  for TRIAL in 1 2 3 4 5
  do
    echo "Running trial $TRIAL for concurrency $CONCURRENCY, config \"docker host mode\":"
    ./target/release/loadgen 10000 $CONCURRENCY $TRIAL "docker host"
  done
done

docker stop server1

# CONFIG 3: Docker bridge mode
docker run --rm -d --name server2 -p 7878:7878 fraud-server
sleep 1

for CONCURRENCY in 1 10 50 100 200
do
  for TRIAL in 1 2 3 4 5
  do
    echo "Running trial $TRIAL for concurrency $CONCURRENCY, config \"docker bridge mode\":"
    ./target/release/loadgen 10000 $CONCURRENCY $TRIAL "docker bridge"
  done
done

docker stop server2