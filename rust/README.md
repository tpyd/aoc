# Advent of Code in Rust

## Setup a new day
- Save the input as `input/year_<year>_day_<day>.rs`
- Copy `src/template.rs` into `src/year_<year>_day_<day>.rs` and fill in missing values
- Go to `src/main.rs`, add the day as a module and add it to the `match` expression
- Go to `src/lib.rs` and add it as a public module to make it visible to Criterion
- Copy `benches/template.rs` into `benches/year_<year>_day_<day>.rs` and fill in missing values
- Go to `Cargo.toml` and add a separate benchmark for the day 

## Solve
`cargo run <year> <day>`

## Run tests
`cargo test --release -- year_<year>_day_<day>`

## Benchmark
`cargo bench --bench year_<year>_day_<day>`

