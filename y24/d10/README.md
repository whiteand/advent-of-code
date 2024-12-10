# Year 24 Day 10

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d10",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d10-solve:
    cat ./y24/d10/input.txt | cargo run --release --package y24d10
y24d10-example:
    cat ./y24/d10/example.txt | cargo run --release --package y24d10
y24d10-test:
    cargo test --package y24d10 --lib -- --nocapture tests
y24d10-bench:
    cargo bench --package y24d10
```