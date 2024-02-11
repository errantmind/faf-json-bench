# faf-json-bench

`faf-json-bench` is a Rust JSON serialization and deserialization benchmarking tool.

Only Linux is supported.

## Why

I was looking for benchmarks and couldn't find any that included all the libraries I wanted tested.

## How to Use

You must comment out incompatible RUSTFLAGS in `./cargo/config`. There will usually only be two ("-Clinker=/usr/bin/clang-16","-Clink-arg=-fuse-ld=/usr/bin/ld.lld"). These are so I can use clang / llvm when running the test but aren't necessary to see results on the benchmark.

After that, if using clang, run it with `./clang_build_and_run.sh`, otherwise run it with `cargo build --release && sudo nice -20 ./target/release/faf-json-bench`

## Libraries Tested

- [serde_json](https://github.com/serde-rs/json)
- [serde-json-core](https://github.com/rust-embedded-community/serde-json-core/)
- [nanoserde](https://github.com/not-fl3/nanoserde)
- [simd-json](https://github.com/simd-lite/simd-json)
- [simd-json-derive](https://github.com/simd-lite/simd-json-derive)
- [sonic-rs](https://github.com/cloudwego/sonic-rs)

## Results (small 26 byte serialization)

### TCMalloc

```
sonic-rs          to_vec               785,789,775 bytes/sec
sonic-rs          to_writer            929,728,115 bytes/sec
serde_json        to_vec               668,032,647 bytes/sec
serde_json        to_writer            770,139,985 bytes/sec
serde_json_core   to_vec               598,008,355 bytes/sec
serde_json_core   to_slice             652,524,808 bytes/sec
nanoserde         serialize_json       302,670,533 bytes/sec
nanoserde         ser_json             515,064,108 bytes/sec
simd_json         to_vec             1,165,577,270 bytes/sec
simd_json         to_writer          1,262,988,965 bytes/sec
simd_json_derive  json_vec           1,056,956,455 bytes/sec
simd_json_derive  json_write         1,334,914,161 bytes/sec
```

### MiMalloc (mimalloc v0.1.39 (libmimalloc-sys v0.1.35))

```
sonic-rs          to_vec               691,168,747 bytes/sec
sonic-rs          to_writer            915,829,798 bytes/sec
serde_json        to_vec               608,365,940 bytes/sec
serde_json        to_writer            759,952,561 bytes/sec
serde_json_core   to_vec               633,651,573 bytes/sec
serde_json_core   to_slice             652,826,200 bytes/sec
nanoserde         serialize_json       280,116,027 bytes/sec
nanoserde         ser_json             495,267,370 bytes/sec
simd_json         to_vec               927,485,208 bytes/sec
simd_json         to_writer          1,234,747,046 bytes/sec
simd_json_derive  json_vec             863,682,707 bytes/sec
simd_json_derive  json_write         1,330,641,520 bytes/sec
```

### MiMalloc (mimalloc-rust v0.2.1)

```
sonic-rs          to_vec               686,959,477 bytes/sec
sonic-rs          to_writer            912,113,921 bytes/sec
serde_json        to_vec               587,635,143 bytes/sec
serde_json        to_writer            748,133,447 bytes/sec
serde_json_core   to_vec               599,925,430 bytes/sec
serde_json_core   to_slice             644,265,518 bytes/sec
nanoserde         serialize_json       300,077,492 bytes/sec
nanoserde         ser_json             515,323,623 bytes/sec
simd_json         to_vec               938,033,529 bytes/sec
simd_json         to_writer          1,263,035,453 bytes/sec
simd_json_derive  json_vec             869,011,511 bytes/sec
simd_json_derive  json_write         1,335,306,561 bytes/sec
```

### SnMalloc Allocator

```
sonic-rs          to_vec               788,752,848 bytes/sec
sonic-rs          to_writer            921,088,350 bytes/sec
serde_json        to_vec               681,721,777 bytes/sec
serde_json        to_writer            760,149,347 bytes/sec
serde_json_core   to_vec               568,485,797 bytes/sec
serde_json_core   to_slice             644,376,763 bytes/sec
nanoserde         serialize_json       346,096,400 bytes/sec
nanoserde         ser_json             515,741,321 bytes/sec
simd_json         to_vec             1,222,398,979 bytes/sec
simd_json         to_writer          1,235,489,129 bytes/sec
simd_json_derive  json_vec           1,052,723,187 bytes/sec
simd_json_derive  json_write         1,335,253,937 bytes/sec
```

## Methodology

- LLVM / Clang (v16)
- MiMalloc Allocator
- Elevated priority
- Minimized system-wide noise. I always take precautions to ensure my benchmarking isn't adversely affected by other running programs by not having anything open/running, including most background services
- Compiled with flags specified in `./cargo/config`, which include `"-Ctarget-cpu=native"`-
- To ensure results aren't optimized away, I always check the output after serializing / deserializing for correctness
- To ensure, loops aren't unpredictably vectorized, I check the time on each iteration instead of doing a fixed number of iterations. My time check uses VDSO on Linux which is very fast. I thought Rust's standard library also used VDSO on Linux but when I switched to my own implementation times improved by over 10%
- Currently only testing with 'small input'

System info for context

![](ref/fetch.png)

## Why Only Linux?

Unless there is some unexpected popular demand, I only want to support what I actually use and would prefer not to add the complexity to support other platforms. Also, I often use techniques for performance reasons that are Linux specific and getting similar results on other platforms may be complex.

## Observations:

- Choice of allocator matters the most when using methods that allocate a buffer (.. yep)
- For `serde_json_core to_vec`, checking the len with `assert!(bytes_len == 26);` after the serialization actually makes the operation faster, probably because it is a compiler hint
- Clang is faster than gcc for some tests but not others. For example, the `serde_json to_writer` is slightly faster with gcc but both `serde-json-core` tests are much faster with Clang

## Contributions

Contributions are welcome, but please discuss before submitting a pull request. If a discussion leads to a pull request, please reference the \#issue in the pull request. Unsolicited pull requests will not be reviewed nor merged.

## License

All code is licensed under AGPL 3.0 unless an individual source file specifies otherwise.
