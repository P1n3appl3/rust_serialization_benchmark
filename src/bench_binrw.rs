use binrw::{BinRead, BinWrite};
use criterion::{black_box, Criterion};

pub fn bench<T>(name: &'static str, c: &mut Criterion, data: &T)
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    const BUFFER_LEN: usize = 10_000_000;

    let mut group = c.benchmark_group(format!("{}/binrw", name));

    let mut serialize_buffer = vec![0; BUFFER_LEN];
    group.bench_function("serialize", |b| {
        b.iter(|| {
            black_box(
                data.write(black_box(serialize_buffer.as_mut_slice()))
                    .unwrap(),
            );
        })
    });

    let deserialize_buffer = Vec::with_capacity(BUFFER_LEN);

    group.bench_function("deserialize", |b| {
        b.iter(|| {
            black_box(T::read(black_box(&deserialize_buffer)).unwrap());
        })
    });

    crate::bench_size(name, "binrw", deserialize_buffer.as_slice());

    group.finish();
}