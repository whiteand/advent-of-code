# Year 21 Day 19

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y21/d19",
]
```

- [ ] Add shortcuts to the justfile:

```
y21d19-solve:
    cat ./y21/d19/input.txt | cargo run --release --package y21d19
y21d19-example:
    cat ./y21/d19/example.txt | cargo run --release --package y21d19
y21d19-test:
    cargo test --package y21d19 --lib -- --nocapture tests
y21d19-bench:
    cargo bench --package y21d19
```