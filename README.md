# Blake2 Yet Another Native Test

**Fuzz**

```sh
$ cargo +nightly fuzz run main
```

**Benchmark**

```sh
$ cargo bench
# bench_blake2b_rs        time:   [12.974 µs 13.099 µs 13.248 µs]
# bench_blake2ya          time:   [10.672 µs 10.753 µs 10.849 µs]
```

21% faster than [blake2b_rs](https://github.com/nervosnetwork/blake2b-rs).
