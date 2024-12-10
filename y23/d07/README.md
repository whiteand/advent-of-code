# Year 23 Day 07

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d07",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d07-solve:
    cat ./y23/d07/input.txt | cargo run --release --package y23d07
y23d07-example:
    cat ./y23/d07/example.txt | cargo run --release --package y23d07
y23d07-test:
    cargo test --package y23d07 --lib -- --nocapture tests
y23d07-bench:
    cargo bench --package y23d07
```