# Year 22 Day 23

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d23",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d23-solve:
    cat ./y22/d23/input.txt | cargo run --release --package y22d23
y22d23-example:
    cat ./y22/d23/example.txt | cargo run --release --package y22d23
y22d23-test:
    cargo test --package y22d23 --lib -- --nocapture tests
y22d23-bench:
    cargo bench --package y22d23
```