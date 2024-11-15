# Year 15 Day 11

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d11",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d11-solve:
    cat ./y15/d11/input.txt | cargo run --release --package y15d11
y15d11-example:
    cat ./y15/d11/example.txt | cargo run --release --package y15d11
y15d11-test:
    cargo test --package y15d11 --lib -- --nocapture tests
y15d11-bench:
    cargo bench --package y15d11
```