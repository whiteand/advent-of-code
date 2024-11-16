# Year 15 Day 18

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d18",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d18-solve:
    cat ./y15/d18/input.txt | cargo run --release --package y15d18
y15d18-example:
    cat ./y15/d18/example.txt | cargo run --release --package y15d18
y15d18-test:
    cargo test --package y15d18 --lib -- --nocapture tests
y15d18-bench:
    cargo bench --package y15d18
```