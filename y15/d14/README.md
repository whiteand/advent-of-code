# Year 15 Day 14

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d14",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d14-solve:
    cat ./y15/d14/input.txt | cargo run --release --package y15d14
y15d14-example:
    cat ./y15/d14/example.txt | cargo run --release --package y15d14
y15d14-test:
    cargo test --package y15d14 --lib -- --nocapture tests
y15d14-bench:
    cargo bench --package y15d14
```