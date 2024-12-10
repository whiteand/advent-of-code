# Year 23 Day 16

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d16",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d16-solve:
    cat ./y23/d16/input.txt | cargo run --release --package y23d16
y23d16-example:
    cat ./y23/d16/example.txt | cargo run --release --package y23d16
y23d16-test:
    cargo test --package y23d16 --lib -- --nocapture tests
y23d16-bench:
    cargo bench --package y23d16
```