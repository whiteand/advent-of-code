# Year 22 Day 01

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d01",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d01-solve:
    cat ./y22/d01/input.txt | cargo run --release --package y22d01
y22d01-example:
    cat ./y22/d01/example.txt | cargo run --release --package y22d01
y22d01-test:
    cargo test --package y22d01 --lib -- --nocapture tests
y22d01-bench:
    cargo bench --package y22d01
```