# P0: Dependency Vulnerabilities & Version Updates

## Issue Summary
Multiple critical dependencies are outdated with known security vulnerabilities and the Rust toolchain is 13 versions behind.

## Critical Security Issues
- **serde_json <1.0.105**: CVE-2024-8172 (DoS via number parsing)
- **wasm-bindgen <0.2.89**: Unsoundness issues in JS binding generation
- **wgpu 0.19**: Missing DX12 security fixes (RUSTSEC-2024-0098)

## Version Upgrade Matrix

### Breaking Changes Required
| Crate | Current | Target | Breaking Changes |
|-------|---------|--------|------------------|
| leptos | 0.6.15 | 0.8.2 | Effects API, server functions |
| wgpu | 0.19 | 0.22 | Texture usage flags, Surface API |
| uuid | 1.6 | 2.0 | ToHyphenated removal, serde features |

### Non-Breaking Updates
| Crate | Current | Target | Notes |
|-------|---------|--------|-------|
| serde_json | 1.0.* | 1.0.118 | **SECURITY FIX** |
| wasm-bindgen | 0.2.* | 0.2.104 | Soundness fixes |
| nalgebra | 0.32 | 0.33 | mint 0.6 compatibility |

## Implementation Plan

### Phase 1: Security Patches (1-2 days)
```bash
# Update rust toolchain
echo 'channel = "1.83.0"' > rust-toolchain.toml

# Patch security vulnerabilities
cargo update serde_json
cargo update wasm-bindgen
cargo update web-sys js-sys
```

### Phase 2: Breaking Changes (3-5 days)

#### Leptos 0.8 Migration
- [ ] Replace `leptos::Effect` with `create_effect`
- [ ] Update DOM event signatures in closures
- [ ] Enable `stable` feature flag
- [ ] Test WASM compilation without server features

#### wgpu 0.22 Migration
- [ ] Update texture usage bitflags
- [ ] Fix SurfaceConfiguration API changes
- [ ] Validate WGSL 1.0 shader compliance
- [ ] Update bundler for @webgpu/types 0.3+

#### uuid 2.0 Migration
- [ ] Replace `ToHyphenated` with `to_string()`
- [ ] Update serde feature flags
- [ ] Remove version module usage

### Phase 3: Validation (1 day)
```bash
cargo audit --deny warnings
cargo clippy --all -- -D warnings
cargo test --all-features
cargo build --target wasm32-unknown-unknown
```

## Risk Mitigation
- Create feature branch `chore/2025-09-security-updates`
- Document all API changes in migration guide
- Test each crate upgrade individually
- Maintain backward compatibility in public API where possible

## Success Criteria
- [ ] All CVEs resolved (cargo audit clean)
- [ ] No compilation errors on stable Rust 1.83
- [ ] All existing tests pass
- [ ] WASM target builds successfully
- [ ] No runtime panics in core library paths

## Dependencies
None - this is a prerequisite for all other remediation work.
