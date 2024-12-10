# Year 22 Day 05

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d05",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d05-solve:
    cat ./y22/d05/input.txt | cargo run --release --package y22d05
y22d05-example:
    cat ./y22/d05/example.txt | cargo run --release --package y22d05
y22d05-test:
    cargo test --package y22d05 --lib -- --nocapture tests
y22d05-bench:
    cargo bench --package y22d05
```