# Year 25 Day 05

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y25/d05",
]
```

- [ ] Add shortcuts to the justfile:

```
y25d05-solve:
    cat ./y25/d05/input.txt | cargo run --release --package y25d05
y25d05-example:
    cat ./y25/d05/example.txt | cargo run --release --package y25d05
y25d05-test:
    cargo test --package y25d05 --lib -- --nocapture tests
y25d05-bench:
    cargo bench --package y25d05
```