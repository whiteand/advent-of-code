# Year 16 Day 22

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d22",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d22-solve:
    cat ./y16/d22/input.txt | cargo run --release --package y16d22
y16d22-example:
    cat ./y16/d22/example.txt | cargo run --release --package y16d22
y16d22-test:
    cargo test --package y16d22 --lib -- --nocapture tests
y16d22-bench:
    cargo bench --package y16d22
```