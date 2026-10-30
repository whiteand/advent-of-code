
default:
    just --list

download-input year day:
  node --env-file .env ./generator/bin/download-input.ts {{year}} {{day}}

generate year day:
  node --env-file .env ./generator/bin/generate.ts {{year}} {{day}}
  /usr/local/bin/cursor ./y{{year}}/d{{day}}/src/day{{day}}.rs
  /usr/local/bin/cursor ./y{{year}}/d{{day}}/example.txt

amend:
    git add .; git commit --amend --no-edit; git push -f

clippy:
    cargo clippy --workspace --profile ci --locked --benches --tests --all-features --no-deps

clear: clear-trace
    rm -rf ./**/target

clear-trace:
   rm -rf ./trace-*.json

solve year day:
    cat ./y{{year}}/d{{day}}/input.txt | cargo run --release --package y{{year}}d{{day}}

solve-log year day:
    cat ./y{{year}}/d{{day}}/input.txt | RUST_LOG=info cargo run --release --package y{{year}}d{{day}}
    
solve-example-log year day:
    cat ./y{{year}}/d{{day}}/example.txt | RUST_LOG=info cargo run --release --package y{{year}}d{{day}}

example year day:
    cat ./y{{year}}/d{{day}}/example.txt | cargo run --release --package y{{year}}d{{day}}

test year day *PARAMS:
    cargo test --package y{{year}}d{{day}} --lib -- --nocapture tests {{PARAMS}}

test-watch year day:
    cargo watch test --package y{{year}}d{{day}} --lib -- --nocapture tests

test-all:
    cargo nextest run --no-fail-fast

fmt:
    cargo fmt --all

lint-all:
    just fmt
    just clippy
    just test-all

bench year day:
    cargo bench --package y{{year}}d{{day}}

bacon year day *PARAMS:
    bacon {{PARAMS}} -- -p y{{year}}d{{day}}


t year day test:
    cargo test --package y{{year}}d{{day}} --lib -- tests::{{test}} --exact --show-output

test-p1-example year day:
    cargo test --package y{{year}}d{{day}} --lib -- day{{day}}::tests::test_part1::case_1_example --exact --show-output
test-p1 year day:
    cargo test --package y{{year}}d{{day}} --lib -- day{{day}}::tests::test_part1::case_2_actual --exact --show-output

test-p2-example year day:
    cargo test --package y{{year}}d{{day}} --lib -- day{{day}}::tests::test_part2::case_1_example --exact --nocapture
test-p2 year day:
    cargo test --package y{{year}}d{{day}} --lib -- day{{day}}::tests::test_part2 --nocapture

fuzz *PARAMS:
    cargo +nightly fuzz {{PARAMS}}

fuzz-10s TARGET:
    just fuzz run {{TARGET}} -- -max_total_time=10

edit:
    code ./justfile

