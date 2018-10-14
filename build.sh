#!/bin/bash
export PATH=/home/tpg/apps/bin:$PATH
export XARGO_RUST_SRC=$(pwd)/deps/rustc-sysroot/src
export RUST_TARGET_PATH=$(pwd)
export CC=arm-mingw32ce-gcc

LD_PRELOAD= CARGO_INCREMENTAL=0 xargo build --target armv4-mingw32ce -v
