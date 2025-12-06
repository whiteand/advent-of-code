# Year 25 Day 06

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y25/d06",
]
```

- [ ] Add shortcuts to the justfile:

```
y25d06-solve:
    cat ./y25/d06/input.txt | cargo run --release --package y25d06
y25d06-example:
    cat ./y25/d06/example.txt | cargo run --release --package y25d06
y25d06-test:
    cargo test --package y25d06 --lib -- --nocapture tests
y25d06-bench:
    cargo bench --package y25d06
```