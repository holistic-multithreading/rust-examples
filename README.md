# run benchmarks

```bash
cargo bench
```

## benchmark results

MacOS Ventura Intel i9 2.9 GHz, 6 physical cores (12 logical) 32Gb memory

### MacOS i9

| range | 1 thread | 12 threads |
| ----- | ------------- | ---------- |
| thousand | 1.5 µs     | 221.5 µs  |
| million  | 1.2 ms     | 438.5 µs  |
| billion  | 1.3 s      | 207.3 ms  |

### MacOS i9 - Docker

```bash
docker run -it -v "$PWD":/code rust sh
cd /code
cargo bench
```

| range | 1 thread | 12 threads |
| ----- | ------------- | ---------- |
| thousand | 1.8 µs  | 464.3 µs  |
| million  | 1.4 ms  | 858.5 µs  |
| billion  | 1.4 s   | 325.5 ms  |
