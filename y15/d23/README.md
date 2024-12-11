# Year 15 Day 23

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d23",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d23-solve:
    cat ./y15/d23/input.txt | cargo run --release --package y15d23
y15d23-example:
    cat ./y15/d23/example.txt | cargo run --release --package y15d23
y15d23-test:
    cargo test --package y15d23 --lib -- --nocapture tests
y15d23-bench:
    cargo bench --package y15d23
```