# zkemail.rs-zkvm Profiling Tools

This module provides comprehensive performance profiling tools specifically designed for analyzing zkemail operations within Zero-Knowledge Virtual Machines (ZKVMs) including SP1 and RISC0.

## Overview

The profiling suite focuses on ZKVM-specific metrics and optimizations:

- **ZKVM Cycle Counting**: Precise measurement of computational cycles within zkVMs
- **Memory Profiling**: ZKVM-specific memory usage analysis and optimization
- **Cross-ZKVM Benchmarking**: Performance comparison between SP1 and RISC0
- **Production Integration**: Real proof generation and verification testing

## Components

### ZKVM Integration

#### SP1 Integration (`sp1/`)
Complete SP1 zkVM integration with guest programs and host scripts:
- Guest program for in-zkVM email verification
- Host scripts for proof generation and execution
- Cycle counting and performance measurement
- Production-ready proof generation

#### RISC0 Integration
RISC0 zkVM integration for comparative analysis:
- RISC0 guest programs
- Performance benchmarking
- Cross-platform compatibility

### Profiling Binaries

#### `zkvm_benchmarks`
Primary ZKVM benchmarking tool for performance analysis:
- Email verification within actual zkVMs
- Cycle count measurement and reporting
- Memory usage analysis in constraint systems
- Multi-size email testing

```bash
# Run SP1 benchmarks
cargo run --features sp1 --bin zkvm_benchmarks

# Run RISC0 benchmarks  
cargo run --features risc0 --bin zkvm_benchmarks
```

#### `zkvm_demo`
Interactive demonstration of ZKVM capabilities:
- Real-time proof generation
- Verification process demonstration
- Performance characteristic display

```bash
cargo run --release --bin zkvm_demo
```

### ZKVM-Specific Benchmarks

#### SP1 Benchmarks
Direct SP1 zkVM execution and proof generation:

```bash
# Execute in SP1 zkVM (no proof)
cd sp1/script
cargo run --bin execute

# Generate SP1 proofs
cargo run --bin prove -- prove --email-size 10
```

#### Performance Testing
Comprehensive ZKVM performance analysis:

```bash
# Run all ZKVM benchmarks
cargo test --features sp1,risc0 zkvm_optimization_tests

# Memory profiling in ZKVM context
cargo run --features sp1 --bin memory_analysis
```

## Quick Start

### 1. Install Dependencies

```bash
# Install SP1
curl -L https://sp1.succinct.xyz | bash
sp1up

# Install RISC0 (optional, for comparison)
cargo install cargo-risczero
cargo risczero install
```

### 2. Build SP1 Integration

```bash
cd sp1
./build.sh  # Unix systems
# or
build.bat   # Windows systems
```

### 3. Run ZKVM Benchmarks

```bash
# Quick benchmark with SP1
cargo run --features sp1 --bin zkvm_benchmarks

# Comprehensive testing
cargo test --features sp1 zkvm_optimization_tests
```

## ZKVM Performance Metrics

The profiling tools provide ZKVM-specific measurements:

- **Cycle Count**: Precise computational cycle measurement within zkVMs
- **Memory Usage**: Constraint system memory requirements
- **Proof Size**: Generated proof data size analysis
- **Verification Time**: Proof verification performance
- **Cross-ZKVM Comparison**: SP1 vs RISC0 performance analysis

## Architecture

### Guest Programs
- **SP1 Guest** (`sp1/program/`): Email verification within SP1 zkVM
- **RISC0 Guest**: Email verification within RISC0 zkVM

### Host Programs  
- **Provers**: Generate and verify ZK proofs
- **Executors**: Performance measurement without proof overhead
- **Benchmarks**: Automated performance testing suites

### Integration
- **Feature Gates**: Enable specific ZKVM frameworks
- **Test Suites**: Comprehensive functionality and performance testing
- **Examples**: Production-ready integration patterns

## Production Usage

For production integration:

1. **Choose ZKVM Framework**: SP1 or RISC0 based on requirements
2. **Build Guest Programs**: Compile verification logic for target zkVM
3. **Deploy Host Infrastructure**: Set up proof generation services
4. **Monitor Performance**: Use profiling tools for optimization

## Testing

```bash
# Run all ZKVM tests
cargo test --features sp1,risc0

# Run specific ZKVM functionality tests
cargo test zkvm_optimization_tests

# Performance regression testing
cargo test --release zkvm_performance_tests
```

## Features

- `sp1`: Enable SP1 zkVM integration
- `risc0`: Enable RISC0 zkVM integration  
- `profiling`: Enable advanced profiling capabilities

## Notes

- All benchmarks focus on ZKVM execution, not native Rust performance
- Actual cycle counts are measured within zkVM environments
- Performance data is specific to constraint system execution
- Production deployment requires proper zkVM infrastructure setup
```

#### `zkvm_demo`
Interactive demonstration of ZKVM capabilities:
- Real-time proof generation
- Verification process demonstration
- Performance characteristic display
- Pattern matching efficiency
- DFA processing benchmarks
- Complex pattern analysis

```bash
cargo run --release --bin regex_profiler
```

#### `generate_regex_data`
Utility for creating compiled DFA test data:
- Generates forward and backward DFA files
- Creates test data for amount and transaction ID patterns

```bash
cargo run --release --bin generate_regex_data
```

### Benchmark Suite

#### Email Benchmarks
Comprehensive email processing performance tests:
- Email parsing (small vs large emails)
- Body extraction benchmarks
- DKIM verification performance
- Hash function benchmarks

```bash
cargo bench --bench email_benchmarks --package zkemail-profiling
```

#### Regex Benchmarks
Pattern matching performance analysis:
- Regex compilation and matching
- DFA creation and processing
- Variable complexity pattern testing

```bash
cargo bench --bench regex_benchmarks --package zkemail-profiling
```

## Usage

### Quick Start

1. **Build the profiling tools:**
   ```bash
   cargo build --release --package zkemail-profiling
   ```

2. **Run comprehensive profiling:**
   ```bash
   # On Linux/macOS
   ./run_profiling.sh
   
   # On Windows
   profile.bat
   ```

3. **Run individual profilers:**
   ```bash
   cargo run --release --bin email_profiler
   cargo run --release --bin regex_profiler
   ```

### Advanced Profiling

#### Flamegraph Generation
For visual CPU usage analysis:

```bash
# Install flamegraph tool
cargo install flamegraph

# Generate flamegraph for email operations
cargo flamegraph --profile=release --bin email_profiler

# Generate flamegraph for benchmarks
cargo flamegraph --profile=release --bench email_benchmarks --package zkemail-profiling -- --bench
```

#### Memory Profiling

**Linux (Valgrind):**
```bash
# Heap profiling with massif
valgrind --tool=massif --massif-out-file=email_massif.out ./target/release/email_profiler

# Generate readable report
ms_print email_massif.out > email_memory_report.txt
```

**Cross-platform (dhat):**
Add to the binary's main function:
```rust
#[global_allocator]
static ALLOCATOR: dhat::Alloc = dhat::Alloc;

fn main() {
    let _dhat = dhat::Dhat::start_heap_profiling();
    // ... application code
    // Profiling data will be written to dhat-heap.json on exit
}
```

## Library API

The profiling library provides utilities for custom performance analysis:

```rust
use zkemail_profiling::{profile_cpu_usage, setup_memory_profiler};

// Initialize profiling environment
setup_memory_profiler();

// Profile a function's execution time
let result = profile_cpu_usage("data_processing", || {
    // expensive computation
    process_data()
});
```

### Available Functions

- `profile_cpu_usage(name, func)` - Profile function execution time
- `profile_memory_usage(name, func)` - Profile memory usage patterns
- `benchmark_function(name, iterations, func)` - Run performance benchmarks
- `setup_memory_profiler()` - Initialize memory profiling environment

## Output

Profiling results are organized in the `profiling_results/` directory:

```
profiling_results/
├── email_profiler_output.txt          # Email profiler console output
├── regex_profiler_output.txt          # Regex profiler console output
├── email_flamegraph.svg               # CPU profiling visualization
├── regex_flamegraph.svg               # Regex CPU profiling
├── zkvm_cycles_report.txt             # ZKVM cycle counting results
├── zkvm_memory_report.txt             # ZKVM memory usage analysis
└── target/zkvm_results/               # ZKVM performance reports
```

## Configuration

The workspace is configured for optimal profiling:

```toml
[profile.release]
opt-level = 3       # Maximum optimizations
debug = true        # Include debug symbols for profiling
lto = true          # Enable Link Time Optimization
codegen-units = 1   # Further optimize at compile time
panic = "abort"     # Remove panic unwind code
```

## Requirements

- Rust toolchain (stable)
- `cargo-flamegraph` for visual profiling
- Linux: `perf` tools for advanced flamegraphs
- Linux: `valgrind` for memory profiling

## Platform Support

- **Windows**: Basic profiling with flamegraph support
- **Linux**: Full profiling suite with Valgrind integration
- **macOS**: Flamegraph profiling with DTrace support

## Integration

This profiling suite integrates with:
- RISC0 and SP1 ZKVM frameworks for cycle counting
- Flamegraph visualization tools for CPU profiling
- dhat heap profiling for memory analysis (cross-platform)
- ZKVM-specific memory estimation tools

For detailed ZKVM optimization workflows, see the main project documentation.
