# Year 23 Day 04

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d04",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d04-solve:
    cat ./y23/d04/input.txt | cargo run --release --package y23d04
y23d04-example:
    cat ./y23/d04/example.txt | cargo run --release --package y23d04
y23d04-test:
    cargo test --package y23d04 --lib -- --nocapture tests
y23d04-bench:
    cargo bench --package y23d04
```