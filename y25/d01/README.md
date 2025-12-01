# Year 25 Day 01

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y25/d01",
]
```

- [ ] Add shortcuts to the justfile:

```
y25d01-solve:
    cat ./y25/d01/input.txt | cargo run --release --package y25d01
y25d01-example:
    cat ./y25/d01/example.txt | cargo run --release --package y25d01
y25d01-test:
    cargo test --package y25d01 --lib -- --nocapture tests
y25d01-bench:
    cargo bench --package y25d01
```