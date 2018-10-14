#!/bin/sh
git clone git://github.com/MaxKellermann/cegcc-build --shallow-submodules --depth 1
cd cegcc-build
git submodule init
git submodule update --depth 1	# Might not need --depth, since --shallow-submodules was specified

# - Add include directory required by gcc build
mkdir gcc/winsup
cd gcc/winsup; ln -s ../../w32api
