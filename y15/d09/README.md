# Year 15 Day 09

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d09",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d09-solve:
    cat ./y15/d09/input.txt | cargo run --release --package y15d09
y15d09-example:
    cat ./y15/d09/example.txt | cargo run --release --package y15d09
y15d09-test:
    cargo test --package y15d09 --lib -- --nocapture tests
y15d09-bench:
    cargo bench --package y15d09
```