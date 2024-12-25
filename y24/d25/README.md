# Year 24 Day 25

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d25",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d25-solve:
    cat ./y24/d25/input.txt | cargo run --release --package y24d25
y24d25-example:
    cat ./y24/d25/example.txt | cargo run --release --package y24d25
y24d25-test:
    cargo test --package y24d25 --lib -- --nocapture tests
y24d25-bench:
    cargo bench --package y24d25
```