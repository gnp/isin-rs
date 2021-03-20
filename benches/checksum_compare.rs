use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use isin::checksum::checksum_functional;
use isin::checksum::checksum_table;

const PAYLOADS: [&str; 3] = [
    "AA000000000", // The least taxing input for the functional style because digit expansion is rarely needed
    "US037833100", // A typical input (this is the payload for the Apple (AAPL) commons stock ISIN)
    "ZZZZZZZZZZZ", // The most taxing input for the functional style because digit expansion is maximized
];

fn bench_checksums(c: &mut Criterion) {
    let mut group = c.benchmark_group("Checksum");
    for p in PAYLOADS.iter() {
        group.bench_with_input(BenchmarkId::new("Functional", p), p, |b, p| {
            b.iter(|| checksum_functional(p.as_bytes()))
        });
        group.bench_with_input(BenchmarkId::new("Table", p), p, |b, p| {
            b.iter(|| checksum_table(p.as_bytes()))
        });
    }
}

criterion_group!(benches, bench_checksums);
criterion_main!(benches);
