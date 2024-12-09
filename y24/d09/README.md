# Year 24 Day 09

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d09",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d09-solve:
    cat ./y24/d09/input.txt | cargo run --release --package y24d09
y24d09-example:
    cat ./y24/d09/example.txt | cargo run --release --package y24d09
y24d09-test:
    cargo test --package y24d09 --lib -- --nocapture tests
y24d09-bench:
    cargo bench --package y24d09
```