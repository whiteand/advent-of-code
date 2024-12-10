# Year 23 Day 11

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d11",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d11-solve:
    cat ./y23/d11/input.txt | cargo run --release --package y23d11
y23d11-example:
    cat ./y23/d11/example.txt | cargo run --release --package y23d11
y23d11-test:
    cargo test --package y23d11 --lib -- --nocapture tests
y23d11-bench:
    cargo bench --package y23d11
```