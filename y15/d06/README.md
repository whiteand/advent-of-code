# Year 15 Day 06

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d06",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d06-solve:
    cat ./y15/d06/input.txt | cargo run --release --package y15d06
y15d06-example:
    cat ./y15/d06/example.txt | cargo run --release --package y15d06
y15d06-test:
    cargo test --package y15d06 --lib -- --nocapture tests
y15d06-bench:
    cargo bench --package y15d06
```