# Year 22 Day 16

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d16",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d16-solve:
    cat ./y22/d16/input.txt | cargo run --release --package y22d16
y22d16-example:
    cat ./y22/d16/example.txt | cargo run --release --package y22d16
y22d16-test:
    cargo test --package y22d16 --lib -- --nocapture tests
y22d16-bench:
    cargo bench --package y22d16
```