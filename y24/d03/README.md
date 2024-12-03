# Year 24 Day 03

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d03",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d03-solve:
    cat ./y24/d03/input.txt | cargo run --release --package y24d03
y24d03-example:
    cat ./y24/d03/example.txt | cargo run --release --package y24d03
y24d03-test:
    cargo test --package y24d03 --lib -- --nocapture tests
y24d03-bench:
    cargo bench --package y24d03
```