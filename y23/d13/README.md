# Year 23 Day 13

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d13",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d13-solve:
    cat ./y23/d13/input.txt | cargo run --release --package y23d13
y23d13-example:
    cat ./y23/d13/example.txt | cargo run --release --package y23d13
y23d13-test:
    cargo test --package y23d13 --lib -- --nocapture tests
y23d13-bench:
    cargo bench --package y23d13
```