# Year 22 Day 11

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d11",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d11-solve:
    cat ./y22/d11/input.txt | cargo run --release --package y22d11
y22d11-example:
    cat ./y22/d11/example.txt | cargo run --release --package y22d11
y22d11-test:
    cargo test --package y22d11 --lib -- --nocapture tests
y22d11-bench:
    cargo bench --package y22d11
```