# SP1 zkEmail Integration

This directory contains the SP1 zkVM integration for zkemail.rs-zkvm, enabling zero-knowledge proof generation for email verification within the SP1 framework.

## Directory Structure

```
sp1/
├── program/           # SP1 guest program (runs inside zkVM)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs    # Email verification logic for SP1 zkVM
├── script/            # SP1 host programs (prover and executor)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs    # Main prover script
│       └── execute.rs # Execution-only script for benchmarking
├── elf/               # Generated ELF files (created during build)
├── build.sh           # Build script for Unix systems
└── build.bat          # Build script for Windows systems
```

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/getting-started/install.html)

## Building the SP1 Program

### On Unix systems (Linux/macOS):
```bash
cd profiling/sp1
./build.sh
```

### On Windows:
```cmd
cd profiling\sp1
build.bat
```

## Running SP1 zkEmail

### Execute without proof generation (for benchmarking):
```bash
cd script
cargo run --bin execute
```

### Generate a proof:
```bash
cd script
cargo run --bin prove -- prove --email-size 10
```

### Execute with custom email size:
```bash
cd script
cargo run --bin prove -- execute --email-size 20
```

## Integration with Main Profiling Crate

The SP1 integration is also available through the main profiling crate:

```bash
# From the profiling/ directory
cargo run --features sp1 --bin zkvm_benchmarks
```

## Performance Characteristics

The SP1 integration provides:

- **Cycle Counting**: Actual zkVM cycle measurements during execution
- **Memory Profiling**: ZKVM-specific memory usage analysis  
- **Proof Generation**: Production-ready ZK proof creation
- **Verification**: On-chain verifiable proofs

## Example Output

```
SP1 zkEmail - Execution Mode

--- Testing 10KB email ---
Email verification: VALID
Cycles in zkVM: 1,234,567
Total instructions: 2,345,678
Total cycles: 1,234,567
Host execution time: 125ms
```

## Architecture

1. **Guest Program** (`program/src/main.rs`): Runs inside SP1 zkVM, performs email verification
2. **Host Script** (`script/src/main.rs`): Manages proof generation and verification
3. **Executor** (`script/src/execute.rs`): Benchmarking without proof overhead

The guest program receives an `EmailVerificationInput` struct, performs verification using `zkemail-core`, and outputs an `EmailVerificationOutput` with cycle count information.

## Notes

- ELF files are generated during the build process and included in the host scripts
- Actual cycle counts are measured within the zkVM for accurate performance data
- The integration supports variable email sizes for comprehensive benchmarking
- All verification logic maintains compatibility with the core zkemail.rs functionality
