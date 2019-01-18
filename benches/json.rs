#[macro_use]
extern crate criterion;

use criterion::Criterion;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use varnishslog_types::HttpAccessRecord;

fn json_deserialize(lines: &[String]) -> u64 {
    lines.iter().enumerate().map(|(no, line)| {
        match serde_json::from_str::<HttpAccessRecord>(&line) {
            Err(err) => panic!("{} [{}]: {}", err, no, line),
            _ => 1
        }
    }).sum()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("json deserialize 100", move |b| {
        let test_data = File::open("log.100k.json").unwrap();
        let lines = BufReader::new(test_data).lines().map(|l| l.unwrap());
        let lines: Vec<_> = lines.take(100).collect();
        b.iter(|| json_deserialize(lines.as_slice()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);