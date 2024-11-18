# Year 15 Day 21

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d21",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d21-solve:
    cat ./y15/d21/input.txt | cargo run --release --package y15d21
y15d21-example:
    cat ./y15/d21/example.txt | cargo run --release --package y15d21
y15d21-test:
    cargo test --package y15d21 --lib -- --nocapture tests
y15d21-bench:
    cargo bench --package y15d21
```