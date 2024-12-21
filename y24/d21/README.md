# Year 24 Day 21

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d21",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d21-solve:
    cat ./y24/d21/input.txt | cargo run --release --package y24d21
y24d21-example:
    cat ./y24/d21/example.txt | cargo run --release --package y24d21
y24d21-test:
    cargo test --package y24d21 --lib -- --nocapture tests
y24d21-bench:
    cargo bench --package y24d21
```