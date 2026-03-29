# docker-overhead-bench

You may get issues with the OS limiting the amount of connections at once. If you get this error:

```
{ code: 24, kind: Uncategorized, message: "Too many open files" }
```

The operating system is limiting the connections. Run `ulimit -n` to check the limit. If it is not `65536`, run `ulimit -n 65536` to set the limit to `65536`.