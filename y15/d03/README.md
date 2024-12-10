# Year 15 Day 03

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d03",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d03-solve:
    cat ./y15/d03/input.txt | cargo run --release --package y15d03
y15d03-example:
    cat ./y15/d03/example.txt | cargo run --release --package y15d03
y15d03-test:
    cargo test --package y15d03 --lib -- --nocapture tests
y15d03-bench:
    cargo bench --package y15d03
```