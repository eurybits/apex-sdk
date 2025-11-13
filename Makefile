.PHONY: help setup build test bench docs clean lint format

help:
	@echo "Apex SDK - Development Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  setup    - Install development dependencies"
	@echo "  build    - Build all crates"
	@echo "  test     - Run all tests"
	@echo "  bench    - Run benchmarks"
	@echo "  docs     - Generate documentation"
	@echo "  lint     - Run clippy linter"
	@echo "  format   - Format code with rustfmt"
	@echo "  clean    - Clean build artifacts"

setup:
	@echo "Installing Rust toolchain..."
	rustup update stable
	rustup component add clippy rustfmt
	@echo "✅ Setup complete!"

build:
	@echo "Building all crates..."
	cargo build --all-features
	@echo "✅ Build complete!"

test:
	@echo "Running tests..."
	cargo test --all-features
	@echo "✅ Tests passed!"

bench:
	@echo "Running benchmarks..."
	cargo bench --all-features
	@echo "✅ Benchmarks complete!"

docs:
	@echo "Generating documentation..."
	cargo doc --all-features --no-deps --open
	@echo "✅ Documentation generated!"

lint:
	@echo "Running clippy..."
	cargo clippy --all-features -- -D warnings
	@echo "✅ Lint checks passed!"

format:
	@echo "Formatting code..."
	cargo fmt --all
	@echo "✅ Code formatted!"

clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@echo "✅ Clean complete!"

.DEFAULT_GOAL := help
