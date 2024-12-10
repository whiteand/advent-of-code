# Year 22 Day 13

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d13",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d13-solve:
    cat ./y22/d13/input.txt | cargo run --release --package y22d13
y22d13-example:
    cat ./y22/d13/example.txt | cargo run --release --package y22d13
y22d13-test:
    cargo test --package y22d13 --lib -- --nocapture tests
y22d13-bench:
    cargo bench --package y22d13
```