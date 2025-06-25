#!/bin/bash
export RUSTC_BOOTSTRAP=1
export RUST_BACKTRACE=full
cargo build --profile dev --target aarch64-unknown-uefi -Zbuild-std=core,compiler_builtins,alloc -Zbuild-std-features=compiler-builtins-mem -Zunstable-options --timings=html
