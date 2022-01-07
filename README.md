# Counting triangles in data streams

Implementation of Buriol, Luciana S., et al. "Counting triangles in data streams." Proceedings of the twenty-fifth ACM SIGMOD-SIGACT-SIGART symposium on Principles of database systems. 2006.

## Building

To build and run the complete version:

```bash
make build
make run
```

### Alternate version

To build and run the alternate version of the incidence stream algorithm:

```bash
make alt=1 build
make alt=1 run num_samples=<num_samples> filename=<fileName>
```

##  Test

Tu run in debug mode:

```bash
make debug
```

### Analyze results

To start collecting results in `<filename>.csv` run:

```bash
make <filename>.csv
```

To analyze the collected results:

```bahs
make analyze
```
