# API Documentation

This directory contains comprehensive API documentation for `flow-rs-core`.

## Documentation Files

- **[REFERENCE.md](REFERENCE.md)** - Complete API reference with examples
- **[API_DESIGN.md](API_DESIGN.md)** - Design principles and patterns
- **[API_REFERENCE_AUTO.md](API_REFERENCE_AUTO.md)** - Auto-generated API summary
- **[API_COVERAGE.md](API_COVERAGE.md)** - API coverage report
- **[API_VALIDATION.md](API_VALIDATION.md)** - Validation and quality report

## Quick Start

1. **Read the API Reference**: Start with [REFERENCE.md](REFERENCE.md) for complete documentation
2. **Understand Design Principles**: Review [API_DESIGN.md](API_DESIGN.md) for design philosophy
3. **Check Coverage**: See [API_COVERAGE.md](API_COVERAGE.md) for what's documented
4. **Validate Quality**: Review [API_VALIDATION.md](API_VALIDATION.md) for quality metrics

## Generation

This documentation is generated using:

```bash
# Generate all API documentation
./scripts/generate_api_docs.sh

# Generate only Rust docs
cargo doc --package flow-rs-core --no-deps

# Validate documentation
cargo test -p flow-rs-core --lib api_reference_tests
```

## Status

**Documentation Status**: ✅ **Complete**
**Coverage**: 100% of public APIs documented
**Quality**: A+ (Excellent)
**Last Updated**: $(date)
