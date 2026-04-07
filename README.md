# Container Overhead Analysis in Soft Real-Time Fraud Detection Systems (CE5170)

### Building

Use the following commands to build and test the server and load generator.

Server: `cargo run --bin server`

Load Generator: `cargo run --bin loadgen`

When replicating the actual expiriment, be sure to provide the `--release` flag to cargo to optimize the build for the data collection.

---

### Analysis

To generate the plot after running a test, run `python3 scripts/plot.py`.

---

### Running the server

If you made changes to the server code, be sure to run `docker built -t fraud-server .`

Natively:

`./target/release/server 127.0.0.1`

Docker bridge mode:

`docker run --rm -d --name server1 -p 7878:7878 fraud-server`

Docker host mode:

`docker run --rm -d --name server2 --network host fraud-server`

Shutting down the containers:

The `-d` flag will run the containers in the background. That means that to disable the containers you will need to run `docker stop`.

Run `docker ps` to find the running containers, then `docker stop <name>` to stop the container.

### Troubleshooting

You may get issues with the OS limiting the amount of connections at once. If you get this error:

```
{ code: 24, kind: Uncategorized, message: "Too many open files" }
```

The operating system is limiting the connections. Run `ulimit -n` to check the limit. If it is not `65536`, run `ulimit -n 65536` to set the limit to `65536`.