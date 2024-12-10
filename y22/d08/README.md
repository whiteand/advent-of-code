# Year 22 Day 08

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d08",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d08-solve:
    cat ./y22/d08/input.txt | cargo run --release --package y22d08
y22d08-example:
    cat ./y22/d08/example.txt | cargo run --release --package y22d08
y22d08-test:
    cargo test --package y22d08 --lib -- --nocapture tests
y22d08-bench:
    cargo bench --package y22d08
```