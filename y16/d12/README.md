# Year 16 Day 12

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d12",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d12-solve:
    cat ./y16/d12/input.txt | cargo run --release --package y16d12
y16d12-example:
    cat ./y16/d12/example.txt | cargo run --release --package y16d12
y16d12-test:
    cargo test --package y16d12 --lib -- --nocapture tests
y16d12-bench:
    cargo bench --package y16d12
```