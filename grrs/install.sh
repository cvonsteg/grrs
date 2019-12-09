#!/bin/bash

# Build app
 cargo b --release

# Add to local binaries
ln -s $PWD/target/release/grrs /usr/local/bin/grrs
