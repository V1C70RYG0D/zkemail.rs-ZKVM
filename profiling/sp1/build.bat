@echo off
REM SP1 zkEmail Build Script for Windows
REM This script builds the SP1 guest program and generates the required ELF files

echo Building SP1 zkEmail guest program...

REM Navigate to the program directory
pushd "%~dp0program"

REM Build the SP1 program
cargo prove build

REM Copy the generated ELF to the expected location
if not exist "..\elf" mkdir "..\elf"
copy ".\elf\zkmail-sp1-program" "..\elf\"

echo SP1 program built successfully!
echo ELF file available at: ..\elf\zkmail-sp1-program

REM Navigate back to script directory to test
popd
pushd "%~dp0script"

echo Testing program execution...
cargo run --bin execute

echo Build and test completed successfully!
popd
