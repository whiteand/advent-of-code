# Year 15 Day 22

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d22",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d22-solve:
    cat ./y15/d22/input.txt | cargo run --release --package y15d22
y15d22-example:
    cat ./y15/d22/example.txt | cargo run --release --package y15d22
y15d22-test:
    cargo test --package y15d22 --lib -- --nocapture tests
y15d22-bench:
    cargo bench --package y15d22
```