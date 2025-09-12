#!/bin/bash

# Test runner with timeout to prevent hanging tests
# Usage: ./scripts/test-with-timeout.sh [package] [timeout_seconds]

set -e

PACKAGE=${1:-"leptos-flow-core"}
TIMEOUT=${2:-60}
TEST_THREADS=${3:-1}

echo "🧪 Running tests for $PACKAGE with ${TIMEOUT}s timeout..."

# Function to run tests with timeout
run_tests_with_timeout() {
    local test_command="cargo test -p $PACKAGE --lib -- --test-threads=$TEST_THREADS"

    echo "Command: $test_command"
    echo "Timeout: ${TIMEOUT}s"
    echo "Threads: $TEST_THREADS"
    echo "----------------------------------------"

    # Use macOS's built-in timeout mechanism
    # Start the test process in background
    $test_command &
    local test_pid=$!

    # Wait for the process to complete or timeout
    local elapsed=0
    while [ $elapsed -lt $TIMEOUT ]; do
        if ! kill -0 $test_pid 2>/dev/null; then
            # Process has completed
            wait $test_pid
            local exit_code=$?
            echo "----------------------------------------"
            echo "✅ Tests completed in ${elapsed}s with exit code: $exit_code"
            return $exit_code
        fi

        sleep 1
        elapsed=$((elapsed + 1))

        # Show progress every 10 seconds
        if [ $((elapsed % 10)) -eq 0 ]; then
            echo "⏱️  Tests running... ${elapsed}s elapsed"
        fi
    done

    # Timeout reached
    echo "----------------------------------------"
    echo "⏰ Tests timed out after ${TIMEOUT}s"
    echo "🛑 Killing test process (PID: $test_pid)"

    # Kill the test process and all its children
    pkill -P $test_pid 2>/dev/null || true
    kill -TERM $test_pid 2>/dev/null || true

    # Wait a bit for graceful shutdown
    sleep 2

    # Force kill if still running
    if kill -0 $test_pid 2>/dev/null; then
        echo "🔨 Force killing test process"
        kill -KILL $test_pid 2>/dev/null || true
    fi

    echo "❌ Tests failed due to timeout"
    return 124  # Standard timeout exit code
}

# Function to run specific test modules with timeout
run_module_tests() {
    local module=$1
    local test_command="cargo test -p $PACKAGE --lib $module -- --test-threads=$TEST_THREADS"

    echo "🧪 Running $module tests with ${TIMEOUT}s timeout..."
    echo "Command: $test_command"
    echo "----------------------------------------"

    $test_command &
    local test_pid=$!

    local elapsed=0
    while [ $elapsed -lt $TIMEOUT ]; do
        if ! kill -0 $test_pid 2>/dev/null; then
            wait $test_pid
            local exit_code=$?
            echo "----------------------------------------"
            if [ $exit_code -eq 0 ]; then
                echo "✅ $module tests completed successfully in ${elapsed}s"
            else
                echo "❌ $module tests failed with exit code: $exit_code"
            fi
            return $exit_code
        fi

        sleep 1
        elapsed=$((elapsed + 1))
    done

    echo "⏰ $module tests timed out after ${TIMEOUT}s"
    pkill -P $test_pid 2>/dev/null || true
    kill -TERM $test_pid 2>/dev/null || true
    sleep 2
    kill -KILL $test_pid 2>/dev/null || true

    return 124
}

# Main execution
case "${4:-all}" in
    "spatial")
        run_module_tests "spatial"
        ;;
    "proptest")
        run_module_tests "proptest"
        ;;
    "layout")
        run_module_tests "layout"
        ;;
    "all")
        run_tests_with_timeout
        ;;
    *)
        echo "Usage: $0 [package] [timeout_seconds] [test_threads] [module]"
        echo "Modules: all, spatial, proptest, layout"
        echo "Example: $0 leptos-flow-core 30 1 spatial"
        exit 1
        ;;
esac
