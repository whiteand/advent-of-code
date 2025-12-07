# Year 25 Day 07

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y25/d07",
]
```

- [ ] Add shortcuts to the justfile:

```
y25d07-solve:
    cat ./y25/d07/input.txt | cargo run --release --package y25d07
y25d07-example:
    cat ./y25/d07/example.txt | cargo run --release --package y25d07
y25d07-test:
    cargo test --package y25d07 --lib -- --nocapture tests
y25d07-bench:
    cargo bench --package y25d07
```