# Year 15 Day 01

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d01",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d01-solve:
    cat ./y15/d01/input.txt | cargo run --release --package y15d01
y15d01-example:
    cat ./y15/d01/example.txt | cargo run --release --package y15d01
y15d01-test:
    cargo test --package y15d01 --lib -- --nocapture tests
y15d01-bench:
    cargo bench --package y15d01
```