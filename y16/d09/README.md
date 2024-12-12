# Year 16 Day 09

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d09",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d09-solve:
    cat ./y16/d09/input.txt | cargo run --release --package y16d09
y16d09-example:
    cat ./y16/d09/example.txt | cargo run --release --package y16d09
y16d09-test:
    cargo test --package y16d09 --lib -- --nocapture tests
y16d09-bench:
    cargo bench --package y16d09
```