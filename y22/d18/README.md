# Year 22 Day 18

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d18",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d18-solve:
    cat ./y22/d18/input.txt | cargo run --release --package y22d18
y22d18-example:
    cat ./y22/d18/example.txt | cargo run --release --package y22d18
y22d18-test:
    cargo test --package y22d18 --lib -- --nocapture tests
y22d18-bench:
    cargo bench --package y22d18
```