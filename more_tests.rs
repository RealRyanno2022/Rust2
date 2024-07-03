// Integration Tests
// Tests components, functions and structs together as parts of a working larger program

// End to End Tests


// A good testing suite of 1000 tests has:
// 700 unit tests
// 200 integration tests
// 100 end-to-end tests


// End to End Test
// Designed to test your entire system
// Does the backend, the frontend, APIs, database and third party services 

// Benchmark Testing
// A bit unfinished!

// We will use Criterion instead

// Criterion
// A better way to benchmark

[package]
name = "hello"
version = "0.1.0"

[dependencies]

[dev-dependencies]
criterion = { version = "0.3", feature = ["html_reports"]

[[bench]]                       // [[bench]] allows you to have
                                // multiple benchmark tests. here
                                // snuggle speed is a dependency.
                                // "harness = false" disables the
                                // built-in benchmarking functionality
                                // so that criterion can do its work.
                            
name = "snuggle_speed"
harness = false

// Use the benches dir (to be put at root) to run benchmark tests
// The filename needs to match the Cargo.toml config.
// cargo bench

hello/benches/snuggle_speed.rs


use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hello:snuggle;

pub fn snuggle_benchmark(c: &mut Criterion) {
    c.bench_function("snuggle 2", |b| b.iter(|| snuggle(black_box(2))));
}

criterion_group!(benches, snuggle_benchmark);
criterion_main!(benches);

// each bench has a warmup period of 5 seconds and a run period of 3 seconds
// The outliers are recorded - too many indicates that you're running a lot of background processes
// Don't benchmark on public CI servers because there is too much noise
// Set up a custom runner on dedicated hardware that only does one benchmark at a time


// Somehow this is 10% faster than multiplication

pub fn snuggle(bunnies: u128) -> u128 {
    let mut result = 0;
    for _ in 0..8 {
        result += bunnies
    }
    result
}

// Bitshifting ("<<") is an operation that in pure binary terms shifts all binary units 1 unit to
// the left
// In practise it's a way to multiply by the power of 2
// so x << 3 = x * 2^3
// It's expected to be the slowest out of the three but ends up being in the middle

pub fn snuggle(bunnies: u128) -> u128 {
    bunnies << 3
}































































