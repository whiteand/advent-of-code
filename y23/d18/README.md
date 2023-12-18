# Year 23 Day 18

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d18",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d18-solve:
    cat ./y23/d18/input.txt | cargo run --package y23d18
y23d18-example:
    cat ./y23/d18/example.txt | cargo run --package y23d18
y23d18-test:
    cargo test --package y23d18 --lib -- tests --nocapture
y23d18-bench:
    cargo bench --package y23d18
```