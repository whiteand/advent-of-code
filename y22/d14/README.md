# Year 22 Day 14

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d14",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d14-solve:
    cat ./y22/d14/input.txt | cargo run --release --package y22d14
y22d14-example:
    cat ./y22/d14/example.txt | cargo run --release --package y22d14
y22d14-test:
    cargo test --package y22d14 --lib -- --nocapture tests
y22d14-bench:
    cargo bench --package y22d14
```