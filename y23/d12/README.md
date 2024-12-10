# Year 23 Day 12

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d12",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d12-solve:
    cat ./y23/d12/input.txt | cargo run --release --package y23d12
y23d12-example:
    cat ./y23/d12/example.txt | cargo run --release --package y23d12
y23d12-test:
    cargo test --package y23d12 --lib -- --nocapture tests
y23d12-bench:
    cargo bench --package y23d12
```