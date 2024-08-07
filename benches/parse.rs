use criterion::{criterion_group, criterion_main, Criterion};
use nice_colors::Color;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parses FF0000", |b| b.iter(||
        Color::from_hex_str("FF0000")
    ));
    
    c.bench_function("parses #FF0000", |b| b.iter(||
        Color::from_hex_str("#FF0000")
    ));
    
    c.bench_function("parses rgb(255, 0, 0)", |b| b.iter(||
        Color::from_rgb_str("rgb(255, 0, 0)")
    ));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}

criterion_main!(benches);