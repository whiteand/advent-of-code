# Year 24 Day 19

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d19",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d19-solve:
    cat ./y24/d19/input.txt | cargo run --release --package y24d19
y24d19-example:
    cat ./y24/d19/example.txt | cargo run --release --package y24d19
y24d19-test:
    cargo test --package y24d19 --lib -- --nocapture tests
y24d19-bench:
    cargo bench --package y24d19
```