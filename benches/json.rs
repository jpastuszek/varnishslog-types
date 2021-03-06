use criterion::*;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use varnishslog_types::HttpAccessRecord;

fn data(n: usize) -> Vec<String> {
    let test_data = File::open("real.json").unwrap();
    let lines = BufReader::new(test_data).lines().map(|l| l.unwrap());
    lines.take(n).collect()
}

fn json_deserialize(lines: &[String], fun: impl Fn(HttpAccessRecord) -> usize) -> usize {
    lines.iter().enumerate().map(|(no, line)| {
        match serde_json::from_str::<HttpAccessRecord>(&line) {
            Err(err) => panic!("{} [{}]: {}", err, no, line),
            Ok(v) => fun(v)
        }
    }).sum()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench("json deserialize", Benchmark::new("deserialize only", move |b| {
        let lines = data(100);
        b.iter(|| json_deserialize(lines.as_slice(), |record| 
                record.as_client_request().map(|c| c.recv_total_bytes as usize).unwrap_or(0)))
    }).throughput(Throughput::Elements(100)));
    
    c.bench("json deserialize", Benchmark::new("with header access", move |b| {
        let lines = data(100);
        b.iter(|| json_deserialize(lines.as_slice(), |v| {
            v.as_client_request().and_then(|c| c.request.as_ref().and_then(|r| r.headers.as_indexed().map(|h| {
                h.get("Host").map(|h| h.len()).unwrap_or(0)
                + h.get("User-Agent").map(|h| h.len()).unwrap_or(0)
                + h.get("Accept-Encoding").map(|h| h.len()).unwrap_or(0)
            }))).unwrap_or(0)
        }))
    }).throughput(Throughput::Elements(100)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);