# Advent of Code

## Setup a new day
- Save the input as `input/year_<year>_day_<day>.txt`
- Copy `src/template.rs` into `src/year_<year>_day_<day>.rs` and fill in missing values
- Go to `src/main.rs`, add the day as a module and add it to the `match` expression
- Go to `src/lib.rs` and add it as a public module to make it visible to Criterion
- Copy `benches/year_2015_day_01.rs` into `benches/year_<year>_day_<day>.rs` and replace the year and date values.
- Go to `Cargo.toml` and add a separate benchmark for the day 

## Solve
`cargo run <year> <day>`

## Run tests
`cargo test year_<year>_day_<day>`
`cargo test <year>_day_<day>`
`cargo test <year>`
`cargo test part1`

## Benchmark
`cargo bench --bench year_<year>_day_<day>`

