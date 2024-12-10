# Year 20 Day 01

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y20/d01",
]
```

- [ ] Add shortcuts to the justfile:

```
y20d01-solve:
    cat ./y20/d01/input.txt | cargo run --release --package y20d01
y20d01-example:
    cat ./y20/d01/example.txt | cargo run --release --package y20d01
y20d01-test:
    cargo test --package y20d01 --lib -- --nocapture tests
y20d01-bench:
    cargo bench --package y20d01
```