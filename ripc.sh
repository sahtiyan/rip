#!/bin/bash

# Usage message if arguments are not provided
if [ $# -lt 1 ]; then
    echo "Usage: ripc <input_file>.rip"
    exit 1
fi

# Rip Compiler'ın bulunduğu yol
RIP_COMPILER_DIR="/usr/bin/rip"

# Command to run the Rip Compiler executable with the provided argument
$RIP_COMPILER_DIR/ripc "$@"
