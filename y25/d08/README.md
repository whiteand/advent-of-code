# Year 25 Day 08

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y25/d08",
]
```

- [ ] Add shortcuts to the justfile:

```
y25d08-solve:
    cat ./y25/d08/input.txt | cargo run --release --package y25d08
y25d08-example:
    cat ./y25/d08/example.txt | cargo run --release --package y25d08
y25d08-test:
    cargo test --package y25d08 --lib -- --nocapture tests
y25d08-bench:
    cargo bench --package y25d08
```