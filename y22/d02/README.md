# Year 22 Day 02

## TODOS

- [x] Add this crate to the global `Cargo.toml` file path.

```
members = [
    "y22/d02",
]
```

- [ ] Add shortcuts to the justfile:

```
y22d02-solve:
    cat ./y22/d02/input.txt | cargo run --release --package y22d02
y22d02-example:
    cat ./y22/d02/example.txt | cargo run --release --package y22d02
y22d02-test:
    cargo test --package y22d02 --lib -- --nocapture tests
y22d02-bench:
    cargo bench --package y22d02
```