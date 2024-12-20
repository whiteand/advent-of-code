# Year 16 Day 14

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d14",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d14-solve:
    cat ./y16/d14/input.txt | cargo run --release --package y16d14
y16d14-example:
    cat ./y16/d14/example.txt | cargo run --release --package y16d14
y16d14-test:
    cargo test --package y16d14 --lib -- --nocapture tests
y16d14-bench:
    cargo bench --package y16d14
```