# Year 16 Day 25

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d25",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d25-solve:
    cat ./y16/d25/input.txt | cargo run --release --package y16d25
y16d25-example:
    cat ./y16/d25/example.txt | cargo run --release --package y16d25
y16d25-test:
    cargo test --package y16d25 --lib -- --nocapture tests
y16d25-bench:
    cargo bench --package y16d25
```