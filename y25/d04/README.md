# Year 25 Day 04

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y25/d04",
]
```

- [ ] Add shortcuts to the justfile:

```
y25d04-solve:
    cat ./y25/d04/input.txt | cargo run --release --package y25d04
y25d04-example:
    cat ./y25/d04/example.txt | cargo run --release --package y25d04
y25d04-test:
    cargo test --package y25d04 --lib -- --nocapture tests
y25d04-bench:
    cargo bench --package y25d04
```