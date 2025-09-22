//! API Contract Tests - TDD approach to lock down core interfaces
//!
//! This module contains comprehensive tests that define and validate the
//! public API contracts for leptos-flow-core. These tests serve as:
//!
//! 1. **API Documentation** - Living documentation of expected behavior
//! 2. **Stability Guarantees** - Tests that must pass for API stability
//! 3. **Regression Prevention** - Catch breaking changes early
//! 4. **Usage Examples** - Demonstrate correct API usage patterns

pub mod auto_layout;
pub mod core_types;
pub mod graph;
pub mod group;
pub mod handle;
pub mod layout;
pub mod node;
pub mod selection;
pub mod serialization;
pub mod spatial;
