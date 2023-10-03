use criterion::{black_box, criterion_group, criterion_main, Criterion};

const ISIN_STRINGS: [&str; 3] = [
    "AA0000000005", // The least taxing input for the functional style because digit expansion is rarely needed
    "US0378331005", // A typical input (this is the payload for the Apple (AAPL) commons stock ISIN)
    "ZZZZZZZZZZZ5", // The most taxing input for the functional style because digit expansion is maximized
];

fn bench_parses(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parse");

    for p in ISIN_STRINGS.iter() {
        group.bench_function(*p, |b| b.iter(|| isin::parse(black_box(p))));
    }

    group.finish();
}

criterion_group!(benches, bench_parses);
criterion_main!(benches);
