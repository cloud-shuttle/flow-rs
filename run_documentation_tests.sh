#!/bin/bash

# Run documentation tests from the project root
cd "$(dirname "$0")"
cargo test -p leptos-flow-core --lib documentation_tests
