# Year 24 Day 11

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d11",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d11-solve:
    cat ./y24/d11/input.txt | cargo run --release --package y24d11
y24d11-example:
    cat ./y24/d11/example.txt | cargo run --release --package y24d11
y24d11-test:
    cargo test --package y24d11 --lib -- --nocapture tests
y24d11-bench:
    cargo bench --package y24d11
```