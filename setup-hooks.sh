#!/bin/bash

# Setup script for pre-commit hooks and Playwright E2E testing
# This script installs and configures the development environment

set -e

echo "🚀 Setting up Leptos Flow development environment..."

# Check if Python is available
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is required but not installed"
    exit 1
fi

# Check if Node.js is available
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is required but not installed"
    echo "Please install Node.js 18+ from https://nodejs.org/"
    exit 1
fi

# Check Node.js version
NODE_VERSION=$(node -v | cut -d'v' -f2 | cut -d'.' -f1)
if [ "$NODE_VERSION" -lt 18 ]; then
    echo "❌ Node.js 18+ is required, but you have $(node -v)"
    exit 1
fi

echo "✅ Node.js $(node -v) detected"

# Install pre-commit if not already installed
if ! command -v pre-commit &> /dev/null; then
    echo "📦 Installing pre-commit..."
    pip3 install pre-commit
else
    echo "✅ pre-commit already installed"
fi

# Install pre-commit hooks
echo "🔧 Installing pre-commit hooks..."
pre-commit install

# Install Node.js dependencies
echo "📦 Installing Node.js dependencies..."
npm install

# Install Playwright browsers
echo "🎭 Installing Playwright browsers..."
npx playwright install

# Install Rust tools if needed
echo "🦀 Checking Rust tools..."

# Check if cargo-fmt is available
if ! cargo fmt --version &> /dev/null; then
    echo "📦 Installing rustfmt..."
    rustup component add rustfmt
fi

# Check if clippy is available
if ! cargo clippy --version &> /dev/null; then
    echo "📦 Installing clippy..."
    rustup component add clippy
fi

# Check if wasm-pack is available
if ! command -v wasm-pack &> /dev/null; then
    echo "📦 Installing wasm-pack..."
    cargo install wasm-pack
fi

echo ""
echo "🎉 Setup complete!"
echo ""
echo "📋 Available commands:"
echo "  pre-commit run --all-files    # Run all hooks on all files"
echo "  npm run test:e2e             # Run Playwright E2E tests"
echo "  npm run test:e2e:ui          # Run Playwright tests with UI"
echo "  npm run build:wasm           # Build WASM module"
echo "  npm run dev                  # Start development server"
echo ""
echo "🔧 Pre-commit hooks will now run automatically on git commit"
echo "🎭 Playwright tests can be run with: npm run test:e2e"
echo ""
echo "Happy coding! 🚀"
