# Year 20 Day 02

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y20/d02",
]
```

- [ ] Add shortcuts to the justfile:

```
y20d02-solve:
    cat ./y20/d02/input.txt | cargo run --release --package y20d02
y20d02-example:
    cat ./y20/d02/example.txt | cargo run --release --package y20d02
y20d02-test:
    cargo test --package y20d02 --lib -- --nocapture tests
y20d02-bench:
    cargo bench --package y20d02
```