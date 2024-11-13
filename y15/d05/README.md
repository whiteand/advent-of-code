# Year 15 Day 05

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d05",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d05-solve:
    cat ./y15/d05/input.txt | cargo run --release --package y15d05
y15d05-example:
    cat ./y15/d05/example.txt | cargo run --release --package y15d05
y15d05-test:
    cargo test --package y15d05 --lib -- --nocapture tests
y15d05-bench:
    cargo bench --package y15d05
```