# Actually Portable Executables with Cosmopolitan Libc and Rust

This repository contains a simple `Hello world!` example in the [Rust][rust]
programming language, that builds with [Cosmopolitan Libc][cosmo]. Now it also
includes example snippets scraped from [Rust By Example][rbe],
and it builds around 175 example programs, including those that use Rust's
`std::thread` and `std::sync::Arc`.

> `ripgrep` builds with Cosmopolitan Libc -- check it out
> [here](https://github.com/ahgamut/ripgrep/tree/cosmopolitan).


To build this repo you need a recent version of `gcc` (9 or 10 ought to be
good), a recent version of `binutils` (`ld.bfd` and `objcopy`), and `bash`
due to a simple filter script.

This includes a [custom compilation target][custom-target] for Rust, called
`x86_64-unknown-linux-cosmo`, to provide a build process that uses the
Cosmopolitan Libc amalgamation and `cargo`.

## Building a Rust APE with the `std` crate

1. Download the Cosmopolitan Libc [amalgamation][amalg-download] into the `libcosmo` folder:

```bash
cd libcosmo
wget https://justine.lol/cosmopolitan/cosmopolitan.zip
unzip cosmopolitan.zip
cd ../
```

For reference, I used the `cosmopolitan.zip` v2.2 from November 7, 2022.

2. Download the necessary host toolchain and source code for Rust:

```bash
# Download Rust
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
# I was on Ubuntu 20.04, so I did this
rustup toolchain install nightly-x86_64-unknown-linux-gnu
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```

For reference, I used `rustc 1.67.1 (d5a82bbd2 2023-02-07)`.

3. run `cargo build` to get the debug executable. This uses a bash script that
   removes unnecessary linker arguments. A recent version of `gcc` and `ld.bfd`
   is required.

```bash
cargo +nightly build -Zbuild-std=libc,panic_abort,std -Zbuild-std-features=""  --target=./x86_64-unknown-linux-cosmo.json
```

For reference, I used the below versions of `gcc` and `ld.bfd`

```
gcc version 9.4.0 (Ubuntu 9.4.0-1ubuntu1~20.04.1) 
GNU ld (GNU Binutils for Ubuntu) 2.34
```

4. run `objcopy` on the debug binary to obtain the APE:

```bash
# objcopy is the same version as ld.bfd above
objcopy -SO binary ./target/x86_64-unknown-linux-cosmo/debug/hello.com.dbg ./hello.com
# run the APE
./hello.com
# see syscalls made by the APE
./hello.com --strace
```

Now we have Actually Portable Executables built with Rust!

## TODOs

- [ ] figure out build config to avoid using `libunwind`

The `std` crate relies on
[`backtrace`](https://github.com/rust-lang/backtrace-rs), which depends on
[`libunwind`](https://github.com/libunwind/libunwind) in the default builds for
unix. To work around this, `cosmopolitan.a` currently has stubs for the
functions that `backtrace` relies on. However, it might be easier to provide a
build flag in `Cargo.toml` to use the `noop` module of `backtrace`. 

A small change needs to be submitted to the source code of `backtrace` (in the
`cfg_if!`
[here](https://github.com/rust-lang/backtrace-rs/blob/4e5a3f72929f152752d5659e95bb15c8f6b41eff/src/backtrace/mod.rs#L128))
to allow choosing `noop` when building as part of the `std` crate. This
conditional compilation flag should be accessible when building the `std` crate
either via `Cargo.toml` or something like `-Z use-std-backtrace-noop` in the
build command.

[without-std-branch]: https://github.com/ahgamut/rust-ape-example/tree/without-std
[rust]: https://rust-lang.org
[rbe]: https://doc.rust-lang.org/rust-by-example/
[cosmo]: https://github.com/jart/cosmopolitan
[cosmo-nightly]: https://github.com/jart/cosmopolitan/commit/b69f3d2488dbaf9dcc541e699f5b7c09fbf046e0
[amalg-download]: https://justine.lol/cosmopolitan/download.html
[custom-target]: https://doc.rust-lang.org/rustc/targets/custom.html
[custom-embed]: https://docs.rust-embedded.org/embedonomicon/custom-target.html
