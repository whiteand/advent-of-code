
default:
    just --list

generate year day:
    /Users/whiteand/.cargo/bin/cargo generate --init --path ./template -d year={{year}} -d day={{day}} --name y{{year}}d{{day}}
    /opt/homebrew/bin/nu fetch.nu 20{{year}} {{day}};
    /opt/homebrew/bin/deno --allow-read --allow-write ./scripts/add-member.ts {{year}} {{day}};
    /usr/local/bin/code ./y{{year}}/d{{day}}/src/day{{day}}.rs
    /usr/local/bin/code ./y{{year}}/d{{day}}/example.txt

regenerate year day:
    rm -rf ./y{{year}}/d{{day}}
    just generate {{year}} {{day}}

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

example year day:
    cat ./y{{year}}/d{{day}}/example.txt | cargo run --release --package y{{year}}d{{day}}

test year day *PARAMS:
    cargo test --package y{{year}}d{{day}} --lib -- --nocapture tests {{PARAMS}}

test-watch year day:
    cargo watch test --package y{{year}}d{{day}} --lib -- --nocapture tests

bench year day:
    cargo bench --package y{{year}}d{{day}}

bacon year day *PARAMS:
    bacon {{PARAMS}} -- -p y{{year}}d{{day}}


t year day test:
    cargo test --package y{{year}}d{{day}} --lib -- tests::{{test}} --exact --show-output

test-p1-example year day:
    cargo test --package y{{year}}d{{day}} --lib -- tests::test_part1 --exact --show-output
test-p1 year day:
    cargo test --package y{{year}}d{{day}} --lib -- tests::test_part1_actual --exact --show-output

test-p2-example year day:
    cargo test --package y{{year}}d{{day}} --lib -- tests::test_part2 --exact --show-output
test-p2 year day:
    cargo test --package y{{year}}d{{day}} --lib -- tests::test_part2_actual --exact --show-output

fuzz *PARAMS:
    cargo +nightly fuzz {{PARAMS}}

fuzz-10s TARGET:
    just fuzz run {{TARGET}} -- -max_total_time=10

edit:
    code ./justfile

