# Blake2 Yet Another Native Test

**Fuzz**

```sh
$ cargo +nightly fuzz run main
```

**Benchmark**

```sh
$ cargo bench
# bench_blake2b_rs        time:   [8.3190 µs 8.4019 µs 8.5075 µs]
# bench_blake2ya          time:   [6.7516 µs 6.7632 µs 6.7776 µs]
```

24% faster than [blake2b_rs](https://github.com/nervosnetwork/blake2b-rs).
