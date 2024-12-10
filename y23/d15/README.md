# Year 23 Day 15

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d15",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d15-solve:
    cat ./y23/d15/input.txt | cargo run --release --package y23d15
y23d15-example:
    cat ./y23/d15/example.txt | cargo run --release --package y23d15
y23d15-test:
    cargo test --package y23d15 --lib -- --nocapture tests
y23d15-bench:
    cargo bench --package y23d15
```