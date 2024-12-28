# Year 16 Day 23

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d23",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d23-solve:
    cat ./y16/d23/input.txt | cargo run --release --package y16d23
y16d23-example:
    cat ./y16/d23/example.txt | cargo run --release --package y16d23
y16d23-test:
    cargo test --package y16d23 --lib -- --nocapture tests
y16d23-bench:
    cargo bench --package y16d23
```