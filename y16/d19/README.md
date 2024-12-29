# Year 16 Day 19

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d19",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d19-solve:
    cat ./y16/d19/input.txt | cargo run --release --package y16d19
y16d19-example:
    cat ./y16/d19/example.txt | cargo run --release --package y16d19
y16d19-test:
    cargo test --package y16d19 --lib -- --nocapture tests
y16d19-bench:
    cargo bench --package y16d19
```