# zedi-gen

A high-performance Rust-based tool for generating synthetic X12 835 healthcare claim payment/advice data that adheres to the X12 835 v5010 specification.

## Features

- **Synthetic Population Generation**: Create realistic patient and provider data
- **Claim Generation**: Generate X12 835 claims with configurable complexity
- **Anomaly Injection**: Inject controlled anomalies for testing purposes
- **High Performance**: Generate millions of claims per minute
- **Deterministic Output**: Reproducible results with seed-based randomization
- **Multiple Output Formats**: X12 EDI, JSON, and pretty-printed JSON

## Installation

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/zedi-gen.git
cd zedi-gen

# Build in release mode for optimal performance
cargo build --release

# The binary will be available at ./target/release/zedi-gen
```

## Usage

### Basic Usage

```bash
# Generate 1000 claims with 1% anomalies (default)
zedi-gen generate --count 1000

# Generate claims with 5% anomalies and save to a file
zedi-gen generate --count 5000 --anomaly-rate 5.0 --output claims.json

# Generate claims with a specific random seed for reproducibility
zedi-gen generate --count 1000 --seed 42

# Output in JSON format
zedi-gen generate --count 100 --format json

# Check conformance of an X12 835 file
zedi-gen conformance path/to/claims.edi
```

### Command Line Options

```bash
zedi-gen generate
Generate synthetic X12 835 healthcare claim data

USAGE:
    zedi-gen generate [OPTIONS]

OPTIONS:
    -a, --anomaly-rate <ANOMALY_RATE>    Anomaly injection rate (0.0 to 100.0) [default: 1]
    -c, --count <COUNT>                  Number of claims to generate [default: 1000]
    --data-dir <DATA_DIR>                Data directory for CSV files for realistic generation [default: data]
    --format <FORMAT>                    Output format [default: x12] [possible values: x12, json, json-pretty]
    -o, --output <OUTPUT>                Output file (default: stdout)
    --seed <SEED>                        Random seed for reproducible output
    -h, --help                           Print help information
    -V, --version                        Print version information

zedi-gen conformance
Score conformance of an X12 835 file against the spec

USAGE:
    zedi-gen conformance <INPUT_PATH>

ARGS:
    <INPUT_PATH>    Input X12 835 file to check for conformance
```

## Docker

You can also run zedi-gen using Docker:

```bash
# Build the Docker image
docker build -t zedi-gen .

# Run the container
docker run --rm -v $(pwd):/data zedi-gen generate --count 1000 --output /data/claims.json
```

## X12 Implementation

zedi-gen implements the X12 835 v5010 healthcare claim payment/advice transaction set according to the specification in `docs/spec.md`. The implementation follows a structured approach:

### X12 Module Structure

- **segments.rs**: Contains all X12 835 segment definitions (ISA, GS, ST, BPR, TRN, DTM, N1, CLP, SVC, etc.)
- **envelope.rs**: Implements the X12 envelope structure (Interchange → Functional Group → Transaction Set)
- **mod.rs**: Provides module declarations and convenient re-exports

### Key Design Principles

- **Trait-Based Approach**: All segments implement the `X12Segment` trait for consistent formatting
- **Strong Typing**: Each segment has its own struct with properly typed fields
- **Display Implementation**: All segments implement `Display` trait for EDI output formatting
- **Memory Efficiency**: Optimized for generating millions of claims with minimal memory usage

## Development

### Building and Testing

```bash
# Build in debug mode
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench
# Detailed benchmark report: see docs/benchmarks.md

# Check for warnings and errors
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Project Structure

```
zedi-gen/
├── Cargo.toml           # Project metadata and dependencies
├── src/
│   ├── main.rs         # Application entry point
│   ├── cli.rs           # Command-line interface
│   ├── config.rs        # Configuration management
│   ├── errors.rs        # Error handling
│   ├── population.rs    # Synthetic population generation
│   ├── claims.rs        # X12 835 claim generation
│   ├── anomalies.rs     # Anomaly injection
│   ├── conformance.rs   # Conformance scoring subcommand
│   ├── generator.rs     # Main generator logic
│   └── x12/             # X12 835 specific implementations
        ├── mod.rs       # Module declarations and re-exports
        ├── segments.rs  # X12 835 segment definitions
        └── envelope.rs  # X12 envelope structure (ISA, GS, ST)
├── tests/               # Integration tests
├── benchmarks/          # Benchmark tests
└── docs/                # Documentation
    └── spec.md          # X12 835 specification
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Module Structure and Imports

zedi-gen is structured as a combined library and binary crate, which requires some care with imports:

### Module Structure

- **Library Interface** (`lib.rs`): Exposes types and functionality for use as a library
- **Binary Interface** (`main.rs`): Provides the command-line application 

### Import Patterns

When working with modules in zedi-gen, follow these guidelines:

1. **Use absolute paths** for imports in most cases:
   ```rust
   use crate::x12::envelope::X12Interchange;
   ```

2. **Both `main.rs` and `lib.rs` need module declarations** for proper resolution:
   ```rust
   // In both main.rs and lib.rs
   mod x12;
   ```

3. **Re-exports in `lib.rs`** make common types available from the crate root:
   ```rust
   // In lib.rs
   pub use x12::envelope::X12Interchange;
   ```

4. **Use `#[cfg(feature = "...")]` attributes** when different import patterns are needed between library and binary use cases.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Acknowledgments

- [X12 835 Implementation Guide](https://x12.org/products/health-care-claim-payment-advice-835) - For the X12 835 specification
- [Rust](https://www.rust-lang.org/) - For making systems programming safe and productive
