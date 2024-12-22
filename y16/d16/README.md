# Year 16 Day 16

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d16",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d16-solve:
    cat ./y16/d16/input.txt | cargo run --release --package y16d16
y16d16-example:
    cat ./y16/d16/example.txt | cargo run --release --package y16d16
y16d16-test:
    cargo test --package y16d16 --lib -- --nocapture tests
y16d16-bench:
    cargo bench --package y16d16
```