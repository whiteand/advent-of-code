# Year 15 Day 10

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d10",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d10-solve:
    cat ./y15/d10/input.txt | cargo run --release --package y15d10
y15d10-example:
    cat ./y15/d10/example.txt | cargo run --release --package y15d10
y15d10-test:
    cargo test --package y15d10 --lib -- --nocapture tests
y15d10-bench:
    cargo bench --package y15d10
```