# Year 22 Day 10

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d10",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d10-solve:
    cat ./y22/d10/input.txt | cargo run --release --package y22d10
y22d10-example:
    cat ./y22/d10/example.txt | cargo run --release --package y22d10
y22d10-test:
    cargo test --package y22d10 --lib -- --nocapture tests
y22d10-bench:
    cargo bench --package y22d10
```