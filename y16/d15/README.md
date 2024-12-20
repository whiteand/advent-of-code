# Year 16 Day 15

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d15",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d15-solve:
    cat ./y16/d15/input.txt | cargo run --release --package y16d15
y16d15-example:
    cat ./y16/d15/example.txt | cargo run --release --package y16d15
y16d15-test:
    cargo test --package y16d15 --lib -- --nocapture tests
y16d15-bench:
    cargo bench --package y16d15
```