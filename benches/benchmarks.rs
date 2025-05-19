//! Benchmarks for zedi-gen

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io;
use zedi_gen::{config::Config, generator::Generator, population::PopulationGenerator};

fn generate_claims(c: &mut Criterion) {
    let config = Config {
        claim_count: 1000,
        anomaly_rate: 0.01, // 1% anomalies
        ..Default::default()
    };

    c.bench_function("generate_1000_claims", |b| {
        b.iter(|| {
            let mut generator = Generator::new(config.clone());
            // Direct output to sink to avoid I/O overhead during benchmarking
            generator.generate_to_writer(Box::new(io::sink())).unwrap();
        })
    });
}

fn generate_population(c: &mut Criterion) {
    c.bench_function("generate_1000_people", |b| {
        b.iter(|| {
            let mut generator = PopulationGenerator::new(Some(42));
            for _ in 0..1000 {
                black_box(generator.generate_person());
            }
        })
    });

    c.bench_function("generate_1000_providers", |b| {
        b.iter(|| {
            let mut generator = PopulationGenerator::new(Some(42));
            for _ in 0..1000 {
                black_box(generator.generate_provider());
            }
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10).without_plots();
    targets = generate_claims, generate_population
);
criterion_main!(benches);
