# Year 23 Day 19

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d19",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d19-solve:
    cat ./y23/d19/input.txt | cargo run --release --package y23d19
y23d19-example:
    cat ./y23/d19/example.txt | cargo run --release --package y23d19
y23d19-test:
    cargo test --package y23d19 --lib -- tests
y23d19-bench:
    cargo bench --package y23d19
```