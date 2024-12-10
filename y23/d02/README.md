# Year 23 Day 02

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y23/d02",
]
```

- [ ] Add shortcuts to the justfile:

```
y23d02-solve:
    cat ./y23/d02/input.txt | cargo run --release --package y23d02
y23d02-example:
    cat ./y23/d02/example.txt | cargo run --release --package y23d02
y23d02-test:
    cargo test --package y23d02 --lib -- --nocapture tests
y23d02-bench:
    cargo bench --package y23d02
```