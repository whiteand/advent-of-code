# Year 22 Day 12

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d12",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d12-solve:
    cat ./y22/d12/input.txt | cargo run --release --package y22d12
y22d12-example:
    cat ./y22/d12/example.txt | cargo run --release --package y22d12
y22d12-test:
    cargo test --package y22d12 --lib -- --nocapture tests
y22d12-bench:
    cargo bench --package y22d12
```