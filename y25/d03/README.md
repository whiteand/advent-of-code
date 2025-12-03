# Year 25 Day 03

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y25/d03",
]
```

- [ ] Add shortcuts to the justfile:

```
y25d03-solve:
    cat ./y25/d03/input.txt | cargo run --release --package y25d03
y25d03-example:
    cat ./y25/d03/example.txt | cargo run --release --package y25d03
y25d03-test:
    cargo test --package y25d03 --lib -- --nocapture tests
y25d03-bench:
    cargo bench --package y25d03
```