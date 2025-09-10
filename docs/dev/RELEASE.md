# Release Process for Leptos Flow

## Overview

This document outlines the release process for Leptos Flow, including version management, quality gates, and deployment procedures. We follow semantic versioning and maintain high standards for stability and backward compatibility.

## Version Strategy

### Semantic Versioning

We strictly follow [Semantic Versioning 2.0.0](https://semver.org/):

- **MAJOR.MINOR.PATCH** (e.g., 1.2.3)
- **MAJOR**: Breaking changes that require user code updates
- **MINOR**: New features that are backward compatible
- **PATCH**: Bug fixes and performance improvements

### Pre-Release Versions

For beta testing and early feedback:

- **Alpha**: `1.0.0-alpha.1` - Early development, API unstable
- **Beta**: `1.0.0-beta.1` - Feature complete, API stable, testing phase
- **Release Candidate**: `1.0.0-rc.1` - Production ready, final testing

### Version Numbering Examples

```
0.1.0-alpha.1  → Initial alpha release
0.1.0-beta.1   → First beta 
0.1.0-rc.1     → Release candidate
0.1.0          → First stable release
0.1.1          → Bug fix
0.2.0          → New features
1.0.0          → Major milestone/breaking changes
```

## Release Types

### Patch Releases (0.1.0 → 0.1.1)

**Triggers:**
- Critical bug fixes
- Security vulnerabilities
- Performance regressions
- Documentation corrections

**Timeline:** As needed, typically within 24-48 hours of issue identification

**Process:**
1. Create hotfix branch from main
2. Implement fix with tests
3. Fast-track review process
4. Release immediately upon approval

### Minor Releases (0.1.0 → 0.2.0)

**Triggers:**
- New features
- API additions (backward compatible)
- Significant performance improvements
- New renderer support

**Timeline:** Monthly or bi-monthly

**Process:**
1. Feature freeze 1 week before release
2. Beta release for testing
3. Full QA cycle
4. Stable release

### Major Releases (0.9.0 → 1.0.0)

**Triggers:**
- Breaking API changes
- Architecture redesign
- Major milestone achievements
- Minimum supported version changes

**Timeline:** Quarterly or as needed

**Process:**
1. RFC process for breaking changes
2. Migration guide preparation
3. Extended beta period (4-6 weeks)
4. Community feedback integration
5. Stable release

## Release Checklist

### Pre-Release Phase

#### 1. Code Quality Gates ✅

```bash
# All tests must pass
cargo test --all-features
cargo test --target wasm32-unknown-unknown

# No clippy warnings
cargo clippy --all-targets --all-features -- -D warnings

# Code formatting
cargo fmt --all -- --check

# Security audit
cargo audit

# Dependency check
cargo deny check
```

#### 2. Performance Validation ✅

```bash
# Run benchmark suite
cargo bench --all-features

# Compare with baseline (no regressions > 10%)
cargo bench -- --baseline previous-release

# Memory usage validation
cargo test memory_benchmarks

# Large graph performance test
cargo test --release performance_large_graphs
```

#### 3. Cross-Platform Testing ✅

```bash
# Browser compatibility
npx playwright test --browser=all

# WASM targets
wasm-pack test --headless --firefox --chrome

# Different Rust versions
rustup toolchain install 1.70.0
cargo +1.70.0 test --all-features
```

#### 4. Documentation Updates ✅

- [ ] API documentation current (`cargo doc`)
- [ ] CHANGELOG.md updated with all changes
- [ ] Migration guide (for breaking changes)
- [ ] Example applications work
- [ ] README.md updated
- [ ] Version numbers updated in all files

#### 5. Dependency Management ✅

```toml
# Update Cargo.toml versions
[workspace.package]
version = "0.2.0"  # New version

# Review dependency updates
cargo update
cargo outdated

# Security vulnerability check
cargo audit
```

### Release Execution

#### 1. Version Bump

```bash
# Update version in all Cargo.toml files
find . -name "Cargo.toml" -exec sed -i 's/version = "0.1.0"/version = "0.2.0"/g' {} \;

# Update version in documentation
sed -i 's/leptos-flow = "0.1"/leptos-flow = "0.2"/g' docs/guides/QUICK_START.md
```

#### 2. Create Release Branch

```bash
# Create release branch
git checkout -b release/v0.2.0
git add .
git commit -m "chore: bump version to 0.2.0"

# Push release branch
git push origin release/v0.2.0
```

#### 3. Build and Test Release Artifacts

```bash
# Build all crates
cargo build --release --all-features

# Test WASM build
wasm-pack build leptos-flow-wasm --target web --release

# Generate documentation
cargo doc --no-deps --all-features

# Test examples
cd examples/basic && trunk build --release
cd examples/advanced && trunk build --release
```

#### 4. Create Git Tag

```bash
# Create annotated tag
git tag -a v0.2.0 -m "Release version 0.2.0

Features:
- Added WebGPU renderer support
- Improved layout algorithm performance
- New hook-based API

Bug fixes:
- Fixed memory leak in spatial indexing
- Resolved edge connection issues

Breaking changes:
- None (backward compatible)
"

# Push tag
git push origin v0.2.0
```

#### 5. Publish to crates.io

```bash
# Publish in dependency order
cd leptos-flow-core && cargo publish
sleep 30  # Wait for crates.io to process

cd ../leptos-flow-renderer && cargo publish
sleep 30

cd ../leptos-flow-leptos && cargo publish
sleep 30

cd ../leptos-flow-wasm && cargo publish
sleep 30

cd .. && cargo publish  # Main crate last
```

#### 6. Create GitHub Release

```bash
# Install GitHub CLI if not available
gh --version || curl -sSL https://cli.github.com/packages/install.sh | sh

# Create GitHub release
gh release create v0.2.0 \
    --title "Leptos Flow v0.2.0" \
    --notes-file RELEASE_NOTES.md \
    --latest

# Upload additional assets
gh release upload v0.2.0 \
    target/wasm32-unknown-unknown/release/leptos_flow_wasm.wasm \
    target/doc.tar.gz
```

### Post-Release Phase

#### 1. Update Documentation Sites

```bash
# Deploy documentation
cargo doc --no-deps --all-features
rsync -av target/doc/ docs-server:/var/www/docs/

# Update examples
cd examples && ./deploy.sh
```

#### 2. Announcement and Communication

- [ ] Release announcement on GitHub
- [ ] Update crates.io description
- [ ] Social media announcement (Twitter, Reddit, Discord)
- [ ] Update project website
- [ ] Notify major users/contributors

#### 3. Merge Back to Main

```bash
# Merge release branch back to main
git checkout main
git merge --no-ff release/v0.2.0
git push origin main

# Clean up release branch
git branch -d release/v0.2.0
git push origin --delete release/v0.2.0
```

## Quality Gates

### Automated Gates (CI/CD)

```yaml
# .github/workflows/release.yml
name: Release Quality Gates

on:
  push:
    tags: ['v*']

jobs:
  quality-gates:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v2
        
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: clippy, rustfmt
          
      - name: Quality Gates
        run: |
          # Code quality
          cargo fmt --all -- --check
          cargo clippy --all-targets --all-features -- -D warnings
          
          # Tests
          cargo test --all-features
          cargo test --target wasm32-unknown-unknown
          
          # Security
          cargo audit
          
          # Performance benchmarks
          cargo bench --all-features
          
      - name: Build Release
        run: |
          cargo build --release --all-features
          wasm-pack build --release --target web
          
      - name: Publish to crates.io
        if: startsWith(github.ref, 'refs/tags/')
        run: |
          cargo publish --token ${{ secrets.CRATES_IO_TOKEN }}
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CRATES_IO_TOKEN }}
```

### Manual Quality Gates

#### Performance Regression Testing

```bash
# Before release, run comprehensive performance tests
./scripts/performance-regression-test.sh

# Example script content:
#!/bin/bash
set -e

echo "Running performance regression tests..."

# Baseline performance (previous version)
git checkout previous-release-tag
cargo bench --all-features -- --save-baseline previous

# Current performance
git checkout current-release-branch
cargo bench --all-features -- --baseline previous

# Check for regressions > 10%
if [ $? -ne 0 ]; then
    echo "Performance regression detected!"
    exit 1
fi

echo "Performance tests passed ✅"
```

#### Memory Leak Detection

```bash
# Run extended memory tests
cargo test memory_tests --release -- --ignored

# Run with Valgrind (if available)
valgrind --tool=memcheck --leak-check=full cargo test --release
```

#### Browser Compatibility Matrix

```bash
# Test across all supported browsers
npx playwright test --browser=chromium
npx playwright test --browser=firefox  
npx playwright test --browser=webkit

# Test different screen sizes
npx playwright test --viewport-size=1920,1080
npx playwright test --viewport-size=768,1024
npx playwright test --viewport-size=375,667
```

## Release Communication

### Release Notes Template

```markdown
# Leptos Flow v0.2.0

We're excited to announce Leptos Flow v0.2.0! This release focuses on performance improvements and developer experience enhancements.

## 🚀 New Features

- **WebGPU Renderer**: New high-performance WebGPU renderer for large graphs
- **Improved Hooks API**: Simplified hooks for common use cases
- **Layout Performance**: 3x faster layout algorithms for large graphs

## 🐛 Bug Fixes

- Fixed memory leak in spatial indexing (#123)
- Resolved edge connection issues in Firefox (#145)  
- Improved touch device support (#167)

## ⚡ Performance Improvements

- 40% faster rendering for graphs with 1000+ nodes
- Reduced memory usage by 25%
- Optimized WASM bundle size (now 15% smaller)

## 🔧 Developer Experience

- Better error messages with suggestions
- Improved TypeScript bindings
- New debugging tools and profiler integration

## 📚 Documentation

- Updated quick start guide
- New advanced examples
- Performance optimization guide

## 🔄 Migration Guide

This release is fully backward compatible. No migration needed!

## 📦 Installation

```toml
[dependencies]
leptos-flow = "0.2.0"
```

## 🙏 Contributors

Special thanks to our contributors:
- @contributor1 - WebGPU renderer implementation
- @contributor2 - Performance optimizations
- @contributor3 - Documentation improvements

Full changelog: https://github.com/leptos-flow/leptos-flow/compare/v0.1.0...v0.2.0
```

### Announcement Channels

1. **GitHub Release Page** - Detailed technical notes
2. **Crates.io** - Brief summary and key features
3. **Discord/Community** - Casual announcement with highlights
4. **Social Media** - Brief feature highlights with visuals
5. **Project Website** - Updated landing page and documentation

## Emergency Releases

### Hotfix Process

For critical security or stability issues:

1. **Immediate Response** (< 2 hours)
   - Assess severity and impact
   - Create hotfix branch from latest release tag
   - Implement minimal fix

2. **Fast-Track Testing** (< 4 hours)
   - Run critical test subset
   - Manual verification of fix
   - Security review if applicable

3. **Emergency Release** (< 8 hours)
   - Bump patch version
   - Create git tag
   - Publish to crates.io
   - GitHub security advisory if needed

4. **Communication** (< 12 hours)
   - Release announcement
   - Security disclosure if applicable
   - User notification through all channels

### Rollback Procedure

If a release causes critical issues:

```bash
# Yank problematic version from crates.io
cargo yank --version 0.2.1

# Create immediate hotfix release
git checkout v0.2.0  # Last known good version
git checkout -b hotfix/v0.2.2
# Apply minimal fix
git tag v0.2.2
cargo publish
```

## Release Metrics

### Success Criteria

- [ ] All automated tests pass
- [ ] No performance regressions > 10%
- [ ] Documentation coverage > 95%
- [ ] Example applications build successfully
- [ ] Community feedback positive
- [ ] No critical issues within 48 hours
- [ ] Download metrics show adoption

### Post-Release Monitoring

- Monitor GitHub issues for new bug reports
- Track crates.io download statistics  
- Monitor performance metrics in real deployments
- Collect community feedback via surveys
- Review crash reports and error telemetry

This comprehensive release process ensures high-quality, reliable releases while maintaining good communication with the community and providing clear upgrade paths for users.