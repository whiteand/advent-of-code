# Year 15 Day 25

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d25",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d25-solve:
    cat ./y15/d25/input.txt | cargo run --release --package y15d25
y15d25-example:
    cat ./y15/d25/example.txt | cargo run --release --package y15d25
y15d25-test:
    cargo test --package y15d25 --lib -- --nocapture tests
y15d25-bench:
    cargo bench --package y15d25
```