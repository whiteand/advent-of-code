# Year 25 Day 11

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y25/d11",
]
```

- [ ] Add shortcuts to the justfile:

```
y25d11-solve:
    cat ./y25/d11/input.txt | cargo run --release --package y25d11
y25d11-example:
    cat ./y25/d11/example.txt | cargo run --release --package y25d11
y25d11-test:
    cargo test --package y25d11 --lib -- --nocapture tests
y25d11-bench:
    cargo bench --package y25d11
```