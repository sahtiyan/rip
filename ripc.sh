#!/bin/bash

# Usage message if arguments are not provided
if [ $# -lt 1 ]; then
    echo "Usage: ripc <input_file>.rip"
    exit 1
fi

# Command to run the Rust executable with the provided argument
cargo run -- $1
