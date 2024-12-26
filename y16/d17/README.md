# Year 16 Day 17

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d17",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d17-solve:
    cat ./y16/d17/input.txt | cargo run --release --package y16d17
y16d17-example:
    cat ./y16/d17/example.txt | cargo run --release --package y16d17
y16d17-test:
    cargo test --package y16d17 --lib -- --nocapture tests
y16d17-bench:
    cargo bench --package y16d17
```