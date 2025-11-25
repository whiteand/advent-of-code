# Year 22 Day 20

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d20",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d20-solve:
    cat ./y22/d20/input.txt | cargo run --release --package y22d20
y22d20-example:
    cat ./y22/d20/example.txt | cargo run --release --package y22d20
y22d20-test:
    cargo test --package y22d20 --lib -- --nocapture tests
y22d20-bench:
    cargo bench --package y22d20
```