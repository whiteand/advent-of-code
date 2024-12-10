# Year 15 Day 02

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y15/d02",
]
```

- [ ] Add shortcuts to the justfile:

```
y15d02-solve:
    cat ./y15/d02/input.txt | cargo run --release --package y15d02
y15d02-example:
    cat ./y15/d02/example.txt | cargo run --release --package y15d02
y15d02-test:
    cargo test --package y15d02 --lib -- --nocapture tests
y15d02-bench:
    cargo bench --package y15d02
```