# Year 16 Day 13

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d13",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d13-solve:
    cat ./y16/d13/input.txt | cargo run --release --package y16d13
y16d13-example:
    cat ./y16/d13/example.txt | cargo run --release --package y16d13
y16d13-test:
    cargo test --package y16d13 --lib -- --nocapture tests
y16d13-bench:
    cargo bench --package y16d13
```