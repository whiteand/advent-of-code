# Year 24 Day 05

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d05",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d05-solve:
    cat ./y24/d05/input.txt | cargo run --release --package y24d05
y24d05-example:
    cat ./y24/d05/example.txt | cargo run --release --package y24d05
y24d05-test:
    cargo test --package y24d05 --lib -- --nocapture tests
y24d05-bench:
    cargo bench --package y24d05
```