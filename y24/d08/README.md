# Year 24 Day 08

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d08",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d08-solve:
    cat ./y24/d08/input.txt | cargo run --release --package y24d08
y24d08-example:
    cat ./y24/d08/example.txt | cargo run --release --package y24d08
y24d08-test:
    cargo test --package y24d08 --lib -- --nocapture tests
y24d08-bench:
    cargo bench --package y24d08
```