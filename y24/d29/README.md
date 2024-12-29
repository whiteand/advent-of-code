# Year 24 Day 29

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d29",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d29-solve:
    cat ./y24/d29/input.txt | cargo run --release --package y24d29
y24d29-example:
    cat ./y24/d29/example.txt | cargo run --release --package y24d29
y24d29-test:
    cargo test --package y24d29 --lib -- --nocapture tests
y24d29-bench:
    cargo bench --package y24d29
```