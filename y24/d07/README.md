# Year 24 Day 07

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d07",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d07-solve:
    cat ./y24/d07/input.txt | cargo run --release --package y24d07
y24d07-example:
    cat ./y24/d07/example.txt | cargo run --release --package y24d07
y24d07-test:
    cargo test --package y24d07 --lib -- --nocapture tests
y24d07-bench:
    cargo bench --package y24d07
```