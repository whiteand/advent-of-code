# Year 24 Day 13

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d13",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d13-solve:
    cat ./y24/d13/input.txt | cargo run --release --package y24d13
y24d13-example:
    cat ./y24/d13/example.txt | cargo run --release --package y24d13
y24d13-test:
    cargo test --package y24d13 --lib -- --nocapture tests
y24d13-bench:
    cargo bench --package y24d13
```