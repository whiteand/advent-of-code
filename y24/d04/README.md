# Year 24 Day 04

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d04",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d04-solve:
    cat ./y24/d04/input.txt | cargo run --release --package y24d04
y24d04-example:
    cat ./y24/d04/example.txt | cargo run --release --package y24d04
y24d04-test:
    cargo test --package y24d04 --lib -- --nocapture tests
y24d04-bench:
    cargo bench --package y24d04
```