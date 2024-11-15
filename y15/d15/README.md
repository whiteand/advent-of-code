# Year 15 Day 15

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d15",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d15-solve:
    cat ./y15/d15/input.txt | cargo run --release --package y15d15
y15d15-example:
    cat ./y15/d15/example.txt | cargo run --release --package y15d15
y15d15-test:
    cargo test --package y15d15 --lib -- --nocapture tests
y15d15-bench:
    cargo bench --package y15d15
```