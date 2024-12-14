# Year 24 Day 14

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d14",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d14-solve:
    cat ./y24/d14/input.txt | cargo run --release --package y24d14
y24d14-example:
    cat ./y24/d14/example.txt | cargo run --release --package y24d14
y24d14-test:
    cargo test --package y24d14 --lib -- --nocapture tests
y24d14-bench:
    cargo bench --package y24d14
```