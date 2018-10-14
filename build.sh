#!/bin/bash
set -e
export PROJECT_ROOT=$(pwd)
export PATH=${PROJECT_ROOT}/ThirdParty/prefix/bin:$PATH
export XARGO_RUST_SRC=${PROJECT_ROOT}/deps/rustc-sysroot/src
export RUST_TARGET_PATH=${PROJECT_ROOT}
export CC=arm-mingw32ce-gcc

LD_PRELOAD= CARGO_INCREMENTAL=0 xargo build --target armv4-mingw32ce -v
