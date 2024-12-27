# Year 16 Day 20

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d20",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d20-solve:
    cat ./y16/d20/input.txt | cargo run --release --package y16d20
y16d20-example:
    cat ./y16/d20/example.txt | cargo run --release --package y16d20
y16d20-test:
    cargo test --package y16d20 --lib -- --nocapture tests
y16d20-bench:
    cargo bench --package y16d20
```