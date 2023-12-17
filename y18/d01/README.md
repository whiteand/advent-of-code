# Year 18 Day 01

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y18/d01",
]
```

- [ ] Add shortcuts to the justfile:

```
y18d01-solve:
    cat ./y18/d01/input.txt | cargo run --package y18d01
y18d01-example:
    cat ./y18/d01/example.txt | cargo run --package y18d01
y18d01-test:
    cargo test --package y18d01 --lib -- tests --nocapture
y18d01-bench:
    cargo bench --package y18d01
```