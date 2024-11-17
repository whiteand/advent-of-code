# Year 15 Day 20

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d20",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d20-solve:
    cat ./y15/d20/input.txt | cargo run --release --package y15d20
y15d20-example:
    cat ./y15/d20/example.txt | cargo run --release --package y15d20
y15d20-test:
    cargo test --package y15d20 --lib -- --nocapture tests
y15d20-bench:
    cargo bench --package y15d20
```