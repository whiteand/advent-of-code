# Year 16 Day 10

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y16/d10",
]
```

- [ ] Add shortcuts to the justfile:

```
y16d10-solve:
    cat ./y16/d10/input.txt | cargo run --release --package y16d10
y16d10-example:
    cat ./y16/d10/example.txt | cargo run --release --package y16d10
y16d10-test:
    cargo test --package y16d10 --lib -- --nocapture tests
y16d10-bench:
    cargo bench --package y16d10
```