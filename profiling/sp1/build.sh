#!/bin/bash

# SP1 zkEmail Build Script
# This script builds the SP1 guest program and generates the required ELF files

set -e

echo "Building SP1 zkEmail guest program..."

# Navigate to the program directory
cd "$(dirname "$0")/program"

# Build the SP1 program
cargo prove build

# Copy the generated ELF to the expected location
mkdir -p ../elf
cp ./elf/zkmail-sp1-program ../elf/

echo "SP1 program built successfully!"
echo "ELF file available at: ../elf/zkmail-sp1-program"

# Navigate back to script directory to test
cd ../script

echo "Testing program execution..."
cargo run --bin execute

echo "Build and test completed successfully!"
