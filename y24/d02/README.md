# Year 24 Day 02

## TODOS

- [ ] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y24/d02",
]
```

- [ ] Add shortcuts to the justfile:

```
y24d02-solve:
    cat ./y24/d02/input.txt | cargo run --release --package y24d02
y24d02-example:
    cat ./y24/d02/example.txt | cargo run --release --package y24d02
y24d02-test:
    cargo test --package y24d02 --lib -- --nocapture tests
y24d02-bench:
    cargo bench --package y24d02
```