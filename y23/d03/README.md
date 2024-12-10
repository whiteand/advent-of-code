# Year 23 Day 03

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d03",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d03-solve:
    cat ./y23/d03/input.txt | cargo run --release --package y23d03
y23d03-example:
    cat ./y23/d03/example.txt | cargo run --release --package y23d03
y23d03-test:
    cargo test --package y23d03 --lib -- --nocapture tests
y23d03-bench:
    cargo bench --package y23d03
```