# Year 22 Day 15

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d15",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d15-solve:
    cat ./y22/d15/input.txt | cargo run --release --package y22d15
y22d15-example:
    cat ./y22/d15/example.txt | cargo run --release --package y22d15
y22d15-test:
    cargo test --package y22d15 --lib -- --nocapture tests
y22d15-bench:
    cargo bench --package y22d15
```