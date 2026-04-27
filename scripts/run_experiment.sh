#!/bin/bash

# cargo build --release
# docker build -t fraud-server .

# CONFIG 1: No container
taskset -c 2 chrt -f 90 ./target/release/server 127.0.0.1 &
SERVER_PID=$(lsof -ti :7878)
sleep 1

for CONCURRENCY in 1 10 50 100 150
do
  echo "Warmup trials for $CONCURRENCY:"
  ./target/release/loadgen 10000 $CONCURRENCY
  ./target/release/loadgen 10000 $CONCURRENCY 
  for TRIAL in 1 2 3 4 5
  do
    echo "Running trial $TRIAL for concurrency $CONCURRENCY, config \"native\":"
    ./target/release/loadgen 10000 $CONCURRENCY $TRIAL "native"
    sleep 2
  done
done

kill -9 $SERVER_PID
sleep 1

# CONFIG 2: Docker host mode
docker run --rm --cpuset-cpus=2 --memory-swappiness=0 --cap-add=SYS_NICE --ulimit rtprio=90 -d --name server1 --network host fraud-server
sleep 1

for CONCURRENCY in 1 10 50 100 150
do
  echo "Warmup trials for $CONCURRENCY:"
  ./target/release/loadgen 10000 $CONCURRENCY
  ./target/release/loadgen 10000 $CONCURRENCY 
  for TRIAL in 1 2 3 4 5
  do
    echo "Running trial $TRIAL for concurrency $CONCURRENCY, config \"host\":"
    ./target/release/loadgen 10000 $CONCURRENCY $TRIAL "host"
    sleep 2
  done
done

docker stop server1

# CONFIG 3: Docker bridge mode
docker run --rm --cpuset-cpus=2 --memory-swappiness=0 --cap-add=SYS_NICE --ulimit rtprio=90 -d --name server2 -p 7878:7878 fraud-server
sleep 1

for CONCURRENCY in 1 10 50 100 150
do
  echo "Warmup trials for $CONCURRENCY:"
  ./target/release/loadgen 10000 $CONCURRENCY
  ./target/release/loadgen 10000 $CONCURRENCY 
  for TRIAL in 1 2 3 4 5
  do
    echo "Running trial $TRIAL for concurrency $CONCURRENCY, config \"bridge\":"
    ./target/release/loadgen 10000 $CONCURRENCY $TRIAL "bridge"
    sleep 2
 done
done

docker stop server2
