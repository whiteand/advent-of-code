# Year 23 Day 08

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d08",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d08-solve:
    cat ./y23/d08/input.txt | cargo run --release --package y23d08
y23d08-example:
    cat ./y23/d08/example.txt | cargo run --release --package y23d08
y23d08-test:
    cargo test --package y23d08 --lib -- --nocapture tests
y23d08-bench:
    cargo bench --package y23d08
```