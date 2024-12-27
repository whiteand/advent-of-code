# Year 16 Day 21

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d21",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d21-solve:
    cat ./y16/d21/input.txt | cargo run --release --package y16d21
y16d21-example:
    cat ./y16/d21/example.txt | cargo run --release --package y16d21
y16d21-test:
    cargo test --package y16d21 --lib -- --nocapture tests
y16d21-bench:
    cargo bench --package y16d21
```