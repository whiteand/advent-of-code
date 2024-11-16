# Year 15 Day 19

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d19",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d19-solve:
    cat ./y15/d19/input.txt | cargo run --release --package y15d19
y15d19-example:
    cat ./y15/d19/example.txt | cargo run --release --package y15d19
y15d19-test:
    cargo test --package y15d19 --lib -- --nocapture tests
y15d19-bench:
    cargo bench --package y15d19
```