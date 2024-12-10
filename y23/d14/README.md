# Year 23 Day 14

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d14",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d14-solve:
    cat ./y23/d14/input.txt | cargo run --release --package y23d14
y23d14-example:
    cat ./y23/d14/example.txt | cargo run --release --package y23d14
y23d14-test:
    cargo test --package y23d14 --lib -- --nocapture tests
y23d14-bench:
    cargo bench --package y23d14
```