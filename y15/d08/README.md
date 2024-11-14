# Year 15 Day 08

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d08",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d08-solve:
    cat ./y15/d08/input.txt | cargo run --release --package y15d08
y15d08-example:
    cat ./y15/d08/example.txt | cargo run --release --package y15d08
y15d08-test:
    cargo test --package y15d08 --lib -- --nocapture tests
y15d08-bench:
    cargo bench --package y15d08
```