# Year 16 Day 08

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d08",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d08-solve:
    cat ./y16/d08/input.txt | cargo run --release --package y16d08
y16d08-example:
    cat ./y16/d08/example.txt | cargo run --release --package y16d08
y16d08-test:
    cargo test --package y16d08 --lib -- --nocapture tests
y16d08-bench:
    cargo bench --package y16d08
```