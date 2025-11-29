# Year 22 Day 22

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d22",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d22-solve:
    cat ./y22/d22/input.txt | cargo run --release --package y22d22
y22d22-example:
    cat ./y22/d22/example.txt | cargo run --release --package y22d22
y22d22-test:
    cargo test --package y22d22 --lib -- --nocapture tests
y22d22-bench:
    cargo bench --package y22d22
```