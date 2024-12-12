# Year 24 Day 12

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d12",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d12-solve:
    cat ./y24/d12/input.txt | cargo run --release --package y24d12
y24d12-example:
    cat ./y24/d12/example.txt | cargo run --release --package y24d12
y24d12-test:
    cargo test --package y24d12 --lib -- --nocapture tests
y24d12-bench:
    cargo bench --package y24d12
```