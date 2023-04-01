use bit_array::BitArray;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;

#[allow(clippy::missing_panics_doc)]
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("construct bit array", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(BitArray::new());
            }
        });
    });
    c.bench_function("construct bool array", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box([false; 64]);
            }
        });
    });
    c.bench_function("sum bit array", |b| {
        b.iter(|| {
            let mut sum = 0;
            for _ in 0..1000 {
                let array = black_box(BitArray::from(u64::MAX));
                sum += array.num_active();
            }
            black_box(sum);
        });
    });
    c.bench_function("sum iter bit array", |b| {
        b.iter(|| {
            let mut sum = 0;
            for _ in 0..1000 {
                let array = black_box(BitArray::from(u64::MAX));
                sum += array.into_iter().map(u64::from).sum::<u64>();
            }
            black_box(sum);
        });
    });
    c.bench_function("sum iter bool array", |b| {
        b.iter(|| {
            let mut sum = 0;
            for _ in 0..1000 {
                let array = black_box([true; 64]);
                sum += array.into_iter().map(u64::from).sum::<u64>();
            }
            black_box(sum);
        });
    });
    c.bench_function("sum loop bit array", |b| {
        b.iter(|| {
            let mut sum = 0;
            for _ in 0..1000 {
                let array = black_box(BitArray::from(u64::MAX));
                for i in 0..64 {
                    sum += usize::from(array.get_unchecked(i));
                }
            }
            black_box(sum);
        });
    });
    c.bench_function("sum loop bool array", |b| {
        b.iter(|| {
            let mut sum = 0;
            for _ in 0..1000 {
                let array = black_box([true; 64]);
                for i in 0..64 {
                    sum += usize::from(*array.get(i).unwrap());
                }
            }
            black_box(sum);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
