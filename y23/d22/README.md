# Year 23 Day 22

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d22",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d22-solve:
    cat ./y23/d22/input.txt | cargo run --release --package y23d22
y23d22-example:
    cat ./y23/d22/example.txt | cargo run --release --package y23d22
y23d22-test:
    cargo test --package y23d22 --lib -- --nocapture tests
y23d22-bench:
    cargo bench --package y23d22
```