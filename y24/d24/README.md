# Year 24 Day 24

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d24",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d24-solve:
    cat ./y24/d24/input.txt | cargo run --release --package y24d24
y24d24-example:
    cat ./y24/d24/example.txt | cargo run --release --package y24d24
y24d24-test:
    cargo test --package y24d24 --lib -- --nocapture tests
y24d24-bench:
    cargo bench --package y24d24
```