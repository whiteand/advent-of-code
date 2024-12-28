# Year 16 Day 24

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d24",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d24-solve:
    cat ./y16/d24/input.txt | cargo run --release --package y16d24
y16d24-example:
    cat ./y16/d24/example.txt | cargo run --release --package y16d24
y16d24-test:
    cargo test --package y16d24 --lib -- --nocapture tests
y16d24-bench:
    cargo bench --package y16d24
```