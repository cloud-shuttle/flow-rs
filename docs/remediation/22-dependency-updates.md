# Dependency Updates (Priority 2)

## Status: OUTDATED - Multiple crates need updating

### Current Status (as of Sept 20, 2025)

**Rust Version**: ✅ **1.90.0** (current)
**Leptos**: 0.8.9 → 0.8.10 (minor update available)
**WASM-Bindgen**: 0.2.103 → 0.2.104 (patch update)
**Serde**: 1.0.225 → 1.0.228 (patch update)

### Critical Updates Required

#### 1. Leptos Ecosystem Updates
```toml
# Current → Latest
leptos = "0.8.9" → "0.8.10"
leptos_dom = "0.8.6" → "0.8.7"
leptos_macro = "0.8.8" → "0.8.9"
reactive_graph = "0.2.7" → "0.2.8"
tachys = "0.2.8" → "0.2.9"
```

#### 2. WASM Ecosystem Updates
```toml
# Current → Latest
wasm-bindgen = "0.2.103" → "0.2.104"
wasm-bindgen-futures = "0.4.53" → "0.4.54"
web-sys = "0.3.80" → "0.3.81"
js-sys = "0.3.80" → "0.3.81"
```

#### 3. Serde Updates
```toml
# Current → Latest
serde = "1.0.225" → "1.0.228"
serde_json = "1.0.145" → "1.0.228"
```

### Major Version Updates to Consider

#### Criterion (Benchmarking)
```toml
# Current → Latest
criterion = "0.5.1" → "0.7.0"  # Major version jump
```
**Impact**: Breaking changes in benchmarking API
**Risk**: High - requires benchmark code updates
**Timeline**: Defer to after critical fixes

#### Proptest (Property Testing)
```toml
# Current → Latest
proptest = "1.4" → "1.8.0"  # Major version jump
```
**Impact**: New features and API improvements
**Risk**: Medium - mostly additive changes
**Timeline**: Can update after critical fixes

### Update Strategy

#### Phase 1: Safe Updates (Immediate)
```bash
# Update patch versions (low risk)
cargo update serde
cargo update wasm-bindgen
cargo update web-sys
cargo update js-sys
```

#### Phase 2: Minor Updates (After critical fixes)
```bash
# Update Leptos ecosystem
cargo update leptos
cargo update leptos_dom
cargo update leptos_macro
```

#### Phase 3: Major Updates (After file refactoring)
```bash
# Update criterion (requires benchmark fixes)
# Update proptest (requires test updates)
```

### Breaking Changes to Prepare For

#### Leptos 0.8.10
- Minor API changes in reactive primitives
- Performance improvements in rendering
- Bug fixes in SSR/hydration

#### Serde 1.0.228
- Improved error messages
- Performance optimizations
- New derive macro features

### Testing After Updates

1. **Compilation**: Ensure all crates compile
2. **Unit Tests**: Run full test suite
3. **Integration Tests**: Verify WASM builds work
4. **E2E Tests**: Run Playwright tests
5. **Benchmarks**: Verify performance tests still work

### Rollback Plan

If updates cause issues:
```bash
# Pin to working versions
cargo update -p leptos --precise 0.8.9
cargo update -p wasm-bindgen --precise 0.2.103
```

### Timeline

- **Week 1**: Patch updates (serde, wasm-bindgen)
- **Week 2**: Leptos ecosystem updates
- **Week 3**: Major version updates (criterion, proptest)
- **Week 4**: Full testing and verification

## Priority: MEDIUM
## Estimated Time: 1 week
## Risk Level: LOW-MEDIUM (mostly patch/minor updates)
