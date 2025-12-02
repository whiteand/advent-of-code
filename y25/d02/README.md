# Year 25 Day 02

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y25/d02",
]
```

- [ ] Add shortcuts to the justfile:

```
y25d02-solve:
    cat ./y25/d02/input.txt | cargo run --release --package y25d02
y25d02-example:
    cat ./y25/d02/example.txt | cargo run --release --package y25d02
y25d02-test:
    cargo test --package y25d02 --lib -- --nocapture tests
y25d02-bench:
    cargo bench --package y25d02
```