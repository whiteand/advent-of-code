# Year 22 Day 06

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d06",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d06-solve:
    cat ./y22/d06/input.txt | cargo run --release --package y22d06
y22d06-example:
    cat ./y22/d06/example.txt | cargo run --release --package y22d06
y22d06-test:
    cargo test --package y22d06 --lib -- --nocapture tests
y22d06-bench:
    cargo bench --package y22d06
```