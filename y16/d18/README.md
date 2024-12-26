# Year 16 Day 18

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d18",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d18-solve:
    cat ./y16/d18/input.txt | cargo run --release --package y16d18
y16d18-example:
    cat ./y16/d18/example.txt | cargo run --release --package y16d18
y16d18-test:
    cargo test --package y16d18 --lib -- --nocapture tests
y16d18-bench:
    cargo bench --package y16d18
```