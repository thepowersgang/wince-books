#!/bin/sh
PREFIX=$(pwd)/prefix
cd cegcc-build
cd build-arm-mingw32ce
./build-mingw32ce.sh --prefix=${PREFIX}

