# Year 16 Day 11

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d11",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d11-solve:
    cat ./y16/d11/input.txt | cargo run --release --package y16d11
y16d11-example:
    cat ./y16/d11/example.txt | cargo run --release --package y16d11
y16d11-test:
    cargo test --package y16d11 --lib -- --nocapture tests
y16d11-bench:
    cargo bench --package y16d11
```