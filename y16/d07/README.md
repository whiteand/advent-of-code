# Year 16 Day 07

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d07",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d07-solve:
    cat ./y16/d07/input.txt | cargo run --release --package y16d07
y16d07-example:
    cat ./y16/d07/example.txt | cargo run --release --package y16d07
y16d07-test:
    cargo test --package y16d07 --lib -- --nocapture tests
y16d07-bench:
    cargo bench --package y16d07
```