# Year 23 Day 01

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d01",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d01-solve:
    cat ./y23/d01/input.txt | cargo run --release --package y23d01
y23d01-example:
    cat ./y23/d01/example.txt | cargo run --release --package y23d01
y23d01-test:
    cargo test --package y23d01 --lib -- --nocapture tests
y23d01-bench:
    cargo bench --package y23d01
```