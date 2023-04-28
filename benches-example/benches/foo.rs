// use criterion_bencher_compat as bencher;
// use bencher::{benchmark_group, benchmark_main, Bencher};

// fn a(bench: &mut Bencher) {
//     bench.iter(|| {
//         (0..1000).fold(0, |x, y| x + y)
//     })
// }

// fn b(bench: &mut Bencher) {
//     const N: usize = 1024;
//     bench.iter(|| {
//         vec![0u8; N]
//     });

//     bench.bytes = N as u64;
// }

// benchmark_group!(benches, a, b);
// benchmark_main!(benches);

use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

// Here we have an async function to benchmark
async fn do_something(size: usize) {
    // Do something async with the size
    tokio::time::sleep(tokio::time::Duration::from_millis(size.try_into().unwrap())).await;
}

fn from_elem(c: &mut Criterion) {
    let size: usize = 1024;

    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_with_input(BenchmarkId::new("input_example", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.to_async(&rt).iter(|| do_something(s));
    });
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
