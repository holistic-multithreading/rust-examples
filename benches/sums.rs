use criterion::{criterion_group, criterion_main, Criterion};

fn sums_benchmark_single_threaded(c: &mut Criterion) {

    c.bench_function("single_thread_thousand", |b| b.iter(|| {
        let _total_sum = holistic_threads::sum::count_sum_single_thread(1_000);
    } ));

    c.bench_function("single_thread_million", |b| b.iter(|| {
        let _total_sum = holistic_threads::sum::count_sum_single_thread(1_000_000);
    } ));

    c.bench_function("single_thread_billion", |b| b.iter(|| {
        let _total_sum = holistic_threads::sum::count_sum_single_thread(1_000_000_000);
    } ));
}

fn sums_benchmark_multi_threaded(c: &mut Criterion) {

    c.bench_function("multithreaded_thousand", |b| b.iter(|| {
        let _total_sum = holistic_threads::sum::count_sum_multithreaded(1_000);
    } ));

    c.bench_function("multithreaded_million", |b| b.iter(|| {
        let _total_sum = holistic_threads::sum::count_sum_multithreaded(1_000_000);
    } ));

    c.bench_function("multithreaded_billion", |b| b.iter(|| {
        let _total_sum = holistic_threads::sum::count_sum_multithreaded(1_000_000_000);
    } ));

}


criterion_group!(benches, sums_benchmark_single_threaded, sums_benchmark_multi_threaded);
criterion_main!(benches);
