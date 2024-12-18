# Year 24 Day 18

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d18",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d18-solve:
    cat ./y24/d18/input.txt | cargo run --release --package y24d18
y24d18-example:
    cat ./y24/d18/example.txt | cargo run --release --package y24d18
y24d18-test:
    cargo test --package y24d18 --lib -- --nocapture tests
y24d18-bench:
    cargo bench --package y24d18
```