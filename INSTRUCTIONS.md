### Development

---

Use the following commands to build and test the server and load generator.

Server: `cargo run --bin server`

Load Generator: `cargo run --bin loadgen`

---

### Running the experiment: 

Various configuration must be performed before running the experiment to minimize noise.

For this experiment, `PREEMPT_RT` kernel is used. Ensure that the kernel release is correct.

EX:
```
uname -r
> 6.8.1-1047-realtime
```

- Ensure turbo boost is disabled:
    - `cat /sys/devices/system/cpu/intel_pstate/no_turbo` should return 1.
- Ensure CPU frequency scaling is disabled:
    - `cat /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor` should return performance for all CPUs.
- Ensure limitation of real-time tasks is disabled:
    - `cat /proc/sys/kernel/sched_rt_runtime_us` should return -1.

If any of those checks fail, they should be configured before running the experiment.

EX: `echo 1 | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo`

This can be tweaked to write the values to each configuration.

Once these steps have been completed, the experiment can be run.

There is a `run_experiment.sh` script in the `scripts` folder. From the root, run:

```
chmod +x scripts/run_experiment.sh
./scripts/run_experiment.sh
```

Once the script is finished, the results will be in a `csv` folder in the root.

The Python script in `graphing` can be run to generate graphs to analyze the data.

### Troubleshooting

---

There may be issues with the OS limiting the amount of connections at once.
- If this error occurs: 

```
{ code: 24, kind: Uncategorized, message: "Too many open files" }
```

The operating system is limiting the connections.
- Run `ulimit -n` to check the limit.
- If it is not `65536`, run `ulimit -n 65536` to set the limit to `65536`.

---

If the `run_experiment.sh` script is prematurely killed, there may be lingering processes that need to be manually shut down.

For Docker containers:
- Run `docker ps`, and find the process running.
- Then run `docker stop <name>` to stop the container.

For the native server:
- Run `lsof -i :7878`
- Then run `kill -9 <PID>`
    - This should also work for lingering load generator processes.