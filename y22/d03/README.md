# Year 22 Day 03

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d03",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d03-solve:
    cat ./y22/d03/input.txt | cargo run --release --package y22d03
y22d03-example:
    cat ./y22/d03/example.txt | cargo run --release --package y22d03
y22d03-test:
    cargo test --package y22d03 --lib -- --nocapture tests
y22d03-bench:
    cargo bench --package y22d03
```