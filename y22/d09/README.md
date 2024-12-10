# Year 22 Day 09

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d09",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d09-solve:
    cat ./y22/d09/input.txt | cargo run --release --package y22d09
y22d09-example:
    cat ./y22/d09/example.txt | cargo run --release --package y22d09
y22d09-test:
    cargo test --package y22d09 --lib -- --nocapture tests
y22d09-bench:
    cargo bench --package y22d09
```