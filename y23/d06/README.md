# Year 23 Day 06

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d06",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d06-solve:
    cat ./y23/d06/input.txt | cargo run --release --package y23d06
y23d06-example:
    cat ./y23/d06/example.txt | cargo run --release --package y23d06
y23d06-test:
    cargo test --package y23d06 --lib -- --nocapture tests
y23d06-bench:
    cargo bench --package y23d06
```