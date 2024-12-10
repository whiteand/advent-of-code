# Year 23 Day 09

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d09",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d09-solve:
    cat ./y23/d09/input.txt | cargo run --release --package y23d09
y23d09-example:
    cat ./y23/d09/example.txt | cargo run --release --package y23d09
y23d09-test:
    cargo test --package y23d09 --lib -- --nocapture tests
y23d09-bench:
    cargo bench --package y23d09
```