#!/bin/bash

set -e

# Define the libpostal directory and data directory
LIBPOSTAL_DIR="libpostal"
DATA_DIR="$(pwd)/libpostal_data"

# Clone the libpostal repository if it doesn't exist
if [ ! -d "$LIBPOSTAL_DIR" ]; then
    git clone https://github.com/openvenues/libpostal
fi

# Change to the libpostal directory
pushd libpostal

# Build libpostal if it's not already built
if [ ! -f "src/.libs/libpostal.a" ]; then
    ./bootstrap.sh
    ./configure --datadir="$DATA_DIR"
    make -j$(nproc)
fi

popd
# Print the absolute path of the libpostal directory
echo $(pwd)
