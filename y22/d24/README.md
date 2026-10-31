# Year 22 Day 24

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d24",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d24-solve:
    cat ./y22/d24/input.txt | cargo run --release --package y22d24
y22d24-example:
    cat ./y22/d24/example.txt | cargo run --release --package y22d24
y22d24-test:
    cargo test --package y22d24 --lib -- --nocapture tests
y22d24-bench:
    cargo bench --package y22d24
```