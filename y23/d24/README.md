# Year 23 Day 24

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d24",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d24-solve:
    cat ./y23/d24/input.txt | cargo run --release --package y23d24
y23d24-example:
    cat ./y23/d24/example.txt | cargo run --release --package y23d24
y23d24-test:
    cargo test --package y23d24 --lib -- --nocapture tests
y23d24-bench:
    cargo bench --package y23d24
```