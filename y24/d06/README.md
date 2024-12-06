# Year 24 Day 06

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d06",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d06-solve:
    cat ./y24/d06/input.txt | cargo run --release --package y24d06
y24d06-example:
    cat ./y24/d06/example.txt | cargo run --release --package y24d06
y24d06-test:
    cargo test --package y24d06 --lib -- --nocapture tests
y24d06-bench:
    cargo bench --package y24d06
```