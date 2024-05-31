#!/bin/bash

set -e

# Define the libpostal directory and data directory
LIBPOSTAL_DIR="libpostal"
DATA_DIR="$(pwd)/libpostal_data"

# Clone the libpostal repository if it doesn't exist
if [ ! -d "$LIBPOSTAL_DIR" ]; then
    git clone https://github.com/openvenues/libpostal
fi

cd libpostal

# Build libpostal if it's not already built
if [ ! -f "src/.libs/libpostal.a" ]; then
    ./bootstrap.sh
    ./configure --prefix=$HOME/.local --datadir="$DATA_DIR"
    make -j$(nproc)
    make install
fi

cd ..
