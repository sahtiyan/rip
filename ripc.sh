#!/bin/bash

# Usage message if arguments are not provided
if [ $# -lt 1 ]; then
    echo "Usage: ripc <input_file>.rip"
    exit 1
fi

cargo run -- $1
