# Year 24 Day 15

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d15",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d15-solve:
    cat ./y24/d15/input.txt | cargo run --release --package y24d15
y24d15-example:
    cat ./y24/d15/example.txt | cargo run --release --package y24d15
y24d15-test:
    cargo test --package y24d15 --lib -- --nocapture tests
y24d15-bench:
    cargo bench --package y24d15
```