# Year 22 Day 21

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d21",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d21-solve:
    cat ./y22/d21/input.txt | cargo run --release --package y22d21
y22d21-example:
    cat ./y22/d21/example.txt | cargo run --release --package y22d21
y22d21-test:
    cargo test --package y22d21 --lib -- --nocapture tests
y22d21-bench:
    cargo bench --package y22d21
```