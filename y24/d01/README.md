# Year 24 Day 01

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d01",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d01-solve:
    cat ./y24/d01/input.txt | cargo run --release --package y24d01
y24d01-example:
    cat ./y24/d01/example.txt | cargo run --release --package y24d01
y24d01-test:
    cargo test --package y24d01 --lib -- --nocapture tests
y24d01-bench:
    cargo bench --package y24d01
```