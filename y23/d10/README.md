# Year 23 Day 10

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d10",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d10-solve:
    cat ./y23/d10/input.txt | cargo run --release --package y23d10
y23d10-example:
    cat ./y23/d10/example.txt | cargo run --release --package y23d10
y23d10-test:
    cargo test --package y23d10 --lib -- --nocapture tests
y23d10-bench:
    cargo bench --package y23d10
```