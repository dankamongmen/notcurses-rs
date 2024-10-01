#!/usr/bin/env sh
#
# Verifies multi-feature compilation, test runs, and documentation build.

set -e # stops on error

MSRV="1.65.0" # sync with README.md, Cargo.toml & .github/workflows/check.yml
RCMD="rustup -v run $MSRV"

rustup override set $MSRV

# install additional targets
#
# (1) x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-gnu
#
# (2) x86_64-pc-windows-msvc
rustup target add x86_64-pc-windows-msvc
#
# (3) x86_64-apple-darwin
rustup target add x86_64-apple-darwin
#
# (5) Bare x86_64, softfloat, 64-bit, no_std
# https://doc.rust-lang.org/nightly/rustc/platform-support/x86_64-unknown-none.html
rustup target add x86_64-unknown-none # tier 2 (_64)
#
# (6) Linux i686, 32-bit, std, little-endian, (kernel 3.2+, glibc 2.17+)
# may need to install libc6-dev-amd64-i386-cross for testing
rustup target add i686-unknown-linux-gnu # tier 1 (_686)
#
# (7) Bare ARM64, hardfloat, 64-bit, no_std, little-endian, A64 set, (M1, M2 processors)
rustup target add aarch64-unknown-none # tier 2 (_aarch)
#
# (8) Bare ARMv7-M, 32-bit, no_std, little-endian, Thumb set, (Cortex-M processors)
rustup target add thumbv7m-none-eabi # tier 2 (_thumb)


# check
cmd="$RCMD cargo c"; echo "std, safe\n$ " $cmd; $cmd
# cmd="$RCMD cargo cu"; echo "std, unsafe\n$" $cmd; $cmd
# cmd="$RCMD cargo cn"; echo "no_std, safe\n$" $cmd; $cmd

# check additional targets
# cmd="$RCMD cargo cuT1"; echo "std, unsafe\n$" $cmd; $cmd
# cmd="$RCMD cargo cuT2"; echo "std, unsafe\n$" $cmd; $cmd
# cmd="$RCMD cargo cuT3"; echo "std, unsafe\n$" $cmd; $cmd
# cmd="$RCMD cargo cnuT5"; echo "no_std, no-alloc, unsafe\n$" $cmd; $cmd
# cmd="$RCMD cargo cuT6"; echo "std, unsafe\n$" $cmd; $cmd
# cmd="$RCMD cargo cnuT7"; echo "no_std, no-alloc, unsafe\n$" $cmd; $cmd
# cmd="$RCMD cargo cnuT8"; echo "no_std, no-alloc, unsafe\n$" $cmd; $cmd

# test
cmd="$RCMD cargo t"; echo "tests\n$" $cmd; $cmd
# cmd="$RCMD cargo tu"; echo "tests, unsafe\n$" $cmd; $cmd

# docs
cmd="cargo +nightly nd"; echo "docs\n$" $cmd; $cmd
