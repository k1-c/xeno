# Development Guide

## Setup

```bash
# Clone and build
git clone https://github.com/k1-c/xeno
cd xeno
cargo build

# Install toolchain (handled automatically by rust-toolchain.toml)
# MSRV: 1.82.0
```

## Development Commands

```bash
# Build all crates
cargo build

# Run tests
cargo test

# Run tests with coverage
cargo test --all-features

# Format code
cargo fmt

# Lint with clippy
cargo clippy

# All checks (typical pre-commit)
cargo fmt && cargo clippy && cargo test
```

## Project Structure

```
xeno/
├── core/              # Framework core (platform-agnostic)
├── adapters/
│   ├── hyper/         # Hyper server adapter
│   └── workers/       # Cloudflare Workers adapter
├── examples/
│   ├── hello-hyper/   # Basic hyper example
│   └── hello-workers/ # Basic workers example
└── tools/
    └── openapi-gen/   # OpenAPI generator
```

## Running Examples

```bash
# Hyper example
cd examples/hello-hyper
cargo run
# -> http://localhost:8080

# Workers example (when implemented)
cd examples/hello-workers
wrangler dev
```

## Testing

```bash
# Unit tests
cargo test --lib

# Integration tests  
cargo test --test '*'

# Test specific crate
cargo test -p xeno-core

# Test with output
cargo test -- --nocapture
```

## Development Workflow

1. Create feature branch
2. Implement changes
3. Run `cargo fmt && cargo clippy && cargo test`
4. Commit with conventional commits
5. Create PR

## Configuration Files

- `rust-toolchain.toml` - Rust version (1.82.0)
- `rustfmt.toml` - Code formatting (100 char width)
- `clippy.toml` - Lint configuration
- `Cargo.toml` - Workspace dependencies

## Debugging

```bash
# Run with debug output
RUST_LOG=debug cargo run

# Run specific test with logs
RUST_LOG=debug cargo test test_name -- --nocapture
```

## Performance Testing

```bash
# Release build
cargo build --release

# Benchmark (when implemented)
cargo bench
```

## Documentation

```bash
# Generate docs
cargo doc --open

# Check doc tests
cargo test --doc
```