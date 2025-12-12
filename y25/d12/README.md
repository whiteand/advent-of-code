# Year 25 Day 12

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y25/d12",
]
```

- [ ] Add shortcuts to the justfile:

```
y25d12-solve:
    cat ./y25/d12/input.txt | cargo run --release --package y25d12
y25d12-example:
    cat ./y25/d12/example.txt | cargo run --release --package y25d12
y25d12-test:
    cargo test --package y25d12 --lib -- --nocapture tests
y25d12-bench:
    cargo bench --package y25d12
```