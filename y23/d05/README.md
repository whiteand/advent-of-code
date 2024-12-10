# Year 23 Day 05

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d05",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d05-solve:
    cat ./y23/d05/input.txt | cargo run --release --package y23d05
y23d05-example:
    cat ./y23/d05/example.txt | cargo run --release --package y23d05
y23d05-test:
    cargo test --package y23d05 --lib -- --nocapture tests
y23d05-bench:
    cargo bench --package y23d05
```