# Year 21 Day 17

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y21/d17",
]
```

- [ ] Add shortcuts to the justfile:

```
y21d17-solve:
    cat ./y21/d17/input.txt | cargo run --release --package y21d17
y21d17-example:
    cat ./y21/d17/example.txt | cargo run --release --package y21d17
y21d17-test:
    cargo test --package y21d17 --lib -- --nocapture tests
y21d17-bench:
    cargo bench --package y21d17
```