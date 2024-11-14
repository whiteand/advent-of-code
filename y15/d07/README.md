# Year 15 Day 07

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d07",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d07-solve:
    cat ./y15/d07/input.txt | cargo run --release --package y15d07
y15d07-example:
    cat ./y15/d07/example.txt | cargo run --release --package y15d07
y15d07-test:
    cargo test --package y15d07 --lib -- --nocapture tests
y15d07-bench:
    cargo bench --package y15d07
```