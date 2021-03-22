use criterion::{black_box, criterion_group, criterion_main, Criterion};

use isin;

// const ISIN_STRINGS: [&str; 3] = [
//     "AA0000000005", // The least taxing input for the functional style because digit expansion is rarely needed
//     "US0378331005", // A typical input (this is the payload for the Apple (AAPL) commons stock ISIN)
//     "ZZZZZZZZZZZ5", // The most taxing input for the functional style because digit expansion is maximized
// ];

fn bench_parses(c: &mut Criterion) {
    c.bench_function("x", |b| {
        b.iter(|| isin::parse(black_box("US0378331005")))
    });
}

criterion_group!(benches, bench_parses);
criterion_main!(benches);
