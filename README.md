# faf-json-bench

`faf-json-bench` is a Rust JSON serialization and deserialization benchmarking tool.

Only Linux is supported.

## Why

I was looking for benchmarks and couldn't find any that included all the libraries I wanted tested.

## How to Use

You must comment out incompatible RUSTFLAGS in `./cargo/config`. There will usually only be two ("-Clinker=/usr/bin/clang-15","-Clink-arg=-fuse-ld=/usr/bin/ld.lld"). These are so I can use clang / llvm when running the test but aren't necessary to see results on the benchmark.

After that, just run it as usual with the optional flag to control duration per test. Here are some :

- While working on this, I routinely check it with `cargo run --release -- -d3`
- When actually benchmarking, I set a higher priority and run the benchmark with `sudo nice -20 ./target/release/faf-json-bench -d15` after building it with `cargo build --release`

## Libraries Tested

- [serde_json](https://github.com/serde-rs/json)
- [serde-json-core](https://github.com/rust-embedded-community/serde-json-core/)
- [nanoserde](https://github.com/not-fl3/nanoserde)

## Methodology

- LLVM / Clang (v15)
- MiMalloc Allocator
- Elevated priority
- Minimized system-wide noise. I always take precautions to ensure my benchmarking isn't adversely affected by other running programs by not having anything open/running, including most background services
- Compiled with flags specified in `./cargo/config`, which include `"-Ctarget-cpu=native"`-
- To ensure results aren't optimized away, I always check the output after serializing / deserializing for correctness
- To ensure, loops aren't unpredictably vectorized, I check the time on each iteration instead of doing a fixed number of iterations. My time check uses VDSO on Linux which is very fast. I thought Rust's standard library also used VDSO on Linux but when I switched to my own implementation times improved by over 10%
- Currently only testing with 'small input'

System info for context

![](ref/fetch.png)

## Results

```
serde_json to_vec         580,340,923 bytes/sec
serde_json to_writer      517,163,471 bytes/sec
serde_json_core to_vec    560,080,454 bytes/sec
serde_json_core to_slice  602,837,522 bytes/sec
nanoserde serialize_json  248,805,747 bytes/sec
nanoserde ser_json        439,978,413 bytes/sec
```

## Why Only Linux?

Unless there is some unexpected popular demand, I only want to support what I actually use and would prefer not to add the complexity to support other platforms. Also, I often use techniques for performance reasons that are Linux specific and getting similar results on other platforms may be complex.

## Observations:

- For `serde_json_core to_vec`, checking the len with `assert!(bytes_len == 26);` after the serialization actually makes the operation faster, probably because it is a compiler hint
- Cargo flags make a big difference for some frameworks like `serde_json_core`, which gained ~15% performance after adding flags to `[profile.release.package."*"]` and adding `rustflags` to `./cargo/config`. Only together does the gain appear...
- Clang is faster than gcc for some tests but not others. For example, the `serde_json to_writer` is slightly faster with gcc but both `serde-json-core` tests are much faster with Clang
