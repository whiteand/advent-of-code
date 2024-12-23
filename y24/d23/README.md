# Year 24 Day 23

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d23",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d23-solve:
    cat ./y24/d23/input.txt | cargo run --release --package y24d23
y24d23-example:
    cat ./y24/d23/example.txt | cargo run --release --package y24d23
y24d23-test:
    cargo test --package y24d23 --lib -- --nocapture tests
y24d23-bench:
    cargo bench --package y24d23
```