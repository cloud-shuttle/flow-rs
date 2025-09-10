# Development Guide

This guide covers the development setup, pre-commit hooks, and testing infrastructure for Leptos Flow.

## 🚀 Quick Setup

Run the setup script to install all development dependencies:

```bash
./setup-hooks.sh
```

This will install:
- Pre-commit hooks for code quality
- Playwright for E2E testing
- Rust development tools
- Node.js dependencies

## 🔧 Pre-commit Hooks

We use pre-commit hooks to ensure code quality and consistency. The hooks run automatically on every commit and include:

### Rust Hooks
- **`cargo fmt`** - Auto-format Rust code
- **`cargo clippy`** - Lint for common issues and warnings
- **`cargo check`** - Basic compilation check
- **`cargo test`** - Run all tests
- **`cargo audit`** - Security vulnerability check

### File Quality Hooks
- **Trailing whitespace** - Remove trailing whitespace
- **End of file fixer** - Ensure files end with newline
- **YAML/TOML/JSON** - Validate syntax
- **Large files** - Prevent large files from being committed
- **Merge conflicts** - Check for merge conflict markers

### Markdown Hooks
- **Markdown linting** - Lint and fix markdown files

### WASM Hooks
- **WASM build check** - Verify WASM compilation
- **Playwright E2E tests** - Run end-to-end tests

## 🎭 Playwright E2E Testing

We use Playwright for comprehensive end-to-end testing of our WASM application.

### Running E2E Tests

```bash
# Run all E2E tests
npm run test:e2e

# Run with UI (interactive mode)
npm run test:e2e:ui

# Run in headed mode (see browser)
npm run test:e2e:headed

# Debug mode
npm run test:e2e:debug
```

### E2E Test Structure

```
tests/e2e/
├── global-setup.ts      # Global test setup
├── global-teardown.ts   # Global test cleanup
└── flow-editor.spec.ts  # Main E2E tests
```

### Test Coverage

Our E2E tests cover:
- ✅ Application loading and WASM module initialization
- ✅ Canvas rendering and visual output
- ✅ Mouse interactions (click, drag, pan)
- ✅ Responsive design and mobile compatibility
- ✅ Performance under load
- ✅ Error handling and console output

## 🛠️ Development Commands

### Rust Development
```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all-targets --all-features

# Check compilation
cargo check --all-targets --all-features

# Security audit
cargo audit
```

### WASM Development
```bash
# Build WASM module (development)
npm run build:wasm

# Build WASM module (release)
npm run build:wasm:release

# Start development server
npm run serve

# Development with auto-reload
npm run dev
```

### Pre-commit Commands
```bash
# Run all hooks on all files
pre-commit run --all-files

# Run specific hook
pre-commit run <hook-id>

# Update hook versions
pre-commit autoupdate

# Install hooks
pre-commit install
```

## 📋 Pre-commit Configuration

The pre-commit configuration is in `.pre-commit-config.yaml` and includes:

### Rust Hooks (from doublify/pre-commit-rust)
- `fmt` - Code formatting
- `clippy` - Linting
- `check` - Compilation check
- `test` - Test execution
- `audit` - Security audit

### General Hooks (from pre-commit-hooks)
- File quality checks
- Syntax validation
- Large file prevention

### Local Hooks
- `playwright-e2e` - E2E testing
- `wasm-check` - WASM compilation

## 🚨 Troubleshooting

### Pre-commit Issues

**Hook fails with "command not found":**
```bash
# Reinstall hooks
pre-commit uninstall
pre-commit install
```

**Rust toolchain issues:**
```bash
# Update Rust toolchain
rustup update
rustup component add rustfmt clippy
```

**WASM build issues:**
```bash
# Install wasm-pack
cargo install wasm-pack
```

### Playwright Issues

**Browser installation issues:**
```bash
# Reinstall browsers
npx playwright install --force
```

**Test server issues:**
```bash
# Check if port 8080 is available
lsof -i :8080
# Kill process if needed
kill -9 <PID>
```

## 🔄 CI/CD Integration

Our GitHub Actions workflow automatically runs:

1. **Pre-commit hooks** - Code quality checks
2. **Rust tests** - Unit and integration tests
3. **WASM tests** - Browser-based tests
4. **Playwright E2E tests** - End-to-end testing
5. **Security audit** - Vulnerability scanning
6. **Performance benchmarks** - Performance regression detection

## 📊 Code Quality Metrics

We track:
- **Test coverage** - Unit, integration, and E2E tests
- **Code formatting** - Consistent style
- **Linting** - Code quality and best practices
- **Security** - Vulnerability scanning
- **Performance** - Benchmark tracking
- **Bundle size** - WASM module size monitoring

## 🎯 Best Practices

### Before Committing
1. Run `cargo fmt` to format code
2. Run `cargo clippy` to check for issues
3. Run `cargo test` to ensure tests pass
4. Run `npm run test:e2e` for E2E tests

### Code Style
- Follow Rust naming conventions
- Use meaningful variable names
- Add documentation for public APIs
- Write tests for new functionality

### Performance
- Monitor WASM bundle size
- Profile performance-critical code
- Use appropriate data structures
- Optimize for 60 FPS rendering

## 📚 Additional Resources

- [Pre-commit Documentation](https://pre-commit.com/)
- [Playwright Documentation](https://playwright.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [WASM Book](https://rustwasm.github.io/docs/book/)
- [Leptos Documentation](https://leptos.dev/)
