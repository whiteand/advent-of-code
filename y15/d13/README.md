# Year 15 Day 13

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d13",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d13-solve:
    cat ./y15/d13/input.txt | cargo run --release --package y15d13
y15d13-example:
    cat ./y15/d13/example.txt | cargo run --release --package y15d13
y15d13-test:
    cargo test --package y15d13 --lib -- --nocapture tests
y15d13-bench:
    cargo bench --package y15d13
```