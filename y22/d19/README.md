# Year 22 Day 19

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d19",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d19-solve:
    cat ./y22/d19/input.txt | cargo run --release --package y22d19
y22d19-example:
    cat ./y22/d19/example.txt | cargo run --release --package y22d19
y22d19-test:
    cargo test --package y22d19 --lib -- --nocapture tests
y22d19-bench:
    cargo bench --package y22d19
```