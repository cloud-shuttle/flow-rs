# Makefile for Leptos Flow with timeout management
.PHONY: help test test-quick test-full test-coverage clean fix-format fix-lints

# Default target
help:
	@echo "Leptos Flow Development Commands"
	@echo "================================"
	@echo ""
	@echo "Testing Commands:"
	@echo "  test          - Run all tests with timeout protection"
	@echo "  test-quick    - Run quick tests (10s timeout)"
	@echo "  test-full     - Run all tests including slow ones"
	@echo "  test-core     - Run core package tests only"
	@echo "  test-spatial  - Run spatial tests with 30s timeout"
	@echo "  test-proptest - Run proptest with 45s timeout"
	@echo ""
	@echo "Code Quality:"
	@echo "  fix-format    - Fix code formatting"
	@echo "  fix-lints     - Fix clippy warnings"
	@echo "  test-coverage - Generate test coverage report"
	@echo ""
	@echo "Maintenance:"
	@echo "  clean         - Clean build artifacts"
	@echo "  install-nextest - Install nextest test runner"

# Install nextest for better test management
install-nextest:
	@echo "Installing nextest test runner..."
	cargo install cargo-nextest

# Test commands with timeout protection
test: install-nextest
	@echo "🧪 Running tests with timeout protection..."
	./scripts/test-with-timeout.sh leptos-flow-core 60 1 all

test-quick: install-nextest
	@echo "⚡ Running quick tests (10s timeout)..."
	./scripts/test-with-timeout.sh leptos-flow-core 10 1 all

test-full: install-nextest
	@echo "🔬 Running full test suite..."
	cargo nextest run --profile default

test-core:
	@echo "🎯 Running core package tests..."
	./scripts/test-with-timeout.sh leptos-flow-core 60 1 all

test-spatial:
	@echo "🗺️  Running spatial tests (30s timeout)..."
	./scripts/test-with-timeout.sh leptos-flow-core 30 1 spatial

test-proptest:
	@echo "🎲 Running proptest (45s timeout)..."
	./scripts/test-with-timeout.sh leptos-flow-core 45 1 proptest

test-layout:
	@echo "📐 Running layout tests (30s timeout)..."
	./scripts/test-with-timeout.sh leptos-flow-core 30 1 layout

# Code quality commands
fix-format:
	@echo "🎨 Fixing code formatting..."
	cargo fmt --all

fix-lints:
	@echo "🔧 Fixing clippy warnings..."
	cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged

# Test coverage
test-coverage:
	@echo "📊 Generating test coverage report..."
	cargo tarpaulin -p leptos-flow-core --out Html --out Xml --output-dir coverage-core --timeout 120

# Clean up
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf coverage-core/
	rm -rf target/

# Development workflow
dev-setup: install-nextest
	@echo "🚀 Setting up development environment..."
	cargo fmt --all
	cargo clippy --all-targets --all-features

# CI/CD commands
ci-test: install-nextest
	@echo "🔄 Running CI test suite..."
	cargo nextest run --profile default --workspace

ci-coverage: install-nextest
	@echo "📈 Running CI coverage..."
	cargo tarpaulin --workspace --out Html --out Xml --output-dir coverage --timeout 120
