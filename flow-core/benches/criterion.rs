use criterion::{criterion_group, criterion_main, Criterion};

// This is a configuration benchmark that sets up common parameters
// and can be used to run all benchmarks with consistent settings

fn configure_criterion() -> Criterion {
    Criterion::default()
        .sample_size(100) // Number of samples to collect
        .measurement_time(std::time::Duration::from_secs(10)) // How long to measure
        .warm_up_time(std::time::Duration::from_secs(3)) // Warm-up time
        .confidence_level(0.95) // Statistical confidence level
        .significance_level(0.01) // Significance level for comparisons
        .noise_threshold(0.02) // Noise threshold for detecting changes
}

// Empty benchmark group for configuration
fn config_benchmark(_c: &mut Criterion) {
    // This is just a placeholder to set up the configuration
}

criterion_group! {
    name = config;
    config = configure_criterion();
    targets = config_benchmark
}

criterion_main!(config);
