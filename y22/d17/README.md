# Year 22 Day 17

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d17",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d17-solve:
    cat ./y22/d17/input.txt | cargo run --release --package y22d17
y22d17-example:
    cat ./y22/d17/example.txt | cargo run --release --package y22d17
y22d17-test:
    cargo test --package y22d17 --lib -- --nocapture tests
y22d17-bench:
    cargo bench --package y22d17
```