# Year 23 Day 25

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d25",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d25-solve:
    cat ./y23/d25/input.txt | cargo run --release --package y23d25
y23d25-example:
    cat ./y23/d25/example.txt | cargo run --release --package y23d25
y23d25-test:
    cargo test --package y23d25 --lib -- --nocapture tests
y23d25-bench:
    cargo bench --package y23d25
```