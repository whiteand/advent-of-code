# Year 22 Day 04

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d04",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d04-solve:
    cat ./y22/d04/input.txt | cargo run --release --package y22d04
y22d04-example:
    cat ./y22/d04/example.txt | cargo run --release --package y22d04
y22d04-test:
    cargo test --package y22d04 --lib -- --nocapture tests
y22d04-bench:
    cargo bench --package y22d04
```