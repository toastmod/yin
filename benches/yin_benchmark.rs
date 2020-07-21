use criterion::{black_box, criterion_group, criterion_main, Criterion};
use yin::compute_sample_frequency;

pub fn norm_sine_benchmark(c: &mut Criterion) {
    let sample = {
        use dasp::{signal, Signal};
        let mut signal = signal::rate(1000.0).const_hz(100.0).sine();
        let sample: Vec<f64> = (0..44100).map(|_| signal.next()).collect();
        sample
    };
    c.bench_function("1000 sr, 100.0 freq", |b| {
        b.iter(|| compute_sample_frequency(&sample, 100))
    });
}

criterion_group!(benches, norm_sine_benchmark);
criterion_main!(benches);