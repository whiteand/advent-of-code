
default:
    just --list

generate year day:
    /Users/whiteand/.cargo/bin/cargo generate --init --path ./template -d year={{year}} -d day={{day}} --name y{{year}}d{{day}}
    /opt/homebrew/bin/nu fetch.nu 20{{year}} {{day}};
    /opt/homebrew/bin/deno --allow-read --allow-write ./scripts/add-member.ts {{year}} {{day}};
    /usr/local/bin/code ./y{{year}}/d{{day}}/example.txt
    /usr/local/bin/code ./y{{year}}/d{{day}}/src/lib.rs

clippy:
    cargo clippy --workspace --profile ci --locked --benches --tests --all-features --no-deps

clear: clear-trace
    rm -rf ./**/target

clear-trace:
   rm -rf ./trace-*.json

solve year day:
    cat ./y{{year}}/d{{day}}/input.txt | cargo run --release --package y{{year}}d{{day}}

example year day:
    cat ./y{{year}}/d{{day}}/example.txt | cargo run --release --package y{{year}}d{{day}}

test year day:
    cargo test --package y{{year}}d{{day}} --lib -- --nocapture tests

test-watch year day:
    cargo watch test --package y{{year}}d{{day}} --lib -- --nocapture tests

bench year day:
    cargo bench --package y{{year}}d{{day}}

# test-watch but with specified test
tw year day test:
    cargo watch test --package y{{year}}d{{day}} --lib -- tests::{{test}} --exact --show-output
t year day test:
    cargo test --package y{{year}}d{{day}} --lib -- tests::{{test}} --exact --show-output
test-p1-example-watch year day:
    cargo watch test --package y{{year}}d{{day}} --lib -- tests::test_part1 --exact --show-output
test-p1-example year day:
    cargo test --package y{{year}}d{{day}} --lib -- tests::test_part1 --exact --show-output
test-p1 year day:
    cargo test --package y{{year}}d{{day}} --lib -- tests::test_part1_actual --exact --show-output
test-p2-example-watch year day:
    cargo watch test --package y{{year}}d{{day}} --lib -- tests::test_part2 --exact --show-output
test-p2-example year day:
    cargo test --package y{{year}}d{{day}} --lib -- tests::test_part2 --exact --show-output
test-p2 year day:
    cargo test --package y{{year}}d{{day}} --lib -- tests::test_part2_actual --exact --show-output
test-p2-watch year day:
    cargo watch test --package y{{year}}d{{day}} --lib -- tests::test_part2_actual --exact --show-output