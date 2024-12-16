# Year 24 Day 16

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d16",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d16-solve:
    cat ./y24/d16/input.txt | cargo run --release --package y24d16
y24d16-example:
    cat ./y24/d16/example.txt | cargo run --release --package y24d16
y24d16-test:
    cargo test --package y24d16 --lib -- --nocapture tests
y24d16-bench:
    cargo bench --package y24d16
```