# Year 22 Day 07

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d07",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d07-solve:
    cat ./y22/d07/input.txt | cargo run --release --package y22d07
y22d07-example:
    cat ./y22/d07/example.txt | cargo run --release --package y22d07
y22d07-test:
    cargo test --package y22d07 --lib -- --nocapture tests
y22d07-bench:
    cargo bench --package y22d07
```