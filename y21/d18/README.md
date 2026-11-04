# Year 21 Day 18

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y21/d18",
]
```

- [ ] Add shortcuts to the justfile:

```
y21d18-solve:
    cat ./y21/d18/input.txt | cargo run --release --package y21d18
y21d18-example:
    cat ./y21/d18/example.txt | cargo run --release --package y21d18
y21d18-test:
    cargo test --package y21d18 --lib -- --nocapture tests
y21d18-bench:
    cargo bench --package y21d18
```