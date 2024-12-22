# Year 24 Day 22

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d22",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d22-solve:
    cat ./y24/d22/input.txt | cargo run --release --package y24d22
y24d22-example:
    cat ./y24/d22/example.txt | cargo run --release --package y24d22
y24d22-test:
    cargo test --package y24d22 --lib -- --nocapture tests
y24d22-bench:
    cargo bench --package y24d22
```