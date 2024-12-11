# Year 15 Day 24

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d24",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d24-solve:
    cat ./y15/d24/input.txt | cargo run --release --package y15d24
y15d24-example:
    cat ./y15/d24/example.txt | cargo run --release --package y15d24
y15d24-test:
    cargo test --package y15d24 --lib -- --nocapture tests
y15d24-bench:
    cargo bench --package y15d24
```