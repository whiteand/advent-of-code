# Year 15 Day 17

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d17",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d17-solve:
    cat ./y15/d17/input.txt | cargo run --release --package y15d17
y15d17-example:
    cat ./y15/d17/example.txt | cargo run --release --package y15d17
y15d17-test:
    cargo test --package y15d17 --lib -- --nocapture tests
y15d17-bench:
    cargo bench --package y15d17
```