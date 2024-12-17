# Year 24 Day 17

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d17",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d17-solve:
    cat ./y24/d17/input.txt | cargo run --release --package y24d17
y24d17-example:
    cat ./y24/d17/example.txt | cargo run --release --package y24d17
y24d17-test:
    cargo test --package y24d17 --lib -- --nocapture tests
y24d17-bench:
    cargo bench --package y24d17
```