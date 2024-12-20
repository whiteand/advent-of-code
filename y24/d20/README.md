# Year 24 Day 20

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d20",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d20-solve:
    cat ./y24/d20/input.txt | cargo run --release --package y24d20
y24d20-example:
    cat ./y24/d20/example.txt | cargo run --release --package y24d20
y24d20-test:
    cargo test --package y24d20 --lib -- --nocapture tests
y24d20-bench:
    cargo bench --package y24d20
```