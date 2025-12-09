# Year 25 Day 09

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y25/d09",
]
```

- [ ] Add shortcuts to the justfile:

```
y25d09-solve:
    cat ./y25/d09/input.txt | cargo run --release --package y25d09
y25d09-example:
    cat ./y25/d09/example.txt | cargo run --release --package y25d09
y25d09-test:
    cargo test --package y25d09 --lib -- --nocapture tests
y25d09-bench:
    cargo bench --package y25d09
```