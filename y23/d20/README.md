# Year 23 Day 20

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d20",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d20-solve:
    cat ./y23/d20/input.txt | cargo run --release --package y23d20
y23d20-example:
    cat ./y23/d20/example.txt | cargo run --release --package y23d20
y23d20-test:
    cargo test --package y23d20 --lib -- tests
y23d20-bench:
    cargo bench --package y23d20
```