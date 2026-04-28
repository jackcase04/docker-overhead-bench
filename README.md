# Container Overhead Analysis in Soft Real-Time Fraud Detection Systems 

### Summary

This repository stores the relevant source code for an experiment analyzing the effect of containerization on a real-time fraud detection service.
A simulated fraud detection service written in Rust is deployed in 3 different configurations:
- Natively on the host machine
- In a Docker container with host networking
- In a Docker container with bridge networking (default behavior)

Results of the experiment show that bridge networking has significantly more overhead then native and host configurations, with latency rising from *1x* more mean latency with single threaded configurations, to over *2x* higher mean latency then native and host modes at high concurrency levels *(150 concurrent connections)*.