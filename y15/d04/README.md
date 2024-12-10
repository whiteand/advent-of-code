# Year 15 Day 04

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d04",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d04-solve:
    cat ./y15/d04/input.txt | cargo run --release --package y15d04
y15d04-example:
    cat ./y15/d04/example.txt | cargo run --release --package y15d04
y15d04-test:
    cargo test --package y15d04 --lib -- --nocapture tests
y15d04-bench:
    cargo bench --package y15d04
```