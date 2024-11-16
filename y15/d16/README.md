# Year 15 Day 16

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d16",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d16-solve:
    cat ./y15/d16/input.txt | cargo run --release --package y15d16
y15d16-example:
    cat ./y15/d16/example.txt | cargo run --release --package y15d16
y15d16-test:
    cargo test --package y15d16 --lib -- --nocapture tests
y15d16-bench:
    cargo bench --package y15d16
```