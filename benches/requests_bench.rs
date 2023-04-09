use criterion::{criterion_group, criterion_main, Criterion};
use rayon::prelude::{ParallelBridge, ParallelIterator};

fn make_request(client: &reqwest::blocking::Client, endpoint: &str, symbol: &str, start_date: &time::Date, end_date: &time::Date) {
    client.get(format!("http://127.0.0.1:8881/api/{endpoint}?symbol={symbol}&start_date={start_date}&end_date={end_date}")).send().unwrap();
}

fn executor(
    (endpoint, client,zipper):
    (
        &str,
        reqwest::blocking::Client,
        std::iter::Zip<
            std::iter::Cycle<std::slice::Iter<&str>>,
            std::iter::Zip<std::iter::Cycle<std::slice::Iter<time::Date>>, std::iter::Cycle<std::slice::Iter<time::Date>>>
        >
    )
) {
    zipper
        .take(5_000)
        .for_each(|(symbol, (start, end))| make_request(&client, endpoint, symbol, start, end))
}

fn executor_parallel(
    (endpoint, client,zipper):
    (
        &str,
        reqwest::blocking::Client,
        std::iter::Zip<
            std::iter::Cycle<std::slice::Iter<&str>>,
            std::iter::Zip<std::iter::Cycle<std::slice::Iter<time::Date>>, std::iter::Cycle<std::slice::Iter<time::Date>>>
        >
    )
) {
    zipper
        .take(5_000)
        .par_bridge()
        .for_each(|(symbol, (start, end))| make_request(&client, endpoint, symbol, start, end))
}

fn criterion_benchmark(c: &mut Criterion) {
    let symbols = ["IBM", "AAPL"];
    let starts: Vec<_> = (1..60).step_by(3).map(|d| time::Date::from_ordinal_date(2023, d).unwrap()).collect();
    let ends: Vec<_> = (60..90).step_by(3).map(|d| time::Date::from_ordinal_date(2023, d).unwrap()).collect();

    c.bench_function(
        "sequencial requests to `financial data`",
        |b| b.iter_batched(
            || ("financial_data", reqwest::blocking::Client::new(), std::iter::zip(symbols.iter().cycle(), std::iter::zip(starts.iter().cycle(), ends.iter().cycle()))),
            executor,
            criterion::BatchSize::SmallInput
        )
    );

    c.bench_function(
        "sequencial requests to `statistics`",
        |b| b.iter_batched(
            || ("statistics", reqwest::blocking::Client::new(), std::iter::zip(symbols.iter().cycle(), std::iter::zip(starts.iter().cycle(), ends.iter().cycle()))),
            executor,
            criterion::BatchSize::SmallInput
        )
    );

    c.bench_function(
        "parallel requests to `financial data`",
        |b| b.iter_batched(
            || ("financial_data", reqwest::blocking::Client::new(), std::iter::zip(symbols.iter().cycle(), std::iter::zip(starts.iter().cycle(), ends.iter().cycle()))),
            executor_parallel,
            criterion::BatchSize::SmallInput
        )
    );

    c.bench_function(
        "parallel requests to `statistics`",
        |b| b.iter_batched(
            || ("statistics", reqwest::blocking::Client::new(), std::iter::zip(symbols.iter().cycle(), std::iter::zip(starts.iter().cycle(), ends.iter().cycle()))),
            executor_parallel,
            criterion::BatchSize::SmallInput
        )
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);