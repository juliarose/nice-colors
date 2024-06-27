use criterion::{criterion_group, criterion_main, Criterion};
use nice_colors::Color;

fn criterion_benchmark(c: &mut Criterion) {
    let color = Color { red: 255, blue: 0, green: 0 };
    
    c.bench_function("Darkens color", |b| b.iter(||
        color.darken(0.5)
    ));
    
    c.bench_function("Lightens color", |b| b.iter(||
        color.lighten(0.5)
    ));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}

criterion_main!(benches);